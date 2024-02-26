use crate::data_processing::create_inference_formulae::*;
use crate::data_processing::observations::*;

/// Encode binarized observation with a formula depicting the corresponding state/sub-space.
/// Using binarized values and proposition names, creates a conjunction of literals
/// describing that observation.
pub fn encode_observation(
    observation: &Observation,
    prop_names: &[String],
) -> Result<String, String> {
    if observation.num_values() != prop_names.len() {
        return Err("Numbers of observation's values and propositions differs.".to_string());
    }
    let mut formula = String::new();
    formula.push('(');

    for (i, prop) in prop_names.iter().enumerate() {
        match observation.values[i] {
            VarValue::True => formula.push_str(format!("{prop} & ").as_str()),
            VarValue::False => formula.push_str(format!("~{prop} & ").as_str()),
            VarValue::Any => (),
        }
    }

    // formula might be 'empty' if all props can have arbitrary values - corresponding to 'true'
    if formula.len() == 1 {
        formula.push_str("true");
    } else {
        formula = formula.strip_suffix(" & ").unwrap().to_string();
    }
    formula.push(')');
    Ok(formula)
}

/// Encode several observation vectors with conjunction formulae, one by one.
fn encode_multiple_observations(
    observations: &[Observation],
    prop_names: &[String],
) -> Result<Vec<String>, String> {
    observations
        .iter()
        .map(|o| encode_observation(o, prop_names))
        .collect::<Result<Vec<String>, String>>()
}

/// Encode (ordered) set of observations to a single HCTL formula. The particular formula
/// template is chosen depending on the type of data.
pub fn encode_observation_list_hctl(observation_list: ObservationList) -> Result<String, String> {
    let encoded_observations =
        encode_multiple_observations(&observation_list.observations, &observation_list.var_names)?;
    match observation_list.data_type {
        ObservationType::Attractor => Ok(mk_formula_attractor_set(encoded_observations)),
        ObservationType::FixedPoint => Ok(mk_formula_fixed_point_set(encoded_observations)),
        ObservationType::TimeSeries => Ok(mk_formula_reachability_chain(encoded_observations)),
        ObservationType::Unspecified => Err("Cannot encode data with unspecified type".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use crate::data_processing::data_encoding::{
        encode_multiple_observations, encode_observation, encode_observation_list_hctl,
    };
    use crate::data_processing::observations::{Observation, ObservationList, ObservationType};

    #[test]
    /// Test encoding of an observation.
    fn test_observation_encoding() {
        let prop_names = vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
            "e".to_string(),
        ];

        let observation1 = Observation::try_from_str("001-1".to_string()).unwrap();
        let encoded1 = "(~a & ~b & c & e)";
        assert_eq!(
            encode_observation(&observation1, &prop_names).unwrap(),
            encoded1
        );

        let observation2 = Observation::try_from_str("001--".to_string()).unwrap();
        let encoded2 = "(~a & ~b & c)";
        assert_eq!(
            encode_observation(&observation2, &prop_names).unwrap(),
            encoded2
        );

        let observation3 = Observation::try_from_str("-----".to_string()).unwrap();
        let encoded3 = "(true)";
        assert_eq!(
            encode_observation(&observation3, &prop_names).unwrap(),
            encoded3
        );

        assert_eq!(
            encode_multiple_observations(
                &vec![observation1, observation2, observation3],
                &prop_names
            )
            .unwrap(),
            vec![encoded1, encoded2, encoded3]
        );
    }

    #[test]
    /// Test encoding of a list of observations of various kinds.
    fn test_attractor_observations_encoding() {
        let observation1 = Observation::try_from_str("110".to_string()).unwrap();
        let observation2 = Observation::try_from_str("1-1".to_string()).unwrap();
        let raw_observations = vec![observation1, observation2];
        let prop_names = vec!["a".to_string(), "b".to_string(), "c".to_string()];

        let attr_observations = ObservationList::new(
            raw_observations.clone(),
            prop_names.clone(),
            ObservationType::Attractor,
        );
        assert_eq!(
            encode_observation_list_hctl(attr_observations).unwrap(),
            "((3{x}: (@{x}: ((a & b & ~c) & (AG EF ((a & b & ~c) & {x}))))) & (3{x}: (@{x}: ((a & c) & (AG EF ((a & c) & {x}))))))".to_string(),
        );

        let fixed_point_observations = ObservationList::new(
            raw_observations.clone(),
            prop_names.clone(),
            ObservationType::FixedPoint,
        );
        assert_eq!(
            encode_observation_list_hctl(fixed_point_observations).unwrap(),
            "((3{x}: (@{x}: ((a & b & ~c) & (AX ((a & b & ~c) & {x}))))) & (3{x}: (@{x}: ((a & c) & (AX ((a & c) & {x}))))))".to_string(),
        );

        let time_series_observations = ObservationList::new(
            raw_observations.clone(),
            prop_names.clone(),
            ObservationType::TimeSeries,
        );
        assert_eq!(
            encode_observation_list_hctl(time_series_observations).unwrap(),
            "(3{x}: (@{x}: ((a & b & ~c)) & EF ((a & c))))".to_string(),
        );

        let unspecified_observations = ObservationList::new(
            raw_observations.clone(),
            prop_names.clone(),
            ObservationType::Unspecified,
        );
        assert!(encode_observation_list_hctl(unspecified_observations).is_err());
    }
}
