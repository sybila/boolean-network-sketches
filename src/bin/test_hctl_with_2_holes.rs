use biodivine_lib_param_bn::symbolic_async_graph::SymbolicAsyncGraph;
use biodivine_lib_param_bn::BooleanNetwork;
use boolean_network_sketches::hctl_with_holes::reachability_two_holes::two_hole_analysis;
use boolean_network_sketches::hctl_with_holes::utils::encode_obs_weight_pairs;
#[allow(unused_imports)]
use boolean_network_sketches::hctl_with_holes::{MODEL_20V, MODEL_53V};
use std::time::SystemTime;

fn main() {
    let use_large = false;

    let mut model = MODEL_20V;
    let mut start_from_ones = true;
    if use_large {
        model = MODEL_53V;
        start_from_ones = false;
    }

    let bn = BooleanNetwork::try_from(model).unwrap();
    let props_vec: Vec<_> = bn
        .variables()
        .map(|v| bn.get_variable_name(v).clone())
        .collect();
    let stg = SymbolicAsyncGraph::new(&bn).unwrap();

    let start_time = SystemTime::now();

    // variants of each observation and their weights (one being ones, the other zeros)
    let observation1_weight_pairs = encode_obs_weight_pairs(&props_vec, start_from_ones).unwrap();
    let observation2_weight_pairs = encode_obs_weight_pairs(&props_vec, !start_from_ones).unwrap();

    let best_weight = two_hole_analysis(
        &stg,
        observation1_weight_pairs,
        observation2_weight_pairs,
        start_time,
    );
    let time = start_time.elapsed().unwrap().as_millis();
    println!("BEST WEIGHT: {:?}", best_weight);
    println!("TOTAL ELAPSED: {time}\n\n");
}
