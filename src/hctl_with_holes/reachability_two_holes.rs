use crate::hctl_with_holes::utils::{fwd_saturated, update_weight};
use biodivine_hctl_model_checker::model_checking::model_check_formula;
use biodivine_lib_param_bn::biodivine_std::traits::Set;
use biodivine_lib_param_bn::symbolic_async_graph::SymbolicAsyncGraph;
use std::time::SystemTime;

/// Reachability from an "uncertain" initial observation to another "uncertain" one.
/// Currently, we consider all combinations of the following pattern:
///     1111 -> 0000, 111* -> 0000, 11** -> 0000, ..., 1111 -> 000*, 1111 -> 00**, ..., 111* -> 000*, ...
pub fn two_hole_analysis(
    stg: &SymbolicAsyncGraph,
    observation1_weight_pairs: Vec<(String, usize)>,
    observation2_weight_pairs: Vec<(String, usize)>,
    time_0: SystemTime,
) -> Option<usize> {
    let mut best_weight = None;

    // the HCTL formula for this would be like "3{x}: @{x}: (%observation1% & EF %observation2%)"

    // Start from the most specific set for the observation 1. Each subsequent set is thus a superset of the set
    // from the previous step. This enables us to reuse results of forward reachability from previous steps.
    let mut last_fwd_result = stg.empty_colored_vertices().clone();
    for (observation1_formula, weight1) in observation1_weight_pairs {
        // Compute saturated reachability from the current set for observation 1.
        let raw_observation1 = model_check_formula(&observation1_formula, stg).unwrap();
        let fwd_seed_set = raw_observation1.union(&last_fwd_result);
        let fwd_set_from_observation1 = fwd_saturated(stg, &fwd_seed_set);
        last_fwd_result = fwd_set_from_observation1.clone();

        // For all variants of sets for observation 2, just check if there is an intersection with the reachable
        // states from observation 1.
        for (observation2_formula, weight2) in &observation2_weight_pairs {
            let raw_observation2 = model_check_formula(observation2_formula, stg).unwrap();

            // check if the result is trivial (i.e., some states of the second observation's set are directly
            // contained in the set of the initial observation)
            if !raw_observation1.intersect(&raw_observation2).is_empty() {
                let weight_sum = weight1 + weight2;
                best_weight = update_weight(best_weight, weight_sum);
                println!("trivial success - w1: {weight1}, w2: {weight2}, w1+w2: {weight_sum}");
                continue;
            }

            let result = fwd_set_from_observation1.intersect(&raw_observation2);
            let time = time_0.elapsed().unwrap().as_millis();
            if !result.is_empty() {
                let cardinality = result.exact_cardinality();
                let weight_sum = weight1 + weight2;
                best_weight = update_weight(best_weight, weight_sum);
                println!("success - w1: {weight1}, w2: {weight2}, w1+w2: {weight_sum} time: {time}, states: {cardinality}");
            } else {
                println!("---       w1: {weight1}, w2: {weight2}, time: {time}");
            }
        }
    }
    best_weight
}
