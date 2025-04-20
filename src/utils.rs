//! Contains several useful utilities for either the inference procedure or for post-processing
//! the results.

use biodivine_hctl_model_checker::model_checking::{
    model_check_formula_dirty, model_check_tree_dirty,
};

use biodivine_lib_param_bn::biodivine_std::traits::Set;
use biodivine_lib_param_bn::symbolic_async_graph::{GraphColors, SymbolicAsyncGraph};
use biodivine_lib_param_bn::{BooleanNetwork, FnUpdate};

use biodivine_hctl_model_checker::preprocessing::hctl_tree::HctlTreeNode;
use biodivine_lib_bdd::BddPartialValuation;
use rand::rngs::ThreadRng;
use std::collections::HashMap;

/// Apply properties (constraints) given by HCTL `formulae` on the graph's colors.
/// Returns a graph with colour space restricted only to the suitable colors.
pub fn apply_constraints_and_restrict(
    formulae: Vec<String>,
    mut graph: SymbolicAsyncGraph,
    message: &str,
) -> SymbolicAsyncGraph {
    for formula in formulae {
        let inferred_colors = model_check_formula_dirty(&formula, &graph)
            .unwrap()
            .colors();
        graph = SymbolicAsyncGraph::with_custom_context(
            graph.as_network().unwrap(),
            graph.symbolic_context().clone(),
            inferred_colors.as_bdd().clone(),
        )
        .unwrap();

        if !message.is_empty() {
            println!("{message}")
        }
    }
    graph
}

/// Apply properties (constraints) given by HCTL formulae `trees` on the graph's colors.
/// Returns a graph with colour space restricted only to the suitable colors.
pub fn apply_constraint_trees_and_restrict(
    formulae_trees: Vec<HctlTreeNode>,
    mut graph: SymbolicAsyncGraph,
    message: &str,
) -> SymbolicAsyncGraph {
    for formula_tree in formulae_trees {
        // do it one by one now to track progress, even though it could be done at once
        let inferred_colors = model_check_tree_dirty(formula_tree, &graph)
            .unwrap()
            .colors();
        graph = SymbolicAsyncGraph::with_custom_context(
            graph.as_network().unwrap(),
            graph.symbolic_context().clone(),
            inferred_colors.as_bdd().clone(),
        )
        .unwrap();

        if !message.is_empty() {
            println!("{message}")
        }
    }
    graph
}

/// Check if `inferred_colors` contain the color of the specific network
/// represented by `goal_aeon_string`.
pub fn check_if_result_contains_goal(
    graph: SymbolicAsyncGraph,
    goal_aeon_string: Option<String>,
    inferred_colors: GraphColors,
) {
    // if the goal network was supplied, check whether it is part of the solution set
    if let Some(goal_model) = goal_aeon_string {
        let goal_bn = BooleanNetwork::try_from(goal_model.as_str()).unwrap();
        match graph.mk_subnetwork_colors(&goal_bn) {
            Ok(goal_colors) => {
                // we will need intersection of goal colors with the ones from the result
                // if the goal is subset of result, it is ok
                if goal_colors.intersect(&inferred_colors).approx_cardinality()
                    == goal_colors.approx_cardinality()
                {
                    println!("OK - goal network is included in the candidate set.")
                } else {
                    println!("NOK - goal network is NOT included in the candidate set.")
                }
            }
            Err(e) => println!("{e}"),
        }
    } else {
        println!("Goal network not provided.")
    }
}

/// Check if the resulting set `inferred_colors` contain the color of the specific network
/// represented by `goal_aeon_string`.
///
/// This is an unsafe version of function `check_if_result_contains_goal`, - there are no input
/// validity checks, just returns a value (and also no prints).
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

/// Randomly select a color from the given set of colors.
/// This is a workaround that should be modified in future.
pub fn pick_random_color(
    rng: &mut ThreadRng,
    graph: &SymbolicAsyncGraph,
    color_set: &GraphColors,
) -> GraphColors {
    let random_witness = color_set.as_bdd().random_valuation(rng).unwrap();

    let bdd_vars = graph.symbolic_context().bdd_variable_set();
    let mut partial_valuation = BddPartialValuation::empty();
    for var in bdd_vars.variables() {
        if !graph
            .symbolic_context()
            .parameter_variables()
            .contains(&var)
        {
            // Only "copy" the values of parameter variables. The rest should be irrelevant.
            continue;
        }
        partial_valuation.set_value(var, random_witness.value(var));
    }
    let singleton_bdd = bdd_vars.mk_conjunctive_clause(&partial_valuation);
    // We can directly build a `GraphColors` object because we only copied the parameter
    // variables from the random valuation (although the `pick_witness` method shouldn't
    // really care about extra variables in the BDD at all).
    let singleton_set = graph.unit_colors().copy(singleton_bdd);
    singleton_set
}

/// Naively go through all candidates given by their `colors` and summarize their update fns.
/// For each variable, compute how many variants of its update function are present between the
/// candidates.
///
/// Note that this could be done symbolically to scale significantly better if needed.
pub fn summarize_candidates_naively(
    graph: &SymbolicAsyncGraph,
    mut colors: GraphColors,
    print_exact_fns: bool,
) {
    // prepare the map for capturing update fn variants <VarName: <UpdateFn: Count>>
    let mut update_fns: HashMap<String, HashMap<FnUpdate, i32>> = HashMap::new();
    for v in graph.as_network().unwrap().variables() {
        update_fns.insert(
            graph.as_network().unwrap().get_variable_name(v).clone(),
            HashMap::new(),
        );
    }

    // iterate through candidates (colors)
    while !colors.is_empty() {
        // progress print after each 100 000 candidates searched
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

    // update_fns is structured as Map<VarName : Map<UpdateFn : Count>>
    for (var_name, fn_map) in update_fns {
        print!("{} [{}]:  ", var_name, fn_map.len());
        if fn_map.len() == 1 {
            vars_with_unique_fns.push(var_name);
        } else {
            vars_with_variable_fns.push(var_name);
        }

        for (update_fn, num) in fn_map {
            if print_exact_fns {
                print!(
                    "\"{}\" ${num}$  ",
                    update_fn.to_string(graph.as_network().unwrap())
                );
            } else {
                print!("{num} ");
            }
        }
        println!();
    }

    vars_with_variable_fns.sort();
    vars_with_unique_fns.sort();

    println!();
    println!(
        "{} variables with more than 1 possible update fns: {:?}",
        vars_with_variable_fns.len(),
        vars_with_variable_fns
    );
    println!(
        "{} variables with only single possible update fn: {:?}",
        vars_with_unique_fns.len(),
        vars_with_unique_fns
    );
}

#[cfg(test)]
mod tests {
    use crate::utils::pick_random_color;
    use biodivine_hctl_model_checker::mc_utils::get_extended_symbolic_graph;
    use biodivine_lib_param_bn::BooleanNetwork;

    const TEST_MODEL: &str = r"
v_3 -| v_1
v_1 -?? v_2
$v_3:f(v_1, v_2) | v_3
v_2 -? v_3
v_1 -> v_3
v_3 -?? v_3
";

    #[test]
    /// Test basic properties of random color selection.
    fn test_pick_color() {
        let mut rng = rand::thread_rng();
        let bn = BooleanNetwork::try_from(TEST_MODEL).unwrap();
        let stg = get_extended_symbolic_graph(&bn, 1).unwrap();
        let color_set = stg.mk_unit_colors();

        let singleton_set = pick_random_color(&mut rng, &stg, &color_set);
        assert_eq!(singleton_set.approx_cardinality(), 1.0); // only one color is selected
    }
}
