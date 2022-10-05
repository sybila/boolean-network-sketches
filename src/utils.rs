use biodivine_lib_param_bn::biodivine_std::traits::Set;
use biodivine_lib_param_bn::symbolic_async_graph::{GraphColors, SymbolicAsyncGraph};
use biodivine_lib_param_bn::BooleanNetwork;

use biodivine_hctl_model_checker::analysis::model_check_formula_unsafe;

/// Applies constraints given by HCTL `formulae` on the graph's colors
/// Returns graph with colour space restricted only to the suitable colors
pub fn apply_constraints_and_restrict(
    formulae: Vec<String>,
    mut graph: SymbolicAsyncGraph,
    message: &str,
) -> SymbolicAsyncGraph {
    for formula in formulae {
        let inferred_colors = model_check_formula_unsafe(formula, &graph).colors();
        graph = SymbolicAsyncGraph::new_restrict_colors_from_existing(graph, &inferred_colors);
        if !message.is_empty() {
            println!("{}", message)
        }
    }
    graph
}

/// checks if `inferred_colors` contain the color of the specific network
/// represented by `goal_aeon_string`
pub fn check_if_result_contains_goal(
    graph: SymbolicAsyncGraph,
    goal_aeon_string: Option<String>,
    inferred_colors: GraphColors,
) {
    // if the goal network was supplied, lets check whether it is part of the solution set
    if let Some(goal_model) = goal_aeon_string {
        let goal_bn = BooleanNetwork::try_from(goal_model.as_str()).unwrap();
        match graph.mk_subnetwork_colors(&goal_bn) {
            Ok(goal_colors) => {
                // we will need intersection of goal colors with the ones from the result
                // if the goal is subset of result, it went well
                if goal_colors.intersect(&inferred_colors).approx_cardinality()
                    == goal_colors.approx_cardinality()
                {
                    println!("OK - color of goal network is included in resulting set.")
                } else {
                    println!("NOK - color of goal network is NOT included in resulting set.")
                }
            }
            Err(e) => println!("{}", e),
        }
    } else {
        println!("Goal network not provided.")
    }
}
