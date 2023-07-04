use crate::data_processing::create_inference_formulae::*;
use crate::data_processing::observations::*;

/// Encode binarized observation with a formula depicting the corresponding state/sub-space.
/// Using binarized values and proposition names, creates a conjunction of literals
/// describing that observation.
pub fn encode_observation(observation: &Observation, prop_names: &[String]) -> String {
    let mut formula = String::new();
    formula.push('(');

    for (i, prop) in prop_names.iter().enumerate() {
        match observation.values[i] {
            VarValue::True => formula.push_str(prop),
            VarValue::False => formula.push_str(format!("~{prop}").as_str()),
            VarValue::Any => (),
        }
        formula.push_str(" & ");
    }
    formula.push_str("true )"); // to finish the conjunction without affecting evaluation
    formula
}

/// Encode several observation vectors with conjunction formulae, one by one.
fn encode_multiple_observations(
    observations: &[Observation],
    prop_names: &[String],
) -> Vec<String> {
    observations
        .iter()
        .map(|o| encode_observation(o, prop_names))
        .collect()
}

/// Encode (ordered) set of observations to a single HCTL formula. The particular formula
/// template is chosen depending on the type of data.
pub fn encode_observation_list_hctl(observation_list: ObservationList) -> String {
    let encoded_observations =
        encode_multiple_observations(&observation_list.observations, &observation_list.var_names);
    match observation_list.data_type {
        ObservationType::Attractor => mk_formula_attractor_set(encoded_observations),
        ObservationType::FixedPoint => mk_formula_fixed_point_set(encoded_observations),
        ObservationType::TimeSeries => mk_formula_reachability_chain(encoded_observations),
    }
}
