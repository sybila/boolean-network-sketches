use biodivine_hctl_model_checker::analysis::model_check_formula;

use biodivine_lib_param_bn::biodivine_std::traits::Set;
use biodivine_lib_param_bn::symbolic_async_graph::{GraphColors, SymbolicAsyncGraph};
use biodivine_lib_param_bn::{BooleanNetwork, FnUpdate};

use std::collections::HashMap;

/// Applies constraints given by HCTL `formulae` on the graph's colors
/// Returns graph with colour space restricted only to the suitable colors
pub fn apply_constraints_and_restrict(
    formulae: Vec<String>,
    mut graph: SymbolicAsyncGraph,
    message: &str,
) -> SymbolicAsyncGraph {
    for formula in formulae {
        let inferred_colors = model_check_formula(formula, &graph).unwrap().colors();
        graph = SymbolicAsyncGraph::with_custom_context(
            graph.as_network().clone(),
            graph.symbolic_context().clone(),
            inferred_colors.as_bdd().clone(),
        )
        .unwrap();

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
                    println!("OK - goal network is included in the candidate set.")
                } else {
                    println!("NOK - goal network is NOT included in the candidate set.")
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
        Ok(goal_colors) => {
            goal_colors.intersect(&inferred_colors).approx_cardinality()
                == goal_colors.approx_cardinality()
        }
        Err(_) => false,
    }
}

pub fn summarize_candidates_naively(graph: &SymbolicAsyncGraph, mut colors: GraphColors) {
    // prepare the map for capturing update fn variants <VarName: <UpdateFn: Count>>
    let mut update_fns: HashMap<String, HashMap<FnUpdate, i32>> = HashMap::new();
    for v in graph.as_network().variables() {
        update_fns.insert(
            graph.as_network().get_variable_name(v).clone(),
            HashMap::new(),
        );
    }

    // iterate through candidates (colors) - that cant be done directly for now, so its hacked
    // for a bit and save different possible update fns
    while !colors.is_empty() {
        if colors.approx_cardinality().round() % 100000. < 0.0000001 {
            println!("{}", colors.approx_cardinality().round())
        }
        let c = colors.pick_singleton();
        let bn = graph.pick_witness(&c);

        for v in bn.variables() {
            let var_name = bn.get_variable_name(v).as_str();
            // let update_str = format!("{:?}", bn.get_update_function(v).clone().unwrap());
            let update_fn = bn.get_update_function(v).clone().unwrap();
            if update_fns[var_name].contains_key(&update_fn) {
                *update_fns
                    .get_mut(var_name)
                    .unwrap()
                    .get_mut(&update_fn)
                    .unwrap() += 1;
            } else {
                update_fns
                    .get_mut(var_name)
                    .unwrap()
                    .insert(update_fn.clone(), 1);
            }
        }
        colors = colors.minus(&c);
    }

    // for each var, print all possible update fns and their count
    let mut vars_with_unique_fns = vec![];
    let mut vars_with_variable_fns = vec![];

    for (var_name, fn_map) in update_fns {
        print!("{} [{}]:  ", var_name, fn_map.len());
        if fn_map.len() == 1 {
            vars_with_unique_fns.push(var_name);
        } else {
            vars_with_variable_fns.push(var_name);
        }

        for (_, num) in fn_map {
            print!("{} ", num);
        }
        println!();
    }

    vars_with_variable_fns.sort();
    vars_with_unique_fns.sort();

    println!();
    println!(
        "{} variables with different possible update fns: {:?}",
        vars_with_variable_fns.len(),
        vars_with_variable_fns
    );
    println!(
        "{} variables with only one possible update fn: {:?}",
        vars_with_unique_fns.len(),
        vars_with_unique_fns
    );
}
