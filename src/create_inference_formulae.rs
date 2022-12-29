//! Contains functionality for automatic generation of HCTL formulae expressing several
//! important properties, such as attractors or reachability.
//!
//! Many properties can be encoded in more than a one way, so we include more variants. Some of
//! them are created in a way that model-checking computation can be optimised.

/// Encode boolean vector as a (partial) state observation.
/// Using 0/1 encoding of an observation and proposition names, creates a conjunction of literals
/// describing the observation.
pub fn encode_binary_vector(values: Vec<bool>, prop_names: Vec<&str>) -> String {
    let mut formula = String::new();
    formula.push('(');

    for (i, prop) in prop_names.iter().enumerate() {
        if !values[i] {
            formula.push('~');
        }
        formula.push_str(prop);
        if i != values.len() - 1 {
            formula.push_str(" & ");
        }
    }

    formula.push(')');
    formula
}

/// Create a formula describing the existence of a attractor containing specific state.
///
/// Works only for FULLY described state (conjunction of literals for each proposition).
///
/// Param `attractor_state` is a formula describing a state in a desired attractor.
pub fn mk_attractor_formula_specific(attractor_state: String) -> String {
    assert!(!attractor_state.is_empty());
    format!(
        "(3{{x}}: (@{{x}}: ({} & (AG EF ({})))))",
        attractor_state, attractor_state
    )
}

/// Create a formula describing the existence of a attractor containing partially specified state.
/// Works for both fully or partially described states (but for fully specified states, we
/// recommend using `mk_attractor_formula_specific`).
///
/// Formula is created in a way that the model-checker can use AEON algorithms to optimise its
/// computation.
///
/// Param `attractor_state` is a formula describing a (partial) state in a desired attractor.
pub fn mk_attractor_formula_nonspecific_aeon(attractor_state: String) -> String {
    assert!(!attractor_state.is_empty());
    format!(
        "(3{{x}}: (@{{x}}: ({} & (!{{y}}: AG EF {{y}}))))",
        attractor_state
    )
}

/// Create a formula describing the existence of a attractor containing partially specified state.
///
/// Works for both fully or partially described states (but for fully specified states, we
/// recommend using `mk_attractor_formula_specific`).
///
/// Param `attractor_state` is a formula describing a (partial) state in a desired attractor.
pub fn mk_attractor_formula_nonspecific(attractor_state: String) -> String {
    assert!(!attractor_state.is_empty());
    format!(
        "(3{{x}}: (@{{x}}: ({} & (AG EF ({} & {{x}})))))",
        attractor_state, attractor_state
    )
}

/// Create a formula prohibiting all attractors that do not contain specified states.
///
/// Param `attractor_state_set` is a vector of formulae, each describing a state in particular
/// allowed attractor (conjunction of literals).
pub fn mk_forbid_other_attractors_formula(attractor_state_set: Vec<String>) -> String {
    let mut formula = String::new();
    formula.push_str("~(3{x}: (@{x}: ~(AG EF (");
    for attractor_state in attractor_state_set {
        assert!(!attractor_state.is_empty());
        formula.push_str(format!("({}) | ", attractor_state).as_str())
    }
    formula.push_str("false ))))"); // just so that we dont end with "|"
    formula
}

/// Create a formula describing the existence of a specific steady-state.
///
/// Works only for FULLY described states (conjunction with a literal for each proposition).
/// Param `steady_state` is a formula describing that particular state.
pub fn mk_steady_state_formula_specific(steady_state: String) -> String {
    assert!(!steady_state.is_empty());
    format!(
        "(3{{x}}: (@{{x}}: ({} & (AX ({})))))",
        steady_state, steady_state
    )
}

/// Create a formula describing the existence of a (partially specified) steady-state.
///
/// Works for both fully or partially specified described states.
/// Param `steady_state` is a formula describing that particular state.
pub fn mk_steady_state_formula_nonspecific(steady_state: String) -> String {
    assert!(!steady_state.is_empty());
    format!(
        "(3{{x}}: (@{{x}}: ({} & (AX ({} & {{x}})))))",
        steady_state, steady_state
    )
}

/// Create a formula prohibiting all but the given states to be fixed-points.
///
/// Param `steady_state_set` is a vector of formulae, each describing particular allowed state.
pub fn mk_forbid_other_steady_states_formula(steady_state_set: Vec<String>) -> String {
    let mut formula = String::new();
    formula.push_str("~(3{x}: (@{x}: ");
    for steady_state in steady_state_set {
        assert!(!steady_state.is_empty());
        formula.push_str(format!("~({}) & ", steady_state).as_str())
    }
    formula.push_str("(AX {x})))");
    formula
}

/// Create a formula ensuring the existence of a set of fixed points and prohibiting all other
/// states to be fixed-points
///
/// This formula is build in a way that uses advantage of model-checkers cashing (for "AX x").
/// Param `steady_state_set` is a vector of formulae, each describing one state.
pub fn mk_steady_state_formula_combined(steady_state_set: Vec<String>) -> String {
    // part which ensures steady states
    let mut formula = String::new();
    for steady_state in steady_state_set.clone() {
        assert!(!steady_state.is_empty());
        formula.push_str(format!("(3{{x}}: (@{{x}}: {} & (AX {{x}}))) & ", steady_state).as_str())
    }

    // append the sub-formula which forbids additional steady states
    formula.push_str("~(3{x}: (@{x}: ");
    for steady_state in steady_state_set {
        formula.push_str(format!("~( {} )  & ", steady_state).as_str())
    }
    formula.push_str("(AX {x})))");
    formula
}

/// Create a formula describing the existence of a particular trap space (trap space is a part of
/// the state space from which we cannot escape).
///
/// Param `trap_space` is a formula describing desired values of (some) propositions in a trap
/// space.
pub fn mk_trap_space_formula(trap_space: String) -> String {
    assert!(!trap_space.is_empty());
    format!("(3{{x}}: (@{{x}}: {} & (AG ({}))))", trap_space, trap_space)
}

/// Create a formula describing the (non)existence of reachability between two (partial) states.
///
/// `from_state` and `to_state` are both formulae describing particular states.
/// `is_universal` is true iff we want all paths from `from_state` to reach `to_state`.
/// `is_negative` is true iff we want to non-existence of path from `from_state` to `to_state`
pub fn mk_reachability_pair_formula(
    from_state: String,
    to_state: String,
    is_universal: bool,
    is_negative: bool,
) -> String {
    assert!(!(is_negative && is_universal));
    assert!(!to_state.is_empty() && !from_state.is_empty());

    // TODO check the definition and semantics of "universal reachability"
    if is_universal {
        return format!("(3{{x}}: (@{{x}}: {} & (AF ({}))))", from_state, to_state);
    }
    if is_negative {
        return format!("(3{{x}}: (@{{x}}: {} & (~EF ({}))))", from_state, to_state);
    }
    format!("(3{{x}}: (@{{x}}: {} & (EF ({}))))", from_state, to_state)
}

/// Create a formula describing the existence of reachability between every two consecutive states
/// from the `states_sequence`, starting with the first one.
///
/// Basically can be used to describe a time series s0 -> s1 -> ... -> sN
pub fn mk_reachability_chain_formula(states_sequence: Vec<String>) -> String {
    let mut formula = String::new();
    formula.push_str("(3{x}: (@{x}: ");
    let num_states = states_sequence.len();
    for n in 0..num_states {
        assert!(!states_sequence[n].is_empty());
        if n == num_states - 1 {
            break;
        }
        formula.push_str(format!("({}) & EF (", states_sequence[n]).as_str())
    }

    // add the last state and all the closing parentheses
    formula.push_str(format!("{}", states_sequence[num_states - 1]).as_str());
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
    use crate::create_inference_formulae::encode_binary_vector;

    #[test]
    /// Test encoding of Boolean vector to formula.
    fn test_observation_encoding() {
        let values = vec![false, true, false, true];
        let prop_names = vec!["A", "B", "C", "D"];

        assert_eq!(
            encode_binary_vector(values, prop_names),
            "(~A & B & ~C & D)".to_string()
        );
    }
}
