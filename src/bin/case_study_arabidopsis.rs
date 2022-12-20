#[allow(unused_imports)]
use boolean_network_sketches::inference_attractor_data::perform_inference_with_attractors_specific;
use boolean_network_sketches::utils::summarize_candidates_naively;

use biodivine_hctl_model_checker::analysis::get_extended_symbolic_graph;

use biodivine_lib_param_bn::BooleanNetwork;

use clap::Parser;

use std::fs::read_to_string;
use std::time::SystemTime;

/// Structure to collect CLI arguments
#[derive(Parser)]
#[clap(
    author = "Ondřej Huvar",
    about = "Inference case study regarding A. Thaliana."
)]
struct Arguments {
    /// Use the modified variant of the sketch with complex dynamic properties
    #[clap(short, long, takes_value = false)]
    modified_sketch_variant: bool,

    /// Print summarizing info regarding candidates' update functions (may take a long time)
    #[clap(short, long, takes_value = false)]
    summarize_candidates: bool,
}

/// Analysis of the A. thaliana Sepal Primordium Polarity
/// Infers BNs from sketch including attractor data
fn case_study(fixed_point_version: bool, prohibit_extra_attrs: bool, summarize: bool) {
    // parse BN object
    let aeon_string =
        read_to_string("benchmark_models/case_study_arabidopsis/arabidopsis.aeon").unwrap();
    let bn = BooleanNetwork::try_from(aeon_string.as_str()).unwrap();
    println!("Loaded BN model with {} components.", bn.num_vars());

    // Create extended symbolic graph object with 1 HCTL var (we dont need more)
    let graph = get_extended_symbolic_graph(&bn, 1);
    println!(
        "Model has {} symbolic parameters.",
        graph.symbolic_context().num_parameter_variables()
    );
    println!("-------");

    // define observations
    let observation1 = "AGO1 & ~AGO10 & ~AGO7 & ANT & ARF4 & ~AS1 & ~AS2 & ETT & FIL & KAN1 & miR165 & miR390 & ~REV & ~TAS3siRNA & AGO1_miR165 & ~AGO7_miR390 & ~AS1_AS2 & AUXINh & ~CKh & ~GTE6 & ~IPT5";
    let observation2 = "~AGO1 & AGO10 & AGO7 & ANT & ~ARF4 & AS1 & AS2 & ~ETT & ~FIL & ~KAN1 & ~miR165 & miR390 & REV & TAS3siRNA & ~AGO1_miR165 & AGO7_miR390 & AS1_AS2 & AUXINh & CKh & GTE6 & IPT5";

    // perform the colored model checking
    let inferred_colors = perform_inference_with_attractors_specific(
        vec![observation1.to_string(), observation2.to_string()],
        graph.clone(),
        fixed_point_version,
        prohibit_extra_attrs,
    );

    println!(
        "{} consistent candidate networks found in total",
        inferred_colors.approx_cardinality()
    );
    println!("-------");

    if summarize {
        // summarize which update functions are unique for all candidates and which vary
        summarize_candidates_naively(&graph, inferred_colors.clone());
    }
}

/// Run the case study regarding inference of A. Thaliana model
fn main() {
    let args = Arguments::parse();
    let start = SystemTime::now();

    let sketch_mode = if args.modified_sketch_variant {
        "modified variant of the sketch with complex properties"
    } else {
        "initial variant of the sketch with fixed-point properties"
    };
    println!("MODE: {}", sketch_mode);

    // run the variant of the case study specified by the user
    if args.modified_sketch_variant {
        case_study(false, true, args.summarize_candidates);
    } else {
        case_study(true, false, args.summarize_candidates);
    }

    println!(
        "Elapsed time from the start of this computation: {}ms",
        start.elapsed().unwrap().as_millis()
    );
}
