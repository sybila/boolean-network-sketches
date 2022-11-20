#[allow(unused_imports)]
use crate::create_inference_formulae::{
    mk_attractor_formula_specific, mk_steady_state_formula_specific,
    mk_forbid_other_attractors_formula, mk_forbid_other_steady_states_formula,
};

use biodivine_hctl_model_checker::analysis::model_check_formula_unsafe_ex;

use biodivine_lib_param_bn::symbolic_async_graph::{GraphColors, SymbolicAsyncGraph};

/// Optimised version for inference through the attractor data - after evaluating each dynamical
/// constraint (attractor presence), restricts the colour set only to satisfying colours
/// If `forbid_extra_attr` is true, adds constraint on absence of all additional attractors
/// Works only when attractor states are FULLY SPECIFIED - by all propositions TODO: add check
pub fn perform_inference_with_attractors_specific(
    attr_set: Vec<String>,
    mut graph: SymbolicAsyncGraph,
    use_fixed_points: bool,
    forbid_extra_attr: bool,
) -> GraphColors {
    let mut inferred_colors = graph.mk_unit_colors();
    println!(
        "After applying static constraints, {} concretizations remain.",
        inferred_colors.approx_cardinality(),
    );

    // whole constraint formula is just a conjunction of smaller formulas
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

        inferred_colors = model_check_formula_unsafe_ex(formula, &graph)
            .unwrap()
            .colors();

        // restrict the unit_colored_set in the graph object
        graph = SymbolicAsyncGraph::new_restrict_colors_from_existing(graph, &inferred_colors);
        println!("attractor ensured")
    }
    println!(
        "After ensuring attractor presence, {} concretizations remain.",
        inferred_colors.approx_cardinality(),
    );

    // if desired, we add the constraint which forbids any additional attractors
    // that do not correspond to the observations
    if forbid_extra_attr {
        let formula = if use_fixed_points {
            mk_forbid_other_steady_states_formula(attr_set)
        } else {
            mk_forbid_other_attractors_formula(attr_set)
        };
        inferred_colors = model_check_formula_unsafe_ex(formula, &graph)
            .unwrap()
            .colors();
    }

    inferred_colors
}

#[cfg(test)]
mod tests {
    use crate::inference_attractor_data::perform_inference_with_attractors_specific;
    use crate::utils::check_if_result_contains_goal_unsafe;
    use biodivine_lib_param_bn::symbolic_async_graph::SymbolicAsyncGraph;
    use biodivine_lib_param_bn::BooleanNetwork;
    use std::fs::{File, read_to_string};
    use std::io::{BufRead, BufReader};
    use std::path::Path;

    /// Test BN inference through steady-state data
    /// Test 2 cases (with X without additional attractors)
    fn test_inference_with_steady_state_data(
        observations: Vec<String>,
        model_path: &str,
        two_expected_result_numbers: Vec<f64>,
    ) {
        let aeon_string = read_to_string(model_path).unwrap();
        let bn = BooleanNetwork::try_from(aeon_string.as_str()).unwrap();
        // Create graph object with 1 HCTL var (we dont need more)
        let graph = SymbolicAsyncGraph::new(bn, 1).unwrap();

        let inferred_colors = perform_inference_with_attractors_specific(
            observations.clone(),
            graph.clone(),
            true,
            false,
        );
        assert_eq!(
            inferred_colors.approx_cardinality(),
            two_expected_result_numbers[0]
        );

        let inferred_colors = perform_inference_with_attractors_specific(
            observations.clone(),
            graph.clone(),
            true,
            true,
        );
        assert_eq!(
            inferred_colors.approx_cardinality(),
            two_expected_result_numbers[1]
        );
    }

    #[test]
    /// Test BN inference of arabidopsis model through attractor data
    /// Use data from Griffin tool (or similar pre-computed) to check results
    fn test_case_study_arabidopsis() {
        let observation1 = "AGO1 & ~AGO10 & ~AGO7 & ANT & ARF4 & ~AS1 & ~AS2 & ETT & FIL & KAN1 & miR165 & miR390 & ~REV & ~TAS3siRNA & AGO1_miR165 & ~AGO7_miR390 & ~AS1_AS2 & AUXINh & ~CKh & ~GTE6 & ~IPT5";
        let observation2 = "~AGO1 & AGO10 & AGO7 & ANT & ~ARF4 & AS1 & AS2 & ~ETT & ~FIL & ~KAN1 & ~miR165 & miR390 & REV & TAS3siRNA & ~AGO1_miR165 & AGO7_miR390 & AS1_AS2 & AUXINh & CKh & GTE6 & IPT5";
        let observations = vec![observation1.to_string(), observation2.to_string()];

        test_inference_with_steady_state_data(
            observations,
            "benchmark_models/griffin_2/griffin_model2.aeon",
            vec![439296., 48352.],
        );
    }

    /// Test if inferred colors include the color of goal network
    /// As a test data use concrete model and try to infer it back from its steady-state data and
    /// partially defined model that was created by erasing some concrete model's update functions
    fn check_inferred_colors_contain_goal(
        attractor_data_path: &str,
        model_path: &str,
        goal_model_path: &str,
    ) {
        let aeon_string = read_to_string(model_path).unwrap();
        let bn = BooleanNetwork::try_from(aeon_string.as_str()).unwrap();
        // Create graph object with 1 HCTL var (we dont need more)
        let graph = SymbolicAsyncGraph::new(bn, 1).unwrap();

        let goal_aeon_string = read_to_string(goal_model_path.to_string()).unwrap();

        let data_file = File::open(Path::new(attractor_data_path)).unwrap();
        let reader = BufReader::new(&data_file);
        let observations: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

        let inferred_colors = perform_inference_with_attractors_specific(
            observations.clone(),
            graph.clone(),
            true, // only fixed-points
            true,
        );
        assert!(check_if_result_contains_goal_unsafe(
            graph.clone(),
            goal_aeon_string.clone(),
            inferred_colors
        ));

        let inferred_colors = perform_inference_with_attractors_specific(
            observations.clone(),
            graph.clone(),
            true, // only fixed-points
            false,
        );
        assert!(check_if_result_contains_goal_unsafe(
            graph.clone(),
            goal_aeon_string.clone(),
            inferred_colors
        ));
    }

    #[test]
    /// Test if inferred colors include the color of goal network [model id 110]
    fn test_inferred_colors_contain_goal_110() {
        check_inferred_colors_contain_goal(
            "benchmark_models/110_9v/steady_states.txt",
            "benchmark_models/110_9v/110_parametrized.aeon",
            "benchmark_models/110_9v/110.aeon",
        );
    }

    #[test]
    /// Test if inferred colors include the color of goal network [model id 123]
    fn test_inferred_colors_contain_goal_123() {
        check_inferred_colors_contain_goal(
            "benchmark_models/123_60v/steady_states.txt",
            "benchmark_models/123_60v/model_parametrized.aeon",
            "benchmark_models/123_60v/model.aeon",
        );
    }

    #[test]
    /// Test if inferred colors include the color of goal network [model id 1n33]
    fn test_inferred_colors_contain_goal_133() {
        check_inferred_colors_contain_goal(
            "benchmark_models/133_9v/steady_states.txt",
            "benchmark_models/133_9v/133_parametrized.aeon",
            "benchmark_models/133_9v/133.aeon",
        );
    }
}
