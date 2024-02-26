use crate::data_processing::data_encoding::encode_observation;
use crate::data_processing::observations::Observation;
use biodivine_lib_param_bn::biodivine_std::traits::Set;
use biodivine_lib_param_bn::symbolic_async_graph::{GraphColoredVertices, SymbolicAsyncGraph};

/// Compute forward reachable states with saturation-based algorithm.
pub fn fwd_saturated(
    graph: &SymbolicAsyncGraph,
    initial: &GraphColoredVertices,
) -> GraphColoredVertices {
    let mut result = initial.clone();
    let mut done = false;
    while !done {
        done = true;
        for var in graph.as_network().unwrap().variables() {
            let update = &graph.var_post(var, &result).minus(&result);
            if !update.is_empty() {
                result = result.union(update);
                done = false;
                break;
            }
        }
    }
    result
}

/// Generate list of formulae encoding a sequence of partial observations starting at a full state with
/// all propositions being true (or false), and each successive observation loosens up one variable.
/// For example, for 4 propositions, the sequence is following:
///     1111, 111*, 11**, 1***, ****
/// Each observation is given a weight equal to the number of free propositions (full state has weight 0).
pub fn encode_obs_weight_pairs(
    prop_names: &[String],
    use_ones: bool,
) -> Result<Vec<(String, usize)>, String> {
    let n = prop_names.len();
    let mut observation_weight_pairs = Vec::new();
    for i in 0..n + 1 {
        let mut obs_str = String::new();
        for _ in 0..n - i {
            obs_str.push(if use_ones { '1' } else { '0' });
        }
        for _ in n - i..n {
            obs_str.push('-');
        }
        let observation = Observation::try_from_str(obs_str)?;
        let observation_formula = encode_observation(&observation, prop_names)?;
        observation_weight_pairs.push((observation_formula, i));
    }
    Ok(observation_weight_pairs)
}

/// Update the (potentially None-valued) weight to a new value, if it is better (smaller).
/// If the other weight is the same or worse, the original value is returned.
pub fn update_weight(original: Option<usize>, new: usize) -> Option<usize> {
    match original {
        Some(val) if { val < new } => original,
        _ => Some(new),
    }
}
