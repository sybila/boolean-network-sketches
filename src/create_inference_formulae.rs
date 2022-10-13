/// Creates the formula describing the (non)existence of reachability between two states (or partial)
/// `from_state` and `to_state` are both formulae describing particular states
/// `is_universal` is true iff we want all paths from `from_state` to reach `to_state`
/// `is_negative` is true iff we want to non-existence of path from `from_state` to `to_state`
pub fn mk_reachability_pair_formula(
    from_state: String,
    to_state: String,
    is_universal: bool,
    is_negative: bool,
) -> String {
    assert!(!(is_negative && is_universal));
    assert!(!to_state.is_empty() && !from_state.is_empty());
    if is_universal {
        return format!("(3{{x}}: (@{{x}}: {} & (AF ({}))))", from_state, to_state);
    }
    if is_negative {
        return format!("(3{{x}}: (@{{x}}: {} & (~EF ({}))))", from_state, to_state);
    }
    format!("(3{{x}}: (@{{x}}: {} & (EF ({}))))", from_state, to_state)
}

/// Creates the formula describing the existence of reachability between
/// every two consecutive states from the `states`, starting with the first one
/// Basically describes s0 -> s1 -> ... -> sN
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
    formula.push_str((0..num_states+1).map(|_| ")").collect::<String>().as_str());
    formula
}


/// Creates the formula describing the existence of a particular trap space
/// trap space is a part of the state space from which we cannot escape
/// `trap_space` is a formula describing some proposition' values in a desired trap space
pub fn mk_trap_space_formula(trap_space: String) -> String {
    assert!(!trap_space.is_empty());
    format!("(3{{x}}: (@{{x}}: {} & (AG ({}))))", trap_space, trap_space)
}

/// Creates the formula describing the existence of specific attractor
/// Works only for FULLY described states (using all propositions)
/// `attractor_state` is a formula describing state in a desired attractor
pub fn mk_attractor_formula_specific(attractor_state: String) -> String {
    assert!(!attractor_state.is_empty());
    format!("(3{{x}}: (@{{x}}: ({} & (AG EF ({})))))", attractor_state, attractor_state)
}

/// Creates the formula describing the existence of attractor
/// Works for both fully or partially described states (but slower)
/// `attractor_state` is a formula describing state in a desired attractor
pub fn mk_attractor_formula_nonspecific_aeon(attractor_state: String) -> String {
    assert!(!attractor_state.is_empty());
    format!("(3{{x}}: (@{{x}}: ({} & (!{{y}}: AG EF {{y}}))))", attractor_state)
}

/// Creates the formula describing the existence of attractor
/// Works for both fully or partially described states (but slower)
/// `attractor_state` is a formula describing state in a desired attractor
pub fn mk_attractor_formula_nonspecific(attractor_state: String) -> String {
    assert!(!attractor_state.is_empty());
    format!("(3{{x}}: (@{{x}}: ({} & (AG EF ({} & {{x}})))))", attractor_state, attractor_state)
}


/// Creates the formula prohibiting all but the given attractors
/// `attractor_state_set` is a vector of formulae, each describing a state in particular
/// allowed attractor
pub fn mk_forbid_other_attractors_formula(attractor_state_set: Vec<String>) -> String {
    let mut formula = String::new();
    formula.push_str("~(3{x}: (@{x}: ~(AG EF (");
    for attractor_state in attractor_state_set {
        assert!(!attractor_state.is_empty());
        formula.push_str(format!("({}) | ", attractor_state).as_str())
    }
    formula.push_str("false ))))"); // just we dont end with "|"
    formula
}

/// Creates the formula describing the existence of specific steady-state
/// Works only for FULLY described states (using all propositions)
/// `steady_state` is a formula describing particular desired fixed point
pub fn mk_steady_state_formula_specific(steady_state: String) -> String {
    assert!(!steady_state.is_empty());
    format!("(3{{x}}: (@{{x}}: ({} & (AX ({})))))", steady_state, steady_state)
}

/// Creates the formula describing the existence of specific steady-state
/// Works for fully or partially described states (but slower)
/// `steady_state` is a formula describing particular desired fixed point
pub fn mk_steady_state_formula_nonspecific(steady_state: String) -> String {
    assert!(!steady_state.is_empty());
    format!("(3{{x}}: (@{{x}}: ({} & (AX ({} & {{x}})))))", steady_state, steady_state)
}


/// Creates the formula prohibiting all but the given steady-states
/// `steady_state_set` is a vector of formulae, each describing particular allowed fixed point
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

/// Creates the formula ensuring the existence of given fixed points and
/// prohibiting all other steady-states (uses advantage of cashing "AX x")
/// `steady_state_set` is a vector of formulae, each describing particular desired fixed point
pub fn mk_steady_state_formula_both(steady_state_set: Vec<String>) -> String {
    // part which ensures steady states
    let mut formula = String::new();
    for steady_state in steady_state_set.clone() {
        assert!(!steady_state.is_empty());
        formula.push_str(format!("(3{{x}}: (@{{x}}: {} & (AX {{x}}))) & ", steady_state).as_str())
    }

    // appendix which forbids additional steady states
    formula.push_str("~(3{x}: (@{x}: ");
    for steady_state in steady_state_set {
        formula.push_str(format!("~( {} )  & ", steady_state).as_str())
    }
    formula.push_str("(AX {x})))");
    formula
}
