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
    /// Create observation object from string encoding its values.
    pub fn new(values: Vec<VarValue>) -> Self {
        Self { values }
    }

    /// Create observation object from string encoding its values.
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
}

impl fmt::Display for Observation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.values.iter().fold(Ok(()), |result, value| {
            result.and_then(|_| write!(f, "{value}"))
        })
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
        let init_string = format!(
            "{} {} observations with vars {:?}: ",
            self.observations.len(),
            self.data_type,
            self.var_names
        );
        self.observations
            .iter()
            .fold(write!(f, "{init_string}"), |result, observation| {
                result.and_then(|_| write!(f, "> {observation}"))
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::data_processing::observations::{Observation, VarValue};

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
}
