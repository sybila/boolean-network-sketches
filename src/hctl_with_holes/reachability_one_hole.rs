use crate::hctl_with_holes::utils::{fwd_saturated, update_weight};
use biodivine_hctl_model_checker::model_checking::{
    model_check_extended_formula, model_check_extended_formula_dirty, model_check_formula,
    model_check_formula_dirty,
};
use biodivine_lib_param_bn::biodivine_std::traits::Set;
use biodivine_lib_param_bn::symbolic_async_graph::{GraphColoredVertices, SymbolicAsyncGraph};
use std::collections::HashMap;
use std::time::SystemTime;

/// Naive version of the reachability from given (fixed) initial observation to a second "uncertain" one.
/// Simply runs model checking for all variants of the second observation individually.
pub fn naive_eval(
    stg: &SymbolicAsyncGraph,
    time_0: SystemTime,
    init_observation: &GraphColoredVertices,
    second_observation_weight_pairs: Vec<(String, usize)>,
) -> Option<usize> {
    let mut best_weight = None;
    let formula_with_holes = "3{x}: @{x}: (%initial% & EF %second%)".to_string();

    for (observation_formula, weight) in second_observation_weight_pairs {
        let second_observation_set = model_check_formula_dirty(&observation_formula, stg).unwrap();
        let context = HashMap::from([
            ("initial".to_string(), init_observation.clone()),
            ("second".to_string(), second_observation_set.clone()),
        ]);

        // check if the result is trivial (i.e., some states of the second observation's set are directly
        // contained in the set of the initial observation)
        if !init_observation
            .intersect(&second_observation_set)
            .is_empty()
        {
            println!("trivial success - w: {weight}");
            best_weight = update_weight(best_weight, weight);
            continue;
        }

        let result = model_check_extended_formula(&formula_with_holes, stg, &context).unwrap();
        let time = time_0.elapsed().unwrap().as_millis();
        if !result.is_empty() {
            let cardinality = result.exact_cardinality();
            best_weight = update_weight(best_weight, weight);
            println!("success - w: {weight}, time: {time}ms, states: {cardinality}");
        } else {
            println!("---       w: {weight}, time: {time}ms");
        }
    }
    best_weight
}

/// Slightly better version of reachability from given (fixed) initial observation to a second "uncertain" one.
/// Evaluation goes in the order from the most specific (smallest) set to the most general (largest) set.
/// In each computation,instead of computing full EF again, EF result from the previous step is reused.
pub fn specific_to_general_eval(
    stg: &SymbolicAsyncGraph,
    time_0: SystemTime,
    init_observation: &GraphColoredVertices,
    second_observation_weight_pairs: Vec<(String, usize)>,
) -> Option<usize> {
    let mut best_weight = None;
    let formula_with_holes = "3{x}: @{x}: (%initial% & %ef_observation%)".to_string();

    // remember EF result from the last step (initially empty)
    let mut last_ef_result = stg.empty_colored_vertices().clone();
    for (observation_formula, weight) in second_observation_weight_pairs {
        let second_observation = model_check_formula_dirty(&observation_formula, stg).unwrap();

        // check if the result is trivial (i.e., some states of the second observation's set are directly
        // contained in the set of the initial observation)
        if !init_observation.intersect(&second_observation).is_empty() {
            best_weight = update_weight(best_weight, weight);
            println!("trivial success - w: {weight}");
            continue;
        }

        let extended_observation = second_observation.union(&last_ef_result);
        last_ef_result = model_check_extended_formula_dirty(
            "EF %extended_observation%",
            stg,
            &HashMap::from([("extended_observation".to_string(), extended_observation)]),
        )
        .unwrap();

        let context = HashMap::from([
            ("initial".to_string(), init_observation.clone()),
            ("ef_observation".to_string(), last_ef_result.clone()),
        ]);

        let result = model_check_extended_formula(&formula_with_holes, stg, &context).unwrap();
        let time = time_0.elapsed().unwrap().as_millis();
        if !result.is_empty() {
            let cardinality = result.exact_cardinality();
            best_weight = update_weight(best_weight, weight);
            println!("success - w: {weight}, time: {time}ms, states: {cardinality}");
        } else {
            let last_cardinality = last_ef_result.exact_cardinality();
            println!("---       w: {weight}, time: {time}ms, last_ef: {last_cardinality}");
        }
    }
    best_weight
}

/// Better version of reachability from given (fixed) initial observation to a second "uncertain" one.
/// Evaluation goes in the order from the most general (largest) set to the most specific (smallest) set.
/// Starts by pre-computing forward reachability (once) from the initial state, then just iteratively
/// intersects that with the second observation's sets.
pub fn general_to_specific_precomp_eval(
    stg: &SymbolicAsyncGraph,
    time_0: SystemTime,
    init_observation: &GraphColoredVertices,
    mut second_observation_weight_pairs: Vec<(String, usize)>,
) -> Option<usize> {
    let mut best_weight = None;
    // fwd reachable set from the initial observation
    let fwd_set_from_initial = fwd_saturated(stg, init_observation);

    second_observation_weight_pairs.reverse();

    for (observation_formula, weight) in second_observation_weight_pairs {
        let second_observation = model_check_formula(&observation_formula, stg).unwrap();

        // check if the result is trivial (i.e., some states of the second observation's set are directly
        // contained in the set of the initial observation)
        if !init_observation.intersect(&second_observation).is_empty() {
            best_weight = update_weight(best_weight, weight);
            println!("trivial success - w: {weight}");
            continue;
        }

        let result = fwd_set_from_initial.intersect(&second_observation);
        let time = time_0.elapsed().unwrap().as_millis();
        if !result.is_empty() {
            let cardinality = result.exact_cardinality();
            best_weight = update_weight(best_weight, weight);
            println!("success - w: {weight}, time: {time}ms, states: {cardinality}");
        } else {
            println!("---       w: {weight}, time: {time}ms");
        }
    }
    best_weight
}

/// Better version of reachability from given (fixed) initial observation to a second "uncertain" one.
/// Evaluation in the order from the most general (largest) set to the most specific (smallest) set.
/// Starts computing forward reachability from the initial state, and during this computation
/// checks if then the current reach set intersects with the second observation's sets.
///
/// This means that we dont have to compute full reachability set to know the result, if lucky.
pub fn general_to_specific_on_the_fly_eval(
    stg: &SymbolicAsyncGraph,
    time_0: SystemTime,
    init_observation: &GraphColoredVertices,
    mut second_observation_weight_pairs: Vec<(String, usize)>,
) -> Option<usize> {
    let mut best_weight = None;

    let mut done = false;
    let mut result = init_observation.clone();
    let (mut current_formula, mut current_w) = second_observation_weight_pairs.pop().unwrap();
    let mut current_second_observation = model_check_formula(&current_formula, stg).unwrap();

    // saturated reachability
    while !done {
        done = true;
        for var in stg.as_network().unwrap().variables() {
            let update = &stg.var_post(var, &result).minus(&result);
            if !update.is_empty() {
                result = result.union(update);

                // if new reach states found, check for intersection with current set for the second observation
                if !result.intersect(&current_second_observation).is_empty() {
                    let time = time_0.elapsed().unwrap().as_millis();
                    best_weight = update_weight(best_weight, current_w);

                    // check if the result is trivial (i.e., some states of the second observation's set are directly
                    // contained in the set of the initial observation) or not
                    if !init_observation
                        .intersect(&current_second_observation)
                        .is_empty()
                    {
                        println!("trivial success - w: {current_w}, time: {time}ms");
                    } else {
                        println!("success - w: {current_w}, time: {time}ms");
                    }

                    // update the current set for the second observation for the next one
                    let current_obs_pair = second_observation_weight_pairs.pop().unwrap();
                    current_formula = current_obs_pair.0;
                    current_w = current_obs_pair.1;
                    current_second_observation =
                        model_check_formula(&current_formula, stg).unwrap();
                }
                done = false;
                break;
            }
        }
    }

    // check remaining observation's sets (there might have been more than one hits at the last iteration)
    second_observation_weight_pairs.reverse();
    for (observation_formula, weight) in second_observation_weight_pairs {
        let second_observation = model_check_formula(&observation_formula, stg).unwrap();

        let inter = result.intersect(&second_observation);
        let time = time_0.elapsed().unwrap().as_millis();
        if !inter.is_empty() {
            best_weight = update_weight(best_weight, weight);
            println!("success - w: {weight}, time: {time}ms");
        } else {
            println!("---       w: {weight}, time: {time}ms");
        }
    }
    best_weight
}
