use bn_inference_tool::attractor_inference::*;

use clap::Parser;

use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::SystemTime;

/// Structure to collect CLI arguments
#[derive(Parser)]
#[clap(author="Ondrej Huvar", version, about="Inference of BNs from partially defined model and attractors.")]
struct Arguments {
    /// Path to the file with BN model in aeon format
    model_path: String,

    /// Path to the file with attractor data
    attractor_data_path: String,

    /// Forbid all attractors not containing specified states
    #[clap(short, long, takes_value = false)]
    forbid_extra_attrs: bool,

    /// Compute with steady state attractors only
    #[clap(short, long, takes_value = false)]
    steady_states: bool,

    /// Goal model to check for in the resulting ensemble
    #[clap(short, long)]
    goal_model: Option<String>,
}

/// Infer Boolean network using network sketches with attractor data.
/// Sketch is given by 4 components: influence graph, PSBN, static and dynamic properties.
/// The first three are given using aeon model format.
/// Only dynamic properties allowed are attractor data.
fn main() {
    let args = Arguments::parse();
    let goal_aeon_string: Option<String> = match args.goal_model {
        None => None,
        Some(file_name) => Some(read_to_string(file_name.to_string()).unwrap()),
    };

    let data_file = File::open(Path::new(args.attractor_data_path.as_str())).unwrap();
    let reader = BufReader::new(&data_file);
    let data: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    let aeon_string = read_to_string(args.model_path).unwrap();

    let start = SystemTime::now();
    perform_inference_with_attractors_specific(
        data,
        aeon_string,
        args.steady_states,
        args.forbid_extra_attrs,
        goal_aeon_string,
    );
    println!("Elapsed time: {}ms", start.elapsed().unwrap().as_millis());
}
