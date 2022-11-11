use biodivine_lib_param_bn::BooleanNetwork;
use biodivine_lib_param_bn::symbolic_async_graph::SymbolicAsyncGraph;

#[allow(unused_imports)]
use network_sketches::utils::enumerate_candidates_naively;
use network_sketches::inference_attractor_data::perform_inference_with_attractors_specific;

use clap::Parser;

use std::fs::read_to_string;
use std::time::SystemTime;

/// Structure to collect CLI arguments
#[derive(Parser)]
#[clap(author="Ondrej Huvar", about="Inference case study regarding A. Thaliana.")]
struct Arguments {
    /// Consider only fixed-point attractors (simpler dynamical property)
    #[clap(short, long, takes_value = false)]
    fixed_point_version: bool,
}

/// Analysis of the A. thaliana Sepal Primordium Polarity
/// Infers BNs from sketch including attractor data
fn case_study(fixed_point_version: bool) {
    let observation1 = "AGO1 & ~AGO10 & ~AGO7 & ANT & ARF4 & ~AS1 & ~AS2 & ETT & FIL & KAN1 & miR165 & miR390 & ~REV & ~TAS3siRNA & AGO1_miR165 & ~AGO7_miR390 & ~AS1_AS2 & AUXINh & ~CKh & ~GTE6 & ~IPT5";
    let observation2 = "~AGO1 & AGO10 & AGO7 & ANT & ~ARF4 & AS1 & AS2 & ~ETT & ~FIL & ~KAN1 & ~miR165 & miR390 & REV & TAS3siRNA & ~AGO1_miR165 & AGO7_miR390 & AS1_AS2 & AUXINh & CKh & GTE6 & IPT5";

    let aeon_string = read_to_string("benchmark_models/griffin_2/griffin_model2.aeon").unwrap();
    let bn = BooleanNetwork::try_from(aeon_string.as_str()).unwrap();
    println!("Loaded model with {} vars.", bn.num_vars());

    // Create graph object with 1 HCTL var (we dont need more)
    let graph = SymbolicAsyncGraph::new(bn, 1).unwrap();
    println!("Model has {} parameters.", graph.symbolic_context().num_parameter_vars());

    let inferred_colors = perform_inference_with_attractors_specific(
        vec![observation1.to_string(), observation2.to_string()],
        graph.clone(),
        fixed_point_version,
        true,
    );

    println!(
        "{} suitable networks found in total",
        inferred_colors.approx_cardinality()
    );

    //enumerate_candidates_naively(&graph, inferred_colors.clone());
}

/// Run the case study regarding inference of A. Thaliana model
fn main() {
    let args = Arguments::parse();
    let start = SystemTime::now();
    case_study(args.fixed_point_version);
    println!("Elapsed time: {}ms", start.elapsed().unwrap().as_millis());
}
