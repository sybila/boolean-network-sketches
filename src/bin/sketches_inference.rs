//! Run the general inference process using network sketches.
//!
//! Takes model in aeon format and set of HCTL formulae as a sketch description.
//! Computes the set of all consistent networks.
//! Outputs summary of candidate set and a witness network, if desired.

use biodivine_hctl_model_checker::model_checking::get_extended_symbolic_graph;

use biodivine_lib_param_bn::BooleanNetwork;

use boolean_network_sketches::utils::apply_constraints_and_restrict;

use clap::Parser;

use std::fs::read_to_string;
use std::time::SystemTime;

/// Structure to collect CLI arguments.
#[derive(Parser)]
#[clap(
author = "Ondřej Huvar",
about = "Inference through BN sketches."
)]
struct Arguments {
    /// Path to a file with a model in aeon format.
    model_path: String,

    /// Path to a file with a set of HCTL formulae (one per line).
    formulae_path: String,

    /// Print a consistent network at the end (if there is some).
    #[clap(short, long, takes_value = false)]
    print_witness: bool,
}

/// Read the formulae from the specified file.
/// The syntax of these formulae is checked later during parsing.
fn load_formulae(formulae_path: String) -> Vec<String> {
    let formulae_string = read_to_string(formulae_path).unwrap();
    let mut formulae: Vec<String> = Vec::new();
    for line in formulae_string.lines() {
        if !line.trim().is_empty() {
            formulae.push(line.trim().to_string());
        }
    }
    formulae
}

/// Run the inference process
fn main() {
    let args = Arguments::parse();
    let start = SystemTime::now();

    let aeon_string = read_to_string(args.model_path).unwrap();
    let bn = BooleanNetwork::try_from(aeon_string.as_str()).unwrap();
    println!("Loaded BN model with {} components.", bn.num_vars());

    // Create extended graph object with maximum of 3 HCTL var
    // this can be easily extended, there is just no need for more now
    let graph = get_extended_symbolic_graph(&bn, 3);
    println!(
        "Model has {} symbolic parameters.",
        graph.symbolic_context().num_parameter_variables()
    );
    println!("-------");

    // collect the formulae
    let formulae = load_formulae(args.formulae_path);

    // perform the colored model checking
    let graph = apply_constraints_and_restrict(formulae, graph, "property ensured");

    println!(
        "{} consistent candidate networks found in total.",
        graph.mk_unit_colors().approx_cardinality(), // graph has restricted unit colors to satisfying ones
    );

    if args.print_witness {
        println!("-------");
        println!("witness network:\n", );
        print!("{}",
               graph.pick_witness(&graph.mk_unit_colors()).to_bnet(false).unwrap()
        );
        println!("-------");

    }

    println!(
        "Elapsed time from the start of this computation: {}ms",
        start.elapsed().unwrap().as_millis()
    );
}
