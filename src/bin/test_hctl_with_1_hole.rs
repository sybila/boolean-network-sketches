use biodivine_hctl_model_checker::mc_utils::get_extended_symbolic_graph;
use biodivine_hctl_model_checker::model_checking::{
    model_check_formula, model_check_formula_dirty,
};
use biodivine_lib_param_bn::symbolic_async_graph::SymbolicAsyncGraph;
use biodivine_lib_param_bn::BooleanNetwork;
use boolean_network_sketches::data_processing::data_encoding::encode_observation;
use boolean_network_sketches::data_processing::observations::Observation;
use boolean_network_sketches::hctl_with_holes::reachability_one_hole::{
    general_to_specific_on_the_fly_eval, general_to_specific_precomp_eval, naive_eval,
    specific_to_general_eval,
};
use boolean_network_sketches::hctl_with_holes::utils::encode_obs_weight_pairs;
#[allow(unused_imports)]
use boolean_network_sketches::hctl_with_holes::{MODEL_20V, MODEL_53V};
use std::time::SystemTime;

fn main() {
    let use_large = true;

    let mut model = MODEL_20V;
    let mut start_from_ones = false;
    if use_large {
        model = MODEL_53V;
        start_from_ones = true;
    }

    let bn = BooleanNetwork::try_from(model).unwrap();
    let props_vec: Vec<_> = bn
        .variables()
        .map(|v| bn.get_variable_name(v).clone())
        .collect();

    // for the first two methods, we must use new STG and sets with symbolic variables (for HCTL model checking)
    let stg = get_extended_symbolic_graph(&bn, 1).unwrap();

    // fixed initial observation (either ones or zeros)
    let init_observation = if start_from_ones {
        Observation::new_full_true(props_vec.len())
    } else {
        Observation::new_full_false(props_vec.len())
    };
    let init_observation_str = encode_observation(&init_observation, &props_vec).unwrap();
    let init_observation_set =
        model_check_formula_dirty(init_observation_str.as_str(), &stg).unwrap();
    // variants of the second observation and their weights
    let second_observation_weight_pairs =
        encode_obs_weight_pairs(&props_vec, !start_from_ones).unwrap();

    let start = SystemTime::now();
    println!("naive_eval");
    let best_weight = naive_eval(
        &stg,
        start,
        &init_observation_set,
        second_observation_weight_pairs.clone(),
    );
    println!("TOTAL ELAPSED: {}", start.elapsed().unwrap().as_millis());
    println!("BEST WEIGHT: {:?}", best_weight);
    print!("\n\n\n");

    let start = SystemTime::now();
    println!("specific_to_general_eval");
    let best_weight = specific_to_general_eval(
        &stg,
        start,
        &init_observation_set,
        second_observation_weight_pairs.clone(),
    );
    println!("TOTAL ELAPSED: {}", start.elapsed().unwrap().as_millis());
    println!("BEST WEIGHT: {:?}", best_weight);
    print!("\n\n\n");

    // for the following methods, we must use new STG and sets without symbolic variables
    let stg = SymbolicAsyncGraph::new(&bn).unwrap();
    let init_observation_set = model_check_formula(init_observation_str.as_str(), &stg).unwrap();

    let start = SystemTime::now();
    println!("general_to_specific_precomp_eval");
    let best_weight = general_to_specific_precomp_eval(
        &stg,
        start,
        &init_observation_set,
        second_observation_weight_pairs.clone(),
    );
    println!("TOTAL ELAPSED: {}", start.elapsed().unwrap().as_millis());
    println!("BEST WEIGHT: {:?}", best_weight);
    print!("\n\n\n");

    let start = SystemTime::now();
    println!("general_to_specific_on_the_fly_eval");
    let best_weight = general_to_specific_on_the_fly_eval(
        &stg,
        start,
        &init_observation_set,
        second_observation_weight_pairs.clone(),
    );
    println!("TOTAL ELAPSED: {}", start.elapsed().unwrap().as_millis());
    println!("BEST WEIGHT: {:?}", best_weight);
}
