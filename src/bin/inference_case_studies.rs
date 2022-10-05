use biodivine_hctl_model_checker::analysis::model_check_formula_unsafe;

use biodivine_lib_param_bn::symbolic_async_graph::SymbolicAsyncGraph;
use biodivine_lib_param_bn::BooleanNetwork;

use bn_inference_tool::inference_formulae::*;
#[allow(unused_imports)]
use bn_inference_tool::utils::*;
use bn_inference_tool::attractor_inference::perform_inference_with_attractors_specific;

use std::convert::TryFrom;
use std::fs::read_to_string;
use std::time::SystemTime;
use std::env;


/// Analysis of the T Cell Survival Network
fn case_study_1(fully_parametrized: bool) {
    let aeon_string = if fully_parametrized {
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
    // currently does not work for the specified version
    if fully_parametrized {
        check_if_result_contains_goal(graph, Some(goal_aeon_string), inferred_colors);
    }
}

/// Analysis of the A. thaliana Sepal Primordium Polarity
fn case_study_2(fixed_point_version: bool) {
    let aeon_string = read_to_string("benchmark_models/griffin_2/griffin_model2.aeon").unwrap();
    let observation1 = "AGO1 & ~AGO10 & ~AGO7 & ANT & ARF4 & ~AS1 & ~AS2 & ETT & FIL & KAN1 & miR165 & miR390 & ~REV & ~TAS3siRNA & AGO1_miR165 & ~AGO7_miR390 & ~AS1_AS2 & AUXINh & ~CKh & ~GTE6 & ~IPT5";
    let observation2 = "~AGO1 & AGO10 & AGO7 & ANT & ~ARF4 & AS1 & AS2 & ~ETT & ~FIL & ~KAN1 & ~miR165 & miR390 & REV & TAS3siRNA & ~AGO1_miR165 & AGO7_miR390 & AS1_AS2 & AUXINh & CKh & GTE6 & IPT5";

    perform_inference_with_attractors_specific(
        vec![observation1.to_string(), observation2.to_string()],
        aeon_string,
        fixed_point_version,
        true,
        None,
    );
}

/// Analysis of the central nervous system (CNS) development
/// Formula is created automatically from smaller subformulae, observations might not be linked
fn case_study_3() {
    let aeon_string = read_to_string("benchmark_models/CNS_development/model.aeon").unwrap();
    let bn = BooleanNetwork::try_from(aeon_string.as_str()).unwrap();
    println!("Loaded model with {} vars.", bn.num_vars());
    let original_graph = SymbolicAsyncGraph::new(bn, 1).unwrap();
    let mut graph = original_graph.clone();
    println!("Model has {} parameters.", graph.symbolic_context().num_parameter_vars());

    // define the observations
    /*
    "zero": {n: 0 for n in dom1}, # all nodes are 0
    "init": {n: 1 if n == "Pax6" else 0 for n in dom1}, # all nodes are 0 but Pax6
    "tM": {"Pax6": 1, "Tuj1": 0, "Scl": 0, "Aldh1L1": 0, "Olig2": 0, "Sox8": 0},
    "fT": {"Pax6": 1, "Tuj1": 1, "Brn2": 1, "Zic1": 1, "Aldh1L1": 0, "Sox8": 0},
    "tO": {"Pax6": 1, "Tuj1": 0 ,"Scl": 0, "Aldh1L1": 0, "Olig2": 1, "Sox8": 0},
    "fMS": {"Pax6": 1, "Tuj1": 0, "Zic1": 0, "Brn2": 0, "Aldh1L1": 0, "Sox8": 1},
    "tS": {"Pax6": 1, "Tuj1": 0, "Scl": 1, "Aldh1L1": 0, "Olig2": 0, "Sox8": 0},
    "fA": {"Pax6": 1, "Tuj1": 0, "Zic1": 0, "Brn2": 0, "Aldh1L1": 1, "Sox8": 0},
     */
    let zero_state = "~Pax6 & ~Hes5 & ~Mash1 & ~Scl & ~Olig2 & ~Stat3 & ~Zic1 & ~Brn2 & ~Tuj1 & ~Myt1L & ~Sox8 & ~Aldh1L1";
    let init_state = "Pax6 & ~Hes5 & ~Mash1 & ~Scl & ~Olig2 & ~Stat3 & ~Zic1 & ~Brn2 & ~Tuj1 & ~Myt1L & ~Sox8 & ~Aldh1L1";
    let t_m = "Pax6 & ~Scl & ~Olig2 & ~Tuj1 & ~Sox8 & ~Aldh1L1";
    let f_t = "Pax6 & Zic1 & Brn2 & Tuj1 & ~Sox8 & ~Aldh1L1";
    let t_o = "Pax6 & ~Scl & Olig2 & ~Tuj1 & ~Sox8 & ~Aldh1L1";
    let f_ms = "Pax6 & ~Zic1 & ~Brn2 & ~Tuj1 & Sox8 & ~Aldh1L1";
    let t_s = "Pax6 & Scl & ~Olig2 & ~Tuj1 & ~Sox8 & ~Aldh1L1";
    let f_a = "Pax6 & ~Zic1 & ~Brn2 & ~Tuj1 & ~Sox8 & Aldh1L1";

    println!(
        "After applying static constraints, {} concretizations remain.",
        graph.mk_unit_colors().approx_cardinality(),
    );

    /*
     Constraints:
     1) existential
     - fixed points: f_a, f_ms
     - trap spaces: f_t
     - reachability: init -> t_m -> f_t, init -> t_o -> f_ms, init -> t_s -> f_a
     - negative reachability: zero -/> f_t, zero -/> f_ms, zero -/> f_a
     2) universal:
     - fixed points must be only from: [f_a, f_ms, f_t, zero]
     - fixed points reachable from 'init' must be only from: [f_a, f_ms, f_t]
     (for the last one, it is enough to just prohibit reaching 'zero' fixed point from 'init'
     */

    // constraints from the first part of the case study
    let fixed_point_constraints: Vec<String> = vec![
        mk_steady_state_formula_nonspecific(f_a.to_string()),
        mk_steady_state_formula_nonspecific(f_ms.to_string()),
    ];

    let trap_space_constraints: Vec<String> = vec![
        mk_trap_space_formula(f_t.to_string()),
    ];

    let reachability_constraints: Vec<String> = vec![
        mk_reachability_chain_formula(vec![init_state.to_string(), t_m.to_string(), f_t.to_string()]),
        mk_reachability_chain_formula(vec![init_state.to_string(), t_o.to_string(), f_ms.to_string()]),
        mk_reachability_chain_formula(vec![init_state.to_string(), t_s.to_string(), f_a.to_string()]),
    ];

    let negative_reachability_constraints: Vec<String> = vec![
        mk_reachability_pair_formula(zero_state.to_string(), f_t.to_string(), false, true),
        mk_reachability_pair_formula(zero_state.to_string(), f_ms.to_string(), false, true),
        mk_reachability_pair_formula(zero_state.to_string(), f_a.to_string(), false, true),
    ];

    // constraints from the second part of the case study
    let universal_fps = vec![f_a.to_string(), f_ms.to_string(), f_t.to_string(), zero_state.to_string()];
    let universal_constraints: Vec<String> = vec![
        mk_forbid_other_steady_states_formula(universal_fps),
        // any fixed point reachable from "init" must be one of {f_a, f_ms, f_t}
        // if we use previous constraint, we can just prohibit reaching the zero fixed point
        format!("3{{x}}:@{{x}}:(({}) & ~EF(({}) & AX {}))", init_state, zero_state, zero_state),
    ];

    graph = apply_constraints_and_restrict(fixed_point_constraints.clone(), graph, "fixed point ensured");
    graph = apply_constraints_and_restrict(trap_space_constraints.clone(), graph, "trap space ensured");
    graph = apply_constraints_and_restrict(reachability_constraints.clone(), graph, "reachability ensured");
    graph = apply_constraints_and_restrict(negative_reachability_constraints.clone(), graph, "non-reachability ensured");
    println!(
        "After the first set of constraints, {} concretizations remain.",
        graph.unit_colors().approx_cardinality(),
    );

    graph = apply_constraints_and_restrict(universal_constraints, graph, "universal constraint ensured");
    println!(
        "After the second set of constraints, {} concretizations remain.",
        graph.unit_colors().approx_cardinality(),
    );


    println!("-----------------------------------------");
    let mut graph = original_graph.clone();
    graph = apply_constraints_and_restrict(fixed_point_constraints, graph, "constraint ensured");
    println!("Fixed point constraints alone: {} consistent", graph.unit_colors().approx_cardinality());

    println!("-----------------------------------------");
    let mut graph = original_graph.clone();
    graph = apply_constraints_and_restrict(trap_space_constraints, graph, "constraint ensured");
    println!("Trap space constraints alone: {} consistent", graph.unit_colors().approx_cardinality());

    println!("-----------------------------------------");
    let mut graph = original_graph.clone();
    graph = apply_constraints_and_restrict(reachability_constraints, graph, "constraint ensured");
    println!("Reachability constraints alone: {} consistent", graph.unit_colors().approx_cardinality());

    println!("-----------------------------------------");
    let mut graph = original_graph.clone();
    graph = apply_constraints_and_restrict(negative_reachability_constraints, graph, "constraint ensured");
    println!("Negative reachability constraints alone: {} consistent", graph.unit_colors().approx_cardinality());
}

/// Analysis of the central nervous system (CNS) development
/// This time, formula is created manually and should be more precise
#[allow(dead_code)]
fn case_study_3_manual() {
    let aeon_string = read_to_string("benchmark_models/CNS_development/model.aeon").unwrap();
    let bn = BooleanNetwork::try_from(aeon_string.as_str()).unwrap();
    println!("Loaded model with {} vars.", bn.num_vars());
    let original_graph = SymbolicAsyncGraph::new(bn, 1).unwrap();
    let mut graph = original_graph.clone();
    println!("Model has {} parameters.", graph.symbolic_context().num_parameter_vars());

    // define the observations
    let zero_state = "~Pax6 & ~Hes5 & ~Mash1 & ~Scl & ~Olig2 & ~Stat3 & ~Zic1 & ~Brn2 & ~Tuj1 & ~Myt1L & ~Sox8 & ~Aldh1L1";
    let init_state = "Pax6 & ~Hes5 & ~Mash1 & ~Scl & ~Olig2 & ~Stat3 & ~Zic1 & ~Brn2 & ~Tuj1 & ~Myt1L & ~Sox8 & ~Aldh1L1";
    let t_m = "Pax6 & ~Scl & ~Olig2 & ~Tuj1 & ~Sox8 & ~Aldh1L1";
    let f_t = "Pax6 & Zic1 & Brn2 & Tuj1 & ~Sox8 & ~Aldh1L1";
    let t_o = "Pax6 & ~Scl & Olig2 & ~Tuj1 & ~Sox8 & ~Aldh1L1";
    let f_ms = "Pax6 & ~Zic1 & ~Brn2 & ~Tuj1 & Sox8 & ~Aldh1L1";
    let t_s = "Pax6 & Scl & ~Olig2 & ~Tuj1 & ~Sox8 & ~Aldh1L1";
    let f_a = "Pax6 & ~Zic1 & ~Brn2 & ~Tuj1 & ~Sox8 & Aldh1L1";

    println!(
        "After applying static constraints, {} concretizations remain.",
        graph.mk_unit_colors().approx_cardinality(),
    );

    /*
     Constraints:
     1) existential
     - fixed points: f_a, f_ms
     - trap spaces: f_t
     - reachability: init -> t_m -> f_t, init -> t_o -> f_ms, init -> t_s -> f_a
     - negative reachability: zero -/> f_t, zero -/> f_ms, zero -/> f_a
     2) universal:
     - fixed points must be only from: [f_a, f_ms, f_t, zero]
     - fixed points reachable from 'init' must be only from: [f_a, f_ms, f_t]
     (for the last one, it is enough to just prohibit reaching 'zero' fixed point from 'init'
     */

    let formula = format!(
        "(3{{x}}: (@{{x}}: ({}) & (EF (({}) & EF ({}))) \
                                & (EF (({}) & EF ({}))) \
                                & (EF (({}) & EF ({}))) \
        ))", init_state, t_m, f_t, t_o, f_ms, t_s, f_a);
    graph = apply_constraints_and_restrict(vec![formula.clone()], graph, "non-reachability ensured");
    println!(
        "After the first set of constraints, {} concretizations remain.",
        graph.unit_colors().approx_cardinality(),
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("1 argument expected, got {}", args.len() - 1);
        println!("Usage: ./inference-case-study study_num");
        return;
    }

    let start = SystemTime::now();
    match args[1].as_str() {
        "1" => case_study_1(true),
        "2" => case_study_2(false),
        "3" => case_study_3(),
        _ => {
            println!("Argument study_num must be a  number from 1 to 3, got {}", args.len() - 1);
        }
    }
    println!("Elapsed time: {}ms", start.elapsed().unwrap().as_millis());
}
