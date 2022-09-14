use biodivine_lib_param_bn::symbolic_async_graph::SymbolicAsyncGraph;
use biodivine_lib_param_bn::BooleanNetwork;

use hctl_model_checker::analysis::model_check_formula_unsafe;
use hctl_model_checker::inference::utils::check_if_result_contains_goal;

#[allow(unused_imports)]
use crate::inference_formulae::*;

use std::convert::TryFrom;

/// Optimised version - first evaluates formula for specific attractor existence, then (if we want
/// forbid all additional attractors) evaluates the formula for the attractor prohibition, this
/// time only on graph with colors restricted to those from the first part
/// Works only when attractor states are FULLY SPECIFIED - by all propositions TODO: add check
/// If `goal_model` is not none, check whether its colors are included in the resulting set of colors
pub fn perform_inference_with_attractors_specific(
    attr_set: Vec<String>,
    aeon_string: String,
    use_fixed_points: bool,
    forbid_extra_attr: bool,
    goal_aeon_string: Option<String>,
) {
    let bn = BooleanNetwork::try_from(aeon_string.as_str()).unwrap();
    println!("Loaded model with {} vars.", bn.num_vars());

    // To be sure, create graph object with 1 HCTL var
    let mut graph = SymbolicAsyncGraph::new(bn, 1).unwrap();
    println!("Model has {} parameters.", graph.symbolic_context().num_parameter_vars());

    let mut inferred_colors = graph.mk_unit_colors();
    println!(
        "After applying static constraints, {} concretizations remain.",
        inferred_colors.approx_cardinality(),
    );

    // whole formula we want to eval is just a conjunction of smaller formulas
    // "exists attractor_1" & ... & "exists attractor_n" & "NOT exists any other attractor"
    // we will evaluate each conjunct only on colors where previous conjuncts hold

    // first we evaluate the parts that ensure attractor(s) existence
    for attractor_state in attr_set.clone() {
        if attractor_state.is_empty() {
            continue;
        }
        let formula = if use_fixed_points {
            mk_steady_state_formula_specific(attractor_state)
        } else {
            mk_attractor_formula_specific(attractor_state)
        };
        inferred_colors = model_check_formula_unsafe(formula, &graph).colors();
        // we now restrict the unit_colored_set in the graph object
        graph = SymbolicAsyncGraph::new_restrict_colors_from_existing(graph, &inferred_colors);
        println!("attractor ensured")
    }
    println!(
        "After ensuring attractor presence, {} concretizations remain.",
        inferred_colors.approx_cardinality(),
    );

    // if desired, we will add the formula which forbids additional attractors
    if forbid_extra_attr {
        let formula = if use_fixed_points {
            mk_forbid_other_steady_states_formula(attr_set)
        } else {
            mk_forbid_other_attractors_formula(attr_set)
        };
        inferred_colors = model_check_formula_unsafe(formula, &graph).colors();
    }

    println!(
        "{} suitable networks found in total",
        inferred_colors.approx_cardinality()
    );

    // if the goal network was supplied, lets check whether it is part of the solution set
    check_if_result_contains_goal(graph, goal_aeon_string, inferred_colors);
}
