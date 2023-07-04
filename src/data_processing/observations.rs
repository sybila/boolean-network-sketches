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
}

impl fmt::Display for ObservationType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ObservationType::Attractor => write!(f, "attractor"),
            ObservationType::FixedPoint => write!(f, "fixed-point"),
            ObservationType::TimeSeries => write!(f, "time-series"),
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
