//! Run the general inference process using network sketches.
//!
//! Takes model in aeon format and set of HCTL formulae as a sketch description.
//! Computes the set of all consistent networks.
//! Outputs summary of candidate set or set of witness networks, if desired.

use biodivine_hctl_model_checker::model_checking::{
    collect_unique_hctl_vars, get_extended_symbolic_graph,
};
use biodivine_hctl_model_checker::preprocessing::node::HctlTreeNode;
use biodivine_hctl_model_checker::preprocessing::parser::parse_and_minimize_hctl_formula;

use biodivine_lib_param_bn::biodivine_std::traits::Set;
use biodivine_lib_param_bn::{BooleanNetwork, ModelAnnotation};

use boolean_network_sketches::utils::{
    apply_constraint_trees_and_restrict, pick_random_color, summarize_candidates_naively,
};

use clap::Parser;

use std::cmp::max;
use std::collections::HashSet;
use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Structure to collect CLI arguments.
#[derive(Parser)]
#[clap(author = "Ondrej Huvar", about = "Inference through BN sketches.")]
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

/// Run the inference process
fn main() {
    let start = SystemTime::now();
    let mut rng = rand::thread_rng();

    // parse the CLI args
    let args = Arguments::parse();
    let model_path = args.model_path;
    let summarize_candidates = args.summarize_candidates;
    let n_witnesses = args.n_witnesses;
    let mut witness_dir = args.witness_dir; // make mut to be able to set it to "" if dir is invalid

    // load the BN and properties from the model file
    let aeon_string = read_to_string(model_path).unwrap();
    let bn = BooleanNetwork::try_from(aeon_string.as_str()).unwrap();
    println!("Loaded BN model with {} components.", bn.num_vars());
    let annotations = ModelAnnotation::from_model_string(aeon_string.as_str());
    let named_properties = read_model_properties(&annotations).unwrap();

    // compute number of symbolic vars needed to represent the HCTL properties
    let mut num_hctl_vars = 0;
    let mut property_trees: Vec<HctlTreeNode> = Vec::new();
    for (_name, formula) in &named_properties {
        let tree = parse_and_minimize_hctl_formula(&bn, formula.as_str()).unwrap();
        let num_tree_vars = collect_unique_hctl_vars(tree.clone(), HashSet::new()).len();
        num_hctl_vars = max(num_hctl_vars, num_tree_vars);
        property_trees.push(tree);
    }

    // generate the extended STG
    let graph = get_extended_symbolic_graph(&bn, num_hctl_vars as u16);
    println!(
        "Model has {} symbolic parameters.",
        graph.symbolic_context().num_parameter_variables()
    );
    println!("\n---------------------\n");

    // perform the colored model checking
    let graph = apply_constraint_trees_and_restrict(property_trees, graph, "property ensured");
    let valid_colors = graph.mk_unit_colors(); // graph's unit colors have been restricted to consistent ones
    println!(
        "{} consistent candidate networks found in total.",
        valid_colors.approx_cardinality()
    );
    println!("\n---------------------\n");

    // summarize the complete results if required
    if summarize_candidates {
        println!("Summarization of update fns of ALL CONSISTENT CANDIDATES:");
        summarize_candidates_naively(&graph, valid_colors.clone(), true);
        println!("\n---------------------\n");
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
        println!("\nSummarization of update fns of WITNESSES:");
        summarize_candidates_naively(&graph, witness_colors, true);
        println!("\n---------------------\n");
    }

    println!(
        "Elapsed time from the start of this computation: {}ms",
        start.elapsed().unwrap().as_millis()
    );
}
