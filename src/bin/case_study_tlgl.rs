use network_sketches::create_inference_formulae::*;
#[allow(unused_imports)]
use network_sketches::utils::{
    apply_constraints_and_restrict, check_if_result_contains_goal, summarize_candidates_naively
};

use biodivine_hctl_model_checker::analysis::{get_extended_symbolic_graph, model_check_formula};

use biodivine_lib_param_bn::BooleanNetwork;
use biodivine_lib_param_bn::biodivine_std::traits::Set;

use clap::Parser;

use std::fs::read_to_string;
use std::time::SystemTime;

/// Structure to collect CLI arguments
#[derive(Parser)]
#[clap(
    author="Ondrej Huvar",
    about="Inference case study regarding T-LGL model."
)]
struct Arguments {
    /// Use the refined variant of the sketch
    #[clap(short, long, takes_value = false)]
    refined_sketch: bool,

    /// Print summarizing info regarding candidate set
    #[clap(short, long, takes_value = false)]
    summarize_candidates: bool,
}

fn case_study_part_1() {
    let start = SystemTime::now();
    let aeon_string =
        read_to_string("benchmark_models/TLGL_reduced/TLGL_reduced_no_updates.aeon").unwrap();

    // create the partially specified BN
    let bn = BooleanNetwork::try_from(aeon_string.as_str()).unwrap();
    println!("Loaded model with {} vars.", bn.num_vars());
    let mut graph = get_extended_symbolic_graph(&bn, 2);
    println!("Model has {} parameters.", graph.symbolic_context().num_parameter_variables());
    println!("----------");

    // apply static constraints
    println!(
        "After applying static constraints, {} concretizations remain.",
        graph.mk_unit_colors().approx_cardinality(),
    );
    println!("Elapsed time: {}ms", start.elapsed().unwrap().as_millis());
    println!("----------");

    // define data observation and corresponding dynamic property
    let diseased_attractor = "~Apoptosis_ & S1P & sFas & ~Fas & ~Ceramide_ & ~Caspase & MCL1 & ~BID_ & ~DISC_ & FLIP_ & ~IFNG_ & GPCR_";
    let formulae: Vec<String> = vec![mk_attractor_formula_nonspecific(diseased_attractor.to_string()), ];

    // apply dynamic constraints
    graph = apply_constraints_and_restrict(formulae, graph, "attractor ensured");
    println!(
        "After ensuring attractor presence, {} concretizations remain.",
        graph.mk_unit_colors().approx_cardinality(), // graph has restricted unit colors to satisfying ones
    );
    println!("Elapsed time: {}ms", start.elapsed().unwrap().as_millis());
    println!("----------");

    let attrs_all_candidates = model_check_formula("!{x}: AG EF {x}".to_string(), &graph).unwrap();
    println!("Attractors for all candidates computed");
    println!("Elapsed time: {}ms", start.elapsed().unwrap().as_millis());
    println!("----------");

    let programmed_cell_death_formula = "Apoptosis_ & ~S1P & ~sFas & ~Fas & ~Ceramide_ & ~Caspase & ~MCL1 & ~BID_ & ~DISC_ & ~FLIP_ & ~CTLA4_ & ~TCR & ~IFNG_ & ~CREB & ~P2 & ~SMAD_ & ~GPCR_ & ~IAP_";
    let pcd = model_check_formula(programmed_cell_death_formula.to_string(), &graph).unwrap();
    let colors_not_pcd = graph.mk_unit_colors().minus(&attrs_all_candidates.intersect(&pcd).colors());
    println!("There are {} colors without programmed cell death attractor, such as:\n", colors_not_pcd.approx_cardinality());
    println!("{}", graph.pick_witness(&colors_not_pcd).to_bnet(false).unwrap());
    println!("----------");

    let unwanted_state_formula = "Apoptosis_ & (S1P | sFas | Fas | Ceramide_ | Caspase  | MCL1 | BID_ | DISC_  | FLIP_ | CTLA4_ | TCR | IFNG_ | CREB  | P2 | SMAD_ | GPCR_ | IAP_)";
    let unwanted_states = model_check_formula(unwanted_state_formula.to_string(), &graph).unwrap();
    let colors_with_unwanted_states = attrs_all_candidates.intersect(&unwanted_states).colors();
    println!("There are {} colors with unwanted states in attractors, such as:\n", colors_with_unwanted_states.approx_cardinality());
    println!("{}", graph.pick_witness(&colors_with_unwanted_states).to_bnet(false).unwrap());
    println!("----------");

    println!("Elapsed time: {}ms", start.elapsed().unwrap().as_millis());
}

fn case_study_part_2(summarize_candidates: bool) {
    let aeon_string =
        read_to_string("benchmark_models/TLGL_reduced/TLGL_reduced_partial_updates.aeon").unwrap();

    // create the partially specified BN
    let bn = BooleanNetwork::try_from(aeon_string.as_str()).unwrap();
    println!("Loaded model with {} vars.", bn.num_vars());
    let mut graph = get_extended_symbolic_graph(&bn, 2);
    println!("Model has {} parameters.", graph.symbolic_context().num_parameter_variables());
    println!("----------");

    // apply static constraints
    println!(
        "After applying static constraints, {} concretizations remain.",
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
    graph = apply_constraints_and_restrict(formulae, graph, "attractor ensured");
    println!(
        "After ensuring attractor presence, {} concretizations remain.",
        graph.mk_unit_colors().approx_cardinality(),
    );

    // then prohibit all other attractors
    let attr_set = vec![healthy_attractor.to_string(), diseased_attractor.to_string()];
    let formula = mk_forbid_other_attractors_formula(attr_set);
    let inferred_colors = model_check_formula(formula, &graph).unwrap().colors();
    println!(
        "{} suitable networks found in total",
        inferred_colors.approx_cardinality()
    );
    println!("----------");

    println!("{}", graph.pick_witness(&inferred_colors).to_bnet(false).unwrap());
    if summarize_candidates {
        summarize_candidates_naively(&graph, inferred_colors.clone());
    }
}


/// Analysis of the T Cell Survival Network (T-LGL model)
/// Infers BNs from sketch including 2 attractor observations
#[allow(dead_code)]
fn case_study_original(fully_unspecified_logic: bool, summarize_candidates: bool) {
    let aeon_string = if fully_unspecified_logic {
        read_to_string("benchmark_models/TLGL_reduced/TLGL_reduced_no_updates.aeon").unwrap()
    } else {
        read_to_string("benchmark_models/TLGL_reduced/TLGL_reduced_partial_updates.aeon").unwrap()
    };
    let goal_aeon_string =
        read_to_string("benchmark_models/TLGL_reduced/TLGL_reduced.aeon".to_string()).unwrap();

    let bn = BooleanNetwork::try_from(aeon_string.as_str()).unwrap();
    println!("Loaded model with {} vars.", bn.num_vars());
    let mut graph = get_extended_symbolic_graph(&bn, 2);
    println!("Model has {} parameters.", graph.symbolic_context().num_parameter_variables());
    println!("----------");

    // define the observations
    let diseased_attractor = "~Apoptosis_ & S1P & sFas & ~Fas & ~Ceramide_ & ~Caspase & MCL1 & ~BID_ & ~DISC_ & FLIP_ & ~IFNG_ & GPCR_";
    let healthy_attractor = "Apoptosis_ & ~S1P & ~sFas & ~Fas & ~Ceramide_ & ~Caspase & ~MCL1 & ~BID_ & ~DISC_ & ~FLIP_ & ~CTLA4_ & ~TCR & ~IFNG_ & ~CREB & ~P2 & ~SMAD_ & ~GPCR_ & ~IAP_";

    let mut inferred_colors = graph.mk_unit_colors();
    println!(
        "After applying static constraints, {} concretizations remain.",
        inferred_colors.approx_cardinality(),
    );
    println!("----------");

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
    let attr_set = vec![
        healthy_attractor.to_string(),
        diseased_attractor.to_string()
    ];
    let formula = mk_forbid_other_attractors_formula(attr_set);
    inferred_colors = model_check_formula(formula, &graph).unwrap().colors();
    println!(
        "{} suitable networks found in total",
        inferred_colors.approx_cardinality()
    );
    println!("----------");

    //println!("{}", graph.pick_witness(&inferred_colors).to_bnet(false).unwrap());
    if summarize_candidates {
        summarize_candidates_naively(&graph, inferred_colors.clone());
    }

    // check that original model is present among the results
    // currently does not work for the partially specified version due to syntax reasons
    if fully_unspecified_logic {
        check_if_result_contains_goal(graph, Some(goal_aeon_string), inferred_colors);
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

    //case_study_original(args.fully_unspecified_logic, args.summarize_candidates);
}

#[cfg(test)]
mod tests {
    use biodivine_lib_param_bn::BooleanNetwork;

    use biodivine_hctl_model_checker::analysis::{get_extended_symbolic_graph, model_check_formula};

    use network_sketches::create_inference_formulae::*;
    use network_sketches::utils::apply_constraints_and_restrict;
    use std::fs::read_to_string;

    #[test]
    /// Test BN inference of partially defined tlgl model
    /// Use previously computed data to check results
    fn test_case_study_tlgl_small() {
        let aeon_string =
            read_to_string("benchmark_models/TLGL_reduced/TLGL_reduced_partial_updates.aeon").unwrap();
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
            diseased_attractor.to_string()
        ];
        let formula = mk_forbid_other_attractors_formula(attr_set);
        let inferred_colors = model_check_formula(formula, &graph).unwrap().colors();
        assert_eq!(inferred_colors.approx_cardinality(), 378.);
    }
}