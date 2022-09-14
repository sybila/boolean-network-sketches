#[allow(unused_imports)]
use hctl_model_checker::analysis::{analyse_formula, model_check_formula_unsafe, PrintOptions};

use bn_inference_tool::attractor_inference::*;

use std::env;
use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::SystemTime;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("3 arguments expected, got {}", args.len() - 1);
        println!("Usage: ./inference-attractors model_file attractor_data forbid_extra_attrs");
        return;
    }
    if !(args[3].as_str() == "false" || args[3].as_str() == "true") {
        println!(
            "Invalid argument \"{}\", it must be either \"true\" or \"false\"",
            args[3]
        );
        println!("Usage: ./inference-attractors model_file attractor_data (true | false)");
        return;
    }
    let forbid_extra_attrs = match args[3].as_str() {
        "false" => false,
        _ => true, // we need match to be exhaustive
    };

    // TODO: make this automatic from CLI
    //let goal_aeon_string = Some(read_to_string("inference_goal_model.aeon".to_string()).unwrap());
    let goal_aeon_string: Option<String> = None;

    let data_file = File::open(Path::new(args[2].as_str())).unwrap();
    let reader = BufReader::new(&data_file);
    let data: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    let aeon_string = read_to_string(args[1].clone()).unwrap();

    let start = SystemTime::now();
    perform_inference_with_attractors_specific(
        data,
        aeon_string,
        false,
        forbid_extra_attrs,
        goal_aeon_string,
    );
    println!("Elapsed time: {}ms", start.elapsed().unwrap().as_millis());
}
