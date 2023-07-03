//! Contains a struct to represent observations and their data values.

use std::fmt;

/// Enum of possible values for variable in a HCTL formula tree.
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

/// Structure for a HCTL formula syntax tree.
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
            result.and_then(|_| write!(f, "{}", value))
        })
    }
}
