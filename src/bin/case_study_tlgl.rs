use boolean_network_sketches::create_inference_formulae::*;
#[allow(unused_imports)]
use boolean_network_sketches::utils::{
    apply_constraints_and_restrict, check_if_result_contains_goal, summarize_candidates_naively,
};

use biodivine_hctl_model_checker::analysis::{get_extended_symbolic_graph, model_check_formula};

use biodivine_lib_param_bn::biodivine_std::traits::Set;
use biodivine_lib_param_bn::BooleanNetwork;

use clap::Parser;

use std::fs::read_to_string;
use std::time::SystemTime;

/// Structure to collect CLI arguments
#[derive(Parser)]
#[clap(
    author = "Ondřej Huvar",
    about = "Inference case study regarding T-LGL model."
)]
struct Arguments {
    /// Use the refined variant of the sketch
    #[clap(short, long, takes_value = false)]
    refined_sketch: bool,

    /// Print summarizing info regarding candidate set
    #[clap(short, long, takes_value = false)]
    summarize_candidates: bool,
}

/// First part of the case study regarding the initial version of the sketch
/// Analyses model with fully unspecified update logic and experimental data
/// At the end, analyses candidates by computing attractors and checking for unwanted patterns
fn case_study_part_1() {
    let start = SystemTime::now();
    let aeon_string =
        read_to_string("benchmark_models/case_study_TLGL/TLGL_reduced_unknown_updates.aeon")
            .unwrap();

    // create the partially specified BN
    let bn = BooleanNetwork::try_from(aeon_string.as_str()).unwrap();
    println!("Loaded BN model with {} components.", bn.num_vars());
    let mut graph = get_extended_symbolic_graph(&bn, 2);
    println!(
        "Model has {} symbolic parameters.",
        graph.symbolic_context().num_parameter_variables()
    );
    println!("----------");

    // apply update function properties
    println!(
        "After applying update function properties, {} concretizations remain.",
        graph.mk_unit_colors().approx_cardinality(),
    );
    println!("Elapsed time: {}ms", start.elapsed().unwrap().as_millis());
    println!("----------");

    // define data observation and corresponding dynamic property
    let diseased_attractor = "~Apoptosis_ & S1P & sFas & ~Fas & ~Ceramide_ & ~Caspase & MCL1 & ~BID_ & ~DISC_ & FLIP_ & ~IFNG_ & GPCR_";
    let formulae: Vec<String> = vec![mk_attractor_formula_nonspecific(
        diseased_attractor.to_string(),
    )];

    // apply dynamic constraints
    graph = apply_constraints_and_restrict(formulae, graph, "attractor property ensured");
    println!(
        "{} consistent networks found in total.",
        graph.mk_unit_colors().approx_cardinality(), // graph has restricted unit colors to satisfying ones
    );
    println!("Elapsed time: {}ms", start.elapsed().unwrap().as_millis());
    println!("----------");

    // analyse candidates
    println!("Analysing candidate set...");

    // compute attractors symbolically
    let attrs_all_candidates = model_check_formula("!{x}: AG EF {x}".to_string(), &graph).unwrap();
    println!("Attractors for all candidates computed");
    println!("Elapsed time: {}ms", start.elapsed().unwrap().as_millis());
    println!("----------");

    // check for candidates without attractor for programmed cell death
    let programmed_cell_death_formula = "Apoptosis_ & ~S1P & ~sFas & ~Fas & ~Ceramide_ & ~Caspase & ~MCL1 & ~BID_ & ~DISC_ & ~FLIP_ & ~CTLA4_ & ~TCR & ~IFNG_ & ~CREB & ~P2 & ~SMAD_ & ~GPCR_ & ~IAP_";
    let pcd = model_check_formula(programmed_cell_death_formula.to_string(), &graph).unwrap();
    let colors_not_pcd = graph
        .mk_unit_colors()
        .minus(&attrs_all_candidates.intersect(&pcd).colors());
    println!(
        "There are {} candidates without programmed cell death attractor, such as:\n",
        colors_not_pcd.approx_cardinality()
    );
    print!(
        "{}",
        graph.pick_witness(&colors_not_pcd).to_bnet(false).unwrap()
    );
    println!("----------");

    // check for candidates with unwanted attractor states
    let unwanted_state_formula = "Apoptosis_ & (S1P | sFas | Fas | Ceramide_ | Caspase  | MCL1 | BID_ | DISC_  | FLIP_ | CTLA4_ | TCR | IFNG_ | CREB  | P2 | SMAD_ | GPCR_ | IAP_)";
    let unwanted_states = model_check_formula(unwanted_state_formula.to_string(), &graph).unwrap();
    let colors_with_unwanted_states = attrs_all_candidates.intersect(&unwanted_states).colors();
    println!(
        "There are {} candidates with unwanted states in attractors, such as:\n",
        colors_with_unwanted_states.approx_cardinality()
    );
    print!(
        "{}",
        graph
            .pick_witness(&colors_with_unwanted_states)
            .to_bnet(false)
            .unwrap()
    );
    println!("----------");

    println!("Elapsed time: {}ms", start.elapsed().unwrap().as_millis());
}

/// Second part of the case study regarding the refined version of the sketch
/// Extends previous sketch with partially specified update logic and hypotheses regarding
/// additional attractors
/// At the end, prints a witness candidate, and summarizes all candidates
fn case_study_part_2(summarize_candidates: bool) {
    let start = SystemTime::now();
    let aeon_string =
        read_to_string("benchmark_models/case_study_TLGL/TLGL_reduced_partial_updates.aeon")
            .unwrap();

    // create the partially specified BN object
    let bn = BooleanNetwork::try_from(aeon_string.as_str()).unwrap();
    println!("Loaded BN model with {} components.", bn.num_vars());
    let mut graph = get_extended_symbolic_graph(&bn, 2);
    println!(
        "Model has {} symbolic parameters.",
        graph.symbolic_context().num_parameter_variables()
    );
    println!("----------");

    // apply update function properties
    println!(
        "After applying update function properties, {} concretizations remain.",
        graph.mk_unit_colors().approx_cardinality(),
    );
    println!("----------");

    // define both observations and corresponding properties
    let diseased_attractor = "~Apoptosis_ & S1P & sFas & ~Fas & ~Ceramide_ & ~Caspase & MCL1 & ~BID_ & ~DISC_ & FLIP_ & ~IFNG_ & GPCR_";
    let healthy_attractor = "Apoptosis_ & ~S1P & ~sFas & ~Fas & ~Ceramide_ & ~Caspase & ~MCL1 & ~BID_ & ~DISC_ & ~FLIP_ & ~CTLA4_ & ~TCR & ~IFNG_ & ~CREB & ~P2 & ~SMAD_ & ~GPCR_ & ~IAP_";
    let formulae: Vec<String> = vec![
        mk_steady_state_formula_specific(healthy_attractor.to_string()),
        mk_attractor_formula_nonspecific(diseased_attractor.to_string()),
    ];

    // first ensure attractor existence
    graph = apply_constraints_and_restrict(formulae, graph, "attractor property ensured");
    println!(
        "After ensuring both properties regarding attractor presence, {} concretizations remain.",
        graph.mk_unit_colors().approx_cardinality(),
    );

    // then prohibit all other attractors
    let attr_set = vec![
        healthy_attractor.to_string(),
        diseased_attractor.to_string(),
    ];
    let formula = mk_forbid_other_attractors_formula(attr_set);
    let inferred_colors = model_check_formula(formula, &graph).unwrap().colors();
    println!(
        "{} consistent networks found in total",
        inferred_colors.approx_cardinality()
    );
    println!("Elapsed time: {}ms", start.elapsed().unwrap().as_millis());
    println!("----------");

    // print a withess network
    println!("ONE OF THE CANDIDATE NETWORKS:\n");
    print!(
        "{}",
        graph.pick_witness(&inferred_colors).to_bnet(false).unwrap()
    );
    println!("----------");

    // summarize differences and similarities between candidates
    println!("SUMMARIZATION OF CANDIDATES' UPDATE FUNCTIONS:\n");
    if summarize_candidates {
        summarize_candidates_naively(&graph, inferred_colors.clone());
    }
}

/// Run the case study regarding inference of T-LGL model
fn main() {
    let args = Arguments::parse();

    let refined_sketch = if args.refined_sketch {
        "refined variant of the sketch"
    } else {
        "initial variant of the sketch"
    };
    println!("MODE: {}", refined_sketch);

    if args.refined_sketch {
        case_study_part_2(true);
    } else {
        case_study_part_1()
    }
}

#[cfg(test)]
mod tests {
    use biodivine_hctl_model_checker::analysis::{
        get_extended_symbolic_graph, model_check_formula,
    };

    use biodivine_lib_param_bn::BooleanNetwork;

    use boolean_network_sketches::create_inference_formulae::*;
    use boolean_network_sketches::utils::apply_constraints_and_restrict;

    use std::fs::read_to_string;

    #[test]
    /// Test BN inference of partially defined tlgl model
    /// Use previously computed data to check results
    fn test_case_study_tlgl_small() {
        let aeon_string =
            read_to_string("benchmark_models/case_study_TLGL/TLGL_reduced_partial_updates.aeon")
                .unwrap();
        let bn = BooleanNetwork::try_from(aeon_string.as_str()).unwrap();
        let mut graph = get_extended_symbolic_graph(&bn, 2);

        // define the observations
        let diseased_attractor = "~Apoptosis_ & S1P & sFas & ~Fas & ~Ceramide_ & ~Caspase & MCL1 & ~BID_ & ~DISC_ & FLIP_ & ~IFNG_ & GPCR_";
        let healthy_attractor = "Apoptosis_ & ~S1P & ~sFas & ~Fas & ~Ceramide_ & ~Caspase & ~MCL1 & ~BID_ & ~DISC_ & ~FLIP_ & ~CTLA4_ & ~TCR & ~IFNG_ & ~CREB & ~P2 & ~SMAD_ & ~GPCR_ & ~IAP_";

        let formulae: Vec<String> = vec![
            mk_steady_state_formula_specific(healthy_attractor.to_string()),
            mk_attractor_formula_nonspecific(diseased_attractor.to_string()),
        ];

        // first ensure attractor existence
        graph = apply_constraints_and_restrict(formulae, graph, "attractor ensured");
        // prohibit all other attractors
        let attr_set = vec![
            healthy_attractor.to_string(),
            diseased_attractor.to_string(),
        ];
        let formula = mk_forbid_other_attractors_formula(attr_set);
        let inferred_colors = model_check_formula(formula, &graph).unwrap().colors();
        assert_eq!(inferred_colors.approx_cardinality(), 378.);
    }
}
