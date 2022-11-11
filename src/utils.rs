use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

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


/// Checks if `inferred_colors` contain the color of the specific network
/// represented by `goal_aeon_string`
/// Similar to function above, but no prints, no input checks, just return value
pub fn check_if_result_contains_goal_unsafe(
    graph: SymbolicAsyncGraph,
    goal_aeon_string: String,
    inferred_colors: GraphColors,
) -> bool {
    let goal_bn = BooleanNetwork::try_from(goal_aeon_string.as_str()).unwrap();
    match graph.mk_subnetwork_colors(&goal_bn) {
        Ok(goal_colors) =>
            goal_colors.intersect(&inferred_colors).approx_cardinality()
                == goal_colors.approx_cardinality(),
        Err(_) => false
    }
}

#[allow(dead_code)]
pub fn enumerate_candidates_naively(graph: &SymbolicAsyncGraph, mut colors: GraphColors) {
    // open the file for outputs
    let mut output_file = File::create("file.txt").unwrap();

    let mut update_fns: HashMap<String, HashMap<String, i32>> = HashMap::new();
    for v in graph.as_network().variables() {
        update_fns.insert(graph.as_network().get_variable_name(v).clone(), HashMap::new());
    }

    while !colors.is_empty() {
        let c = colors.pick_singleton();
        let bn = graph.pick_witness(&c);

        for v in bn.variables() {
            let var_name = bn.get_variable_name(v).as_str();
            let update_str = format!("{:?}", bn.get_update_function(v).clone().unwrap());
            if update_fns[var_name].contains_key(update_str.as_str()) {
                *update_fns.get_mut(var_name).unwrap().get_mut(update_str.as_str()).unwrap() += 1;
            } else {
                update_fns.get_mut(var_name).unwrap().insert(update_str.clone(), 1);
            }
        }

        write!(output_file, "{}\n", bn.to_bnet(false).unwrap()).unwrap();
        colors = colors.minus(&c);
    }
    for (var_name, fn_map) in update_fns {
        print!("{}:", var_name);
        for (_, num) in fn_map {
            print!("{} ", num);
        }
        println!()
    }
}
