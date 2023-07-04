//! Contains functionality for automatic generation of HCTL formulae expressing several
//! important properties, such as attractors or reachability.
//!
//! Many properties can be encoded in more than a one way, so we include more variants. Some of
//! them are created in a way that model-checking computation can be optimised.

/// Create a formula describing the existence of a attractor containing specific state.
///
/// Works only for FULLY described state (conjunction of literals for each proposition).
/// Param `attractor_state` is a formula describing a state in a desired attractor.
pub fn mk_formula_attractor_specific(attractor_state: String) -> String {
    assert!(!attractor_state.is_empty());
    format!("(3{{x}}: (@{{x}}: ({attractor_state} & (AG EF ({attractor_state})))))")
}

/// Create a formula describing the existence of a attractor containing partially specified state.
/// Works for both fully or partially described states (but for fully specified states, we
/// recommend using `mk_attractor_formula_specific`).
///
/// Formula is created in a way that the model-checker can use AEON algorithms to optimise its
/// computation.
///
/// Param `attractor_state` is a formula describing a (partial) state in a desired attractor.
pub fn mk_formula_attractor_aeon(attractor_state: String) -> String {
    assert!(!attractor_state.is_empty());
    format!("(3{{x}}: (@{{x}}: ({attractor_state} & (!{{y}}: AG EF {{y}}))))")
}

/// Create a formula describing the existence of a attractor containing partially specified state.
///
/// Works for both fully or partially described states (but for fully specified states, we
/// recommend using `mk_attractor_formula_specific`).
///
/// Param `attractor_state` is a formula describing a (partial) state in a desired attractor.
pub fn mk_formula_attractor(attractor_state: String) -> String {
    assert!(!attractor_state.is_empty());
    format!("(3{{x}}: (@{{x}}: ({attractor_state} & (AG EF ({attractor_state} & {{x}})))))")
}

/// Create a formula ensuring the existence of a set of attractor states.
pub fn mk_formula_attractor_set(attractor_state_set: Vec<String>) -> String {
    let mut formula = String::new();
    for attractor_state in attractor_state_set {
        formula.push_str(mk_formula_attractor(attractor_state).as_str());
        formula.push_str(" & ");
    }
    formula.push_str("true"); // append "true" at the end to ensure that formula is valid
    formula
}

/// Create a formula prohibiting all attractors that do not contain specified states.
///
/// Param `attractor_state_set` is a vector of formulae, each describing a state in particular
/// allowed attractor (conjunction of literals).
pub fn mk_formula_forbid_other_attractors(attractor_state_set: Vec<String>) -> String {
    let mut formula = String::new();
    formula.push_str("~(3{x}: (@{x}: ~(AG EF (");
    for attractor_state in attractor_state_set {
        assert!(!attractor_state.is_empty());
        formula.push_str(format!("({attractor_state}) | ").as_str())
    }
    formula.push_str("false ))))"); // just so that we dont end with "|"
    formula
}

/// Create a formula ensuring the existence of a set of attractor states and prohibiting any
/// other attractors not containing these states.
pub fn mk_formula_attractors_combined(attractor_state_set: Vec<String>) -> String {
    // part which ensures attractor states
    let mut formula = String::new();
    for attractor_state in attractor_state_set.clone() {
        formula.push_str(mk_formula_attractor(attractor_state).as_str());
        formula.push_str(" & ");
    }

    // append the sub-formula which forbids additional attractor states
    formula.push_str(mk_formula_forbid_other_attractors(attractor_state_set).as_str());
    formula
}

/// Create a formula describing the existence of a specific steady-state.
///
/// Works only for FULLY described states (conjunction with a literal for each proposition).
/// Param `steady_state` is a formula describing that particular state.
pub fn mk_formula_fixed_point_specific(steady_state: String) -> String {
    assert!(!steady_state.is_empty());
    format!("(3{{x}}: (@{{x}}: ({steady_state} & (AX ({steady_state})))))")
}

/// Create a formula describing the existence of a (partially specified) steady-state.
///
/// Works for both fully or partially specified described states.
/// Param `steady_state` is a formula describing that particular state.
pub fn mk_formula_fixed_point(steady_state: String) -> String {
    assert!(!steady_state.is_empty());
    format!("(3{{x}}: (@{{x}}: ({steady_state} & (AX ({steady_state} & {{x}})))))")
}

/// Create a formula ensuring the existence of a set of fixed points.
pub fn mk_formula_fixed_point_set(steady_state_set: Vec<String>) -> String {
    let mut formula = String::new();
    for steady_state in steady_state_set {
        formula.push_str(mk_formula_fixed_point(steady_state).as_str());
        formula.push_str(" & ");
    }
    formula.push_str("true"); // append "true" at the end to ensure that formula is valid
    formula
}

/// Create a formula prohibiting all but the given states to be fixed-points.
///
/// Param `steady_state_set` is a vector of formulae, each describing particular allowed state.
pub fn mk_formula_forbid_other_fixed_points(steady_state_set: Vec<String>) -> String {
    let mut formula = String::new();
    formula.push_str("~(3{x}: (@{x}: ");
    for steady_state in steady_state_set {
        assert!(!steady_state.is_empty());
        formula.push_str(format!("~({steady_state}) & ").as_str())
    }
    formula.push_str("(AX {x})))");
    formula
}

/// Create a formula ensuring the existence of a set of fixed points and prohibiting all other
/// states to be fixed-points.
///
/// This formula is build in a way that uses advantage of model-checkers cashing (for "AX x").
/// Param `steady_state_set` is a vector of formulae, each describing one state.
pub fn mk_formula_fixed_points_combined(steady_state_set: Vec<String>) -> String {
    // part which ensures steady states
    let mut formula = String::new();
    for steady_state in steady_state_set.clone() {
        formula.push_str(mk_formula_fixed_point(steady_state).as_str());
        formula.push_str(" & ");
    }

    // append the sub-formula which forbids additional steady states
    formula.push_str(mk_formula_forbid_other_fixed_points(steady_state_set).as_str());
    formula
}

/// Create a formula describing the (non)existence of reachability between two (partial) states.
///
/// `from_state` and `to_state` are both formulae describing particular states.
/// `is_negative` is true iff we want to non-existence of path from `from_state` to `to_state`
pub fn mk_formula_reachability_pair(
    from_state: String,
    to_state: String,
    is_negative: bool,
) -> String {
    assert!(!to_state.is_empty() && !from_state.is_empty());
    if is_negative {
        return format!("(3{{x}}: (@{{x}}: {from_state} & (~EF ({to_state}))))");
    }
    format!("(3{{x}}: (@{{x}}: {from_state} & (EF ({to_state}))))")
}

/// Create a formula describing the existence of reachability between every two consecutive states
/// from the `states_sequence`, starting with the first one.
///
/// Basically can be used to describe a time series s0 -> s1 -> ... -> sN
pub fn mk_formula_reachability_chain(states_sequence: Vec<String>) -> String {
    let mut formula = String::new();
    formula.push_str("(3{x}: (@{x}: ");
    let num_states = states_sequence.len();
    for (n, state) in states_sequence.iter().enumerate() {
        assert!(!state.is_empty());
        if n == num_states - 1 {
            break;
        }
        formula.push_str(format!("({state}) & EF (").as_str())
    }

    // add the last state and all the closing parentheses
    formula.push_str(states_sequence[num_states - 1].to_string().as_str());
    formula.push_str(
        (0..num_states + 1)
            .map(|_| ")")
            .collect::<String>()
            .as_str(),
    );
    formula
}

#[cfg(test)]
mod tests {
    use crate::data_processing::create_inference_formulae::*;

    #[test]
    /// Test generating of different kinds of general attractor formulae.
    fn test_attractor_encodings() {
        let attr_states = vec!["a & b & ~c".to_string(), "a & b & c".to_string()];

        assert_eq!(
            mk_formula_attractor_specific(attr_states[0].clone()),
            "(3{x}: (@{x}: (a & b & ~c & (AG EF (a & b & ~c)))))".to_string(),
        );
        assert_eq!(
            mk_formula_attractor_aeon(attr_states[0].clone()),
            "(3{x}: (@{x}: (a & b & ~c & (!{y}: AG EF {y}))))".to_string(),
        );
        assert_eq!(
            mk_formula_attractor(attr_states[0].clone()),
            "(3{x}: (@{x}: (a & b & ~c & (AG EF (a & b & ~c & {x})))))".to_string(),
        );
        assert_eq!(
            mk_formula_forbid_other_attractors(attr_states.clone()),
            "~(3{x}: (@{x}: ~(AG EF ((a & b & ~c) | (a & b & c) | false ))))".to_string(),
        );
    }

    #[test]
    /// Test generating of different kinds of steady state formulae.
    fn test_steady_state_encodings() {
        let attr_states = vec!["a & b & ~c".to_string(), "a & b & c".to_string()];

        assert_eq!(
            mk_formula_fixed_point_specific(attr_states[0].clone()),
            "(3{x}: (@{x}: (a & b & ~c & (AX (a & b & ~c)))))".to_string(),
        );
        assert_eq!(
            mk_formula_fixed_point(attr_states[0].clone()),
            "(3{x}: (@{x}: (a & b & ~c & (AX (a & b & ~c & {x})))))".to_string(),
        );
        assert_eq!(
            mk_formula_forbid_other_fixed_points(attr_states.clone()),
            "~(3{x}: (@{x}: ~(a & b & ~c) & ~(a & b & c) & (AX {x})))".to_string(),
        );
        assert_eq!(
            mk_formula_fixed_points_combined(attr_states.clone()),
            "(3{x}: (@{x}: (a & b & ~c & (AX (a & b & ~c & {x}))))) & (3{x}: (@{x}: (a & b & c & (AX (a & b & c & {x}))))) & ~(3{x}: (@{x}: ~(a & b & ~c) & ~(a & b & c) & (AX {x})))".to_string(),
        );
    }

    #[test]
    /// Test generating reachability formulae.
    fn test_reachability_encoding() {
        let states = vec![
            "a & b & ~c".to_string(),
            "a & b & c".to_string(),
            "~a & b & c".to_string(),
        ];

        assert_eq!(
            mk_formula_reachability_pair(states[0].clone(), states[1].clone(), true),
            "(3{x}: (@{x}: a & b & ~c & (~EF (a & b & c))))".to_string(),
        );
        assert_eq!(
            mk_formula_reachability_pair(states[0].clone(), states[1].clone(), false),
            "(3{x}: (@{x}: a & b & ~c & (EF (a & b & c))))".to_string(),
        );
        assert_eq!(
            mk_formula_reachability_chain(states),
            "(3{x}: (@{x}: (a & b & ~c) & EF ((a & b & c) & EF (~a & b & c))))".to_string(),
        );
    }
}
