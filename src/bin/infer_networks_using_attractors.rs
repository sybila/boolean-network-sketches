use clap::Parser;

use biodivine_lib_param_bn::symbolic_async_graph::SymbolicAsyncGraph;
use biodivine_lib_param_bn::BooleanNetwork;

use network_sketches::inference_attractor_data::*;
use network_sketches::utils::check_if_result_contains_goal;

use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::SystemTime;

/// Structure to collect CLI arguments
#[derive(Parser)]
#[clap(
    author="Ondrej Huvar",
    about="Inference of BNs from partially defined model and attractors."
)]
struct Arguments {
    /// Path to the file with BN model in aeon format
    model_path: String,

    /// Path to the file with attractor data
    attractor_data_path: String,

    /// Prohibit all attractors not containing specified states
    #[clap(short, long, takes_value = false)]
    prohibit_extra_attrs: bool,

    /// Compute with fixed-point attractors only
    #[clap(short, long, takes_value = false)]
    fixed_points: bool,

    /// Goal model to check for in the resulting ensemble
    #[clap(short, long)]
    goal_model: Option<String>,
}

/// Infer Boolean network using network sketches with attractor data.
/// Sketch is given by 4 components: influence graph, PSBN, static and dynamic properties.
/// The first three are given using aeon model format.
/// Only dynamic properties allowed are attractor data.
fn main() {
    let start = SystemTime::now();

    let args = Arguments::parse();
    let goal_aeon_string: Option<String> = match args.goal_model {
        None => None,
        Some(file_name) => Some(read_to_string(file_name.to_string()).unwrap()),
    };
    println!(
        "MODE: fixed point attrs only: {}; other attrs allowed: {}; goal model supplied: {}",
        args.fixed_points,
        !args.prohibit_extra_attrs,
        goal_aeon_string.is_some(),
    );

    let data_file = File::open(Path::new(args.attractor_data_path.as_str())).unwrap();
    let reader = BufReader::new(&data_file);
    let data: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    let aeon_string = read_to_string(args.model_path).unwrap();

    let bn = BooleanNetwork::try_from(aeon_string.as_str()).unwrap();
    println!("Loaded model with {} vars.", bn.num_vars());

    // Create graph object with 1 HCTL var (we dont need more)
    let graph = SymbolicAsyncGraph::new(bn, 1).unwrap();
    println!(
        "Model has {} parameters.",
        graph.symbolic_context().num_parameter_vars()
    );
    println!("----------");

    let inferred_colors = perform_inference_with_attractors_specific(
        data,
        graph.clone(),
        args.fixed_points,
        args.prohibit_extra_attrs,
    );

    println!(
        "{} suitable networks found in total",
        inferred_colors.approx_cardinality()
    );

    println!("----------");

    // check whether goal network (if supplied) is part of the solution set
    check_if_result_contains_goal(graph, goal_aeon_string, inferred_colors);

    println!("Elapsed time: {}ms", start.elapsed().unwrap().as_millis());
}
