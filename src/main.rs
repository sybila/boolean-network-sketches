//! Run the general inference process using network sketches.
//!
//! Takes model in aeon format and set of HCTL formulae as a sketch description.
//! Computes the set of all consistent networks.
//! Outputs summary of candidate set or set of witness networks, if desired.

use biodivine_hctl_model_checker::mc_utils::{
    collect_unique_hctl_vars, get_extended_symbolic_graph,
};
use biodivine_hctl_model_checker::preprocessing::hctl_tree::HctlTreeNode;
use biodivine_hctl_model_checker::preprocessing::parser::parse_and_minimize_hctl_formula;

use biodivine_lib_param_bn::biodivine_std::traits::Set;
use biodivine_lib_param_bn::symbolic_async_graph::SymbolicContext;
use biodivine_lib_param_bn::{BooleanNetwork, ModelAnnotation};

use boolean_network_sketches::utils::{
    apply_constraint_trees_and_restrict, pick_random_color, summarize_candidates_naively,
};

use clap::Parser;

use std::cmp::max;
use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Structure to collect CLI arguments.
#[derive(Parser)]
#[clap(
    author = "Ondrej Huvar",
    about = "Model inference through BN sketches."
)]
struct Arguments {
    /// Path to a file with a model in aeon format. Properties are given as model annotations.
    model_path: String,

    /// Summarize all consistent candidates after finishing computation.
    #[clap(short, long, num_args = 0)]
    summarize_candidates: bool,

    /// Output N consistent networks at the end (if there are).
    #[clap(short, long, default_value = "0")]
    n_witnesses: i32,

    /// Write the witnesses to files in the directory (that must already exist),
    /// one network to one file (if no argument given, just print witnesses).
    #[clap(short, long, default_value = "")]
    witness_dir: String,
}

/// Read the list of named properties from an `.aeon` model annotation object.
///
/// The properties are expected to appear as `#!dynamic_property: NAME: FORMULA` model annotations.
/// They are returned in alphabetic order w.r.t. the property name.
fn read_model_properties(annotations: &ModelAnnotation) -> Result<Vec<(String, String)>, String> {
    let Some(property_node) = annotations.get_child(&["dynamic_property"]) else {
        return Ok(Vec::new());
    };
    let mut properties = Vec::with_capacity(property_node.children().len());
    for (name, child) in property_node.children() {
        if !child.children().is_empty() {
            return Err(format!("Property `{name}` contains nested values."));
        }
        let Some(value) = child.value() else {
            return Err(format!("Found empty dynamic property `{name}`."));
        };
        if value.lines().count() > 1 {
            return Err(format!("Found multiple properties named `{name}`."));
        }
        properties.push((name.clone(), value.clone()));
    }
    // Sort alphabetically to avoid possible non-determinism down the line.
    properties.sort_by(|(x, _), (y, _)| x.cmp(y));
    Ok(properties)
}

/// Perform the inference of Boolean networks from the input sketch.
pub fn run_inference(
    model_path: String,
    summarize_candidates: bool,
    n_witnesses: i32,
    mut witness_dir: String,
) -> Result<(), String> {
    let mut rng = rand::thread_rng();
    let start = SystemTime::now();

    // load the BN and properties from the model file
    println!("INPUT PRE-PROCESSING\n");
    if !Path::new(model_path.as_str()).is_file() {
        return Err(format!("{model_path} is not valid file"));
    }

    // load the model and two sets of formulae (from model annotations)
    let Ok(aeon_string) = read_to_string(model_path.clone()) else {
        return Err(format!("Input file `{model_path}` is not accessible."));
    };
    let bn = BooleanNetwork::try_from(aeon_string.as_str())?;
    let annotations = ModelAnnotation::from_model_string(aeon_string.as_str());
    let named_properties = read_model_properties(&annotations)?;
    println!("Loaded model and properties out of `{model_path}`.");

    // parse formulae and compute number of symbolic vars needed to represent the HCTL properties
    println!("Parsing formulae and generating symbolic representation...");
    let mut num_hctl_vars = 0;
    let mut property_trees: Vec<HctlTreeNode> = Vec::new();
    let plain_context = SymbolicContext::new(&bn).unwrap();
    for (_name, formula) in &named_properties {
        let tree = parse_and_minimize_hctl_formula(&plain_context, formula.as_str())?;
        let num_tree_vars = collect_unique_hctl_vars(tree.clone()).len();
        num_hctl_vars = max(num_hctl_vars, num_tree_vars);
        property_trees.push(tree);
    }
    println!(
        "Successfully parsed all {} properties.",
        property_trees.len(),
    );

    // Instantiate extended STG with enough variables to evaluate all formulae.
    let Ok(graph) = get_extended_symbolic_graph(&bn, num_hctl_vars as u16) else {
        return Err("Unable to generate STG for provided PSBN model.".to_string());
    };
    println!(
        "Successfully encoded model with {} variables and {} parameters.",
        graph.symbolic_context().num_state_variables(),
        graph.symbolic_context().num_parameter_variables(),
    );
    println!("\n---------------------------------\nRUNNING THE INFERENCE\n");

    println!("Successfully processed update function properties.");
    println!("Processing dynamic properties...");

    // perform the colored model checking
    let graph = apply_constraint_trees_and_restrict(property_trees, graph, "- property processed");
    let valid_colors = graph.mk_unit_colors(); // graph's unit colors have been restricted to consistent ones
    println!("Successfully processed all dynamic properties.");
    println!(
        "Inference finished in {}ms.",
        start.elapsed().unwrap().as_millis()
    );

    println!(
        "{} consistent candidate networks found in total.",
        valid_colors.approx_cardinality()
    );
    println!("\n---------------------------------");

    // summarize the complete results if required
    if summarize_candidates {
        println!("SUMMARIZING ALL CONSISTENT CANDIDATES\n");
        println!("There are following variants of update functions for each variable:");
        summarize_candidates_naively(&graph, valid_colors.clone(), true);

        if n_witnesses > 0 {
            println!("\n---------------------------------\nGENERATING WITNESSES\n");
        } else {
            println!("\n---------------------------------\n");
        }
    }

    // generate random witnesses if required
    let mut i = 0;
    let mut valid_colors = valid_colors;
    let mut witness_colors = graph.mk_empty_colors();
    if !witness_dir.is_empty() {
        // check that the dir exists
        let path = PathBuf::from(witness_dir.as_str());
        if !path.is_dir() {
            println!("ERROR: {witness_dir} is not a valid directory.");
            println!("ERROR: Witnesses will only be printed.\n");
            witness_dir = "".to_string();
        }
    }
    while i < n_witnesses {
        if valid_colors.is_empty() {
            println!("There are no more witnesses");
            println!("-------");
            break;
        }

        let c = pick_random_color(&mut rng, &graph, &valid_colors);
        let witness_bn = graph.pick_witness(&c);

        if witness_dir.is_empty() {
            // just print them
            println!("witness network number {}:\n", i + 1);
            print!("{}", witness_bn.to_bnet(false).unwrap());
            println!("-------");
        } else {
            // write witness to its own file
            let file_name = format!("witness{}.aeon", i + 1);
            let file_path = Path::new(witness_dir.as_str()).join(file_name);
            let mut file = File::create(file_path).unwrap();
            file.write_all(witness_bn.to_string().as_bytes()).unwrap();
            println!("witness number {} generated", i + 1);
        }

        valid_colors = valid_colors.minus(&c);
        witness_colors = witness_colors.union(&c);
        i += 1;
    }

    // if some witnesses were generated, always summarize them
    if !witness_colors.is_empty() {
        println!("\nSummarization of update fns of ALL WITNESSES:");
        summarize_candidates_naively(&graph, witness_colors, true);
        println!("\n---------------------------------\n");
    }

    println!(
        "Total elapsed time from the start of the computation: {}ms",
        start.elapsed().unwrap().as_millis()
    );

    Ok(())
}

/// Parse inputs and run the inference process.
fn main() {
    let args = Arguments::parse();

    let inference_res = run_inference(
        args.model_path,
        args.summarize_candidates,
        args.n_witnesses,
        args.witness_dir,
    );

    if inference_res.is_err() {
        println!("Error during computation: {}", inference_res.err().unwrap())
    }
}
