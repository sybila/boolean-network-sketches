use biodivine_lib_param_bn::symbolic_async_graph::SymbolicAsyncGraph;
use biodivine_lib_param_bn::BooleanNetwork;

use biodivine_hctl_model_checker::analysis::model_check_formula_unsafe;

use clap::Parser;

use network_sketches::create_inference_formulae::*;
use network_sketches::utils::{apply_constraints_and_restrict, check_if_result_contains_goal};

use std::convert::TryFrom;
use std::fs::read_to_string;
use std::time::SystemTime;

/// Structure to collect CLI arguments
#[derive(Parser)]
#[clap(author="Ondrej Huvar", about="Inference case study regarding T-LGL model.")]
struct Arguments {
    /// Use the basic sketch variant of the model
    #[clap(short, long, takes_value = false)]
    basic_sketch_version: bool,
}

/// Analysis of the T Cell Survival Network (T-LGL model)
/// Infers BNs from sketch including 2 attractor observations
fn case_study(basic_sketch_version: bool) {
    let aeon_string = if basic_sketch_version {
        read_to_string("benchmark_models/TLGL_reduced/TLGL_reduced_no_updates.aeon").unwrap()
    } else {
        read_to_string("benchmark_models/TLGL_reduced/TLGL_reduced_partial_updates.aeon").unwrap()
    };
    let goal_aeon_string = read_to_string("benchmark_models/TLGL_reduced/TLGL_reduced.aeon".to_string()).unwrap();

    let bn = BooleanNetwork::try_from(aeon_string.as_str()).unwrap();
    println!("Loaded model with {} vars.", bn.num_vars());
    let mut graph = SymbolicAsyncGraph::new(bn, 2).unwrap();
    println!("Model has {} parameters.", graph.symbolic_context().num_parameter_vars());

    // define the observations
    let diseased_attractor = "~Apoptosis_ & S1P & sFas & ~Fas & ~Ceramide_ & ~Caspase & MCL1 & ~BID_ & ~DISC_ & FLIP_ & ~IFNG_ & GPCR_";
    let healthy_attractor = "Apoptosis_ & ~S1P & ~sFas & ~Fas & ~Ceramide_ & ~Caspase & ~MCL1 & ~BID_ & ~DISC_ & ~FLIP_ & ~CTLA4_ & ~TCR & ~IFNG_ & ~CREB & ~P2 & ~SMAD_ & ~GPCR_ & ~IAP_";

    let mut inferred_colors = graph.mk_unit_colors();
    println!(
        "After applying static constraints, {} concretizations remain.",
        inferred_colors.approx_cardinality(),
    );

    let formulae: Vec<String> = vec![
        mk_steady_state_formula_specific(healthy_attractor.to_string()),
        mk_attractor_formula_nonspecific(diseased_attractor.to_string()),
    ];

    // first ensure attractor existence
    graph = apply_constraints_and_restrict(formulae, graph, "attractor ensured");
    println!(
        "After ensuring attractor presence, {} concretizations remain.",
        graph.mk_unit_colors().approx_cardinality(),
    );

    // then prohibit all other attractors
    let attr_set = vec![healthy_attractor.to_string(), diseased_attractor.to_string()];
    let formula = mk_forbid_other_attractors_formula(attr_set);
    inferred_colors = model_check_formula_unsafe(formula, &graph).colors();
    println!(
        "{} suitable networks found in total",
        inferred_colors.approx_cardinality()
    );
    // println!("{}", graph.pick_witness(&inferred_colors).to_string());

    // check that original model is present among the results
    // currently does not work for the partially specified version due to syntax reasons
    if basic_sketch_version {
        check_if_result_contains_goal(graph, Some(goal_aeon_string), inferred_colors);
    }
}

/// Run the case study regarding inference of T-LGL model
fn main() {
    let args = Arguments::parse();

    let start = SystemTime::now();
    case_study(args.basic_sketch_version);
    println!("Elapsed time: {}ms", start.elapsed().unwrap().as_millis());
}
