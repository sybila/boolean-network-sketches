use std::env;
use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader};
use std::path::Path;

use biodivine_lib_param_bn::{BooleanNetwork, Monotonicity};

fn print_steady_states(data_path: String) {
    let data_file = File::open(Path::new(data_path.as_str())).unwrap();
    let reader = BufReader::new(&data_file);
    let data: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    let mut i = 0;
    println!("data = {{");
    for item in data {
        print!("    \"state{}\": {{", i);
        i += 1;
        for var in item.split(" ") {
            if var == "&" {
                continue;
            }
            if var.chars().next().unwrap() == '~' {
                print!("\"{}\": 0, ", var[1..].to_string())
            } else {
                print!("\"{}\": 1, ", var.to_string())
            }
        }
        print!("}},\n")
    }
    println!("}}");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: ./print_regulations model_file");
        return;
    }
    let aeon_string = read_to_string(args[1].clone()).unwrap();
    let bn = BooleanNetwork::try_from(aeon_string.as_str()).unwrap();
    let regulatory_graph = bn.as_graph();

    println!("influences = [");
    for regulation in regulatory_graph.regulations() {
        let monotonicity = regulation.get_monotonicity().unwrap();
        let regulator = regulatory_graph.get_variable_name(regulation.get_regulator());
        let target = regulatory_graph.get_variable_name(regulation.get_target());
        let monotonicity_val = match monotonicity {
            Monotonicity::Inhibition => -1,
            Monotonicity::Activation => 1,
        };
        println!(
            "(\"{}\", \"{}\", dict(sign={})),",
            regulator, target, monotonicity_val
        );
    }
    println!("]");
    println!();

    print_steady_states("inference_data.txt".to_string());
}
