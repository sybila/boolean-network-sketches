//! Structs to represent observations and their data values.

use std::fmt;

/// Enum of possible values for variables in each observation (binary + unknown).
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum VarValue {
    True,
    False,
    Any,
}

impl fmt::Display for VarValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VarValue::True => write!(f, "1"),
            VarValue::False => write!(f, "0"),
            VarValue::Any => write!(f, "-"),
        }
    }
}

/// Structure to represent single observation (vector of binarized values).
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Observation {
    pub values: Vec<VarValue>,
}

impl Observation {
    /// Create `Observation` object from a vector with values.
    pub fn new(values: Vec<VarValue>) -> Self {
        Self { values }
    }

    /// Create `Observation` encoding a vector of `n` True values.
    pub fn new_full_true(n: usize) -> Self {
        Self {
            values: vec![VarValue::True; n],
        }
    }

    /// Create `Observation` encoding a vector of `n` False values.
    pub fn new_full_false(n: usize) -> Self {
        Self {
            values: vec![VarValue::False; n],
        }
    }

    /// Create `Observation` object from string encoding its values.
    pub fn try_from_str(observation_string: String) -> Result<Self, String> {
        let mut observation_vec: Vec<VarValue> = Vec::new();
        for c in observation_string.chars() {
            match c {
                '1' => observation_vec.push(VarValue::True),
                '0' => observation_vec.push(VarValue::False),
                '-' => observation_vec.push(VarValue::Any),
                _ => return Err(format!("Unexpected char '{c}' in an observation.")),
            }
        }
        if observation_vec.is_empty() {
            return Err("Observation can't be empty.".to_string());
        }

        Ok(Self {
            values: observation_vec,
        })
    }

    pub fn num_values(&self) -> usize {
        self.values.len()
    }

    pub fn num_unspecified_values(&self) -> usize {
        self.values.iter().filter(|&v| *v == VarValue::Any).count()
    }

    pub fn num_specified_values(&self) -> usize {
        self.values.iter().filter(|&v| *v != VarValue::Any).count()
    }

    pub fn num_ones(&self) -> usize {
        self.values.iter().filter(|&v| *v == VarValue::True).count()
    }

    pub fn num_zeros(&self) -> usize {
        self.values
            .iter()
            .filter(|&v| *v == VarValue::False)
            .count()
    }
}

impl fmt::Display for Observation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.values
            .iter()
            .try_for_each(|value| write!(f, "{value}"))
    }
}

/// Enum of possible types of observations (depends on how were the data measured).
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ObservationType {
    Attractor,
    FixedPoint,
    TimeSeries,
    Unspecified,
}

impl fmt::Display for ObservationType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ObservationType::Attractor => write!(f, "attractor"),
            ObservationType::FixedPoint => write!(f, "fixed-point"),
            ObservationType::TimeSeries => write!(f, "time-series"),
            ObservationType::Unspecified => write!(f, "unspecified"),
        }
    }
}

/// Structure to store an ordered list of observations (ordered in order to be able to
/// model reachability, etc.).
/// Contains binarized observations' data, names for variables, and type of data.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ObservationList {
    pub observations: Vec<Observation>,
    pub var_names: Vec<String>,
    pub data_type: ObservationType,
}

impl ObservationList {
    /// The provided data must be checked beforehand.
    pub fn new(
        observations: Vec<Observation>,
        var_names: Vec<String>,
        data_type: ObservationType,
    ) -> Self {
        Self {
            observations,
            var_names,
            data_type,
        }
    }
}

impl fmt::Display for ObservationList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut format_string = format!(
            "{} {} observations with vars [",
            self.observations.len(),
            self.data_type,
        );
        for variable in &self.var_names {
            format_string.push_str(format!("{variable}, ").as_str());
        }
        format_string = format_string.strip_suffix(", ").unwrap().to_string();
        format_string.push_str("]: \n");
        for observation in &self.observations {
            format_string.push_str(format!("> {observation}\n").as_str());
        }

        write!(f, "{format_string}")
    }
}

#[cfg(test)]
mod tests {
    use crate::data_processing::observations::{
        Observation, ObservationList, ObservationType, VarValue,
    };

    #[test]
    /// Test creating observation object from string.
    fn test_observation_from_str() {
        let observation_str = "001--".to_string();
        let observation = Observation {
            values: vec![
                VarValue::False,
                VarValue::False,
                VarValue::True,
                VarValue::Any,
                VarValue::Any,
            ],
        };
        assert_eq!(
            Observation::try_from_str(observation_str).unwrap(),
            observation
        );
    }

    #[test]
    /// Test error handling while creating observation object from string.
    fn test_err_observation_from_str() {
        let observation_str1 = "0 1--".to_string();
        let observation_str2 = "0--a".to_string();
        let observation_str3 = "".to_string();

        assert!(Observation::try_from_str(observation_str1).is_err());
        assert!(Observation::try_from_str(observation_str2).is_err());
        assert!(Observation::try_from_str(observation_str3).is_err());
    }

    #[test]
    /// Test displaying of observations.
    fn test_display_observations() {
        let observation_str = "001--".to_string();
        let observation = Observation {
            values: vec![
                VarValue::False,
                VarValue::False,
                VarValue::True,
                VarValue::Any,
                VarValue::Any,
            ],
        };
        assert_eq!(observation.to_string(), observation_str);
    }

    #[test]
    /// Test displaying of observation lists.
    fn test_display_observation_list() {
        let observation1 = Observation::try_from_str("-1".to_string()).unwrap();
        let observation2 = Observation::try_from_str("00".to_string()).unwrap();
        let observation_list = ObservationList {
            observations: vec![observation1, observation2],
            var_names: vec!["a".to_string(), "b".to_string()],
            data_type: ObservationType::Attractor,
        };

        let mut observation_list_str = "2 attractor observations with vars [a, b]: \n".to_string();
        observation_list_str.push_str("> -1\n> 00\n");
        assert_eq!(observation_list.to_string(), observation_list_str);
    }
}
