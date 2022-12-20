use clap::Parser;

use biodivine_lib_param_bn::BooleanNetwork;

use boolean_network_sketches::inference_attractor_data::*;
use boolean_network_sketches::utils::check_if_result_contains_goal;

use biodivine_hctl_model_checker::analysis::get_extended_symbolic_graph;
use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::SystemTime;

/// Structure to collect CLI arguments
#[derive(Parser)]
#[clap(
    author = "Ondřej Huvar",
    about = "Inference of BNs from partially defined model and attractors."
)]
struct Arguments {
    /// Path to a file with a model in aeon format
    model_path: String,

    /// Path to a file with attractor data (one encoded state per line)
    attractor_data_path: String,

    /// Allow attractors not containing specified states (otherwise, a property prohibiting these attractors is used)
    #[clap(short, long, takes_value = false)]
    allow_extra_attrs: bool,

    /// In dynamic properties, address fixed-point attractors only (instead of general attractors)
    #[clap(short, long, takes_value = false)]
    fixed_points: bool,

    /// Path to a fully specified BN model to look for in the resulting set of candidates
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
        args.allow_extra_attrs,
        goal_aeon_string.is_some(),
    );

    let data_file = File::open(Path::new(args.attractor_data_path.as_str())).unwrap();
    let reader = BufReader::new(&data_file);
    let data: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    let aeon_string = read_to_string(args.model_path).unwrap();

    let bn = BooleanNetwork::try_from(aeon_string.as_str()).unwrap();
    println!("Loaded BN model with {} components.", bn.num_vars());

    // Create extended graph object with 1 HCTL var (we dont need more)
    let graph = get_extended_symbolic_graph(&bn, 1);
    println!(
        "Model has {} symbolic parameters.",
        graph.symbolic_context().num_parameter_variables()
    );
    println!("-------");

    let inferred_colors = perform_inference_with_attractors_specific(
        data,
        graph.clone(),
        args.fixed_points,
        !args.allow_extra_attrs,
    );
    println!("-------");

    println!(
        "{} consistent candidate networks found in total",
        inferred_colors.approx_cardinality()
    );

    // check whether goal network (if supplied) is part of the solution set
    check_if_result_contains_goal(graph, goal_aeon_string, inferred_colors);

    println!("Elapsed time: {}ms", start.elapsed().unwrap().as_millis());
}
