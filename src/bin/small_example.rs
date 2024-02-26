use biodivine_hctl_model_checker::mc_utils::get_extended_symbolic_graph;
use biodivine_hctl_model_checker::model_checking::model_check_formula;

use biodivine_lib_param_bn::BooleanNetwork;

use std::fs::read_to_string;
use std::time::SystemTime;

/// Run the computation regarding the small example from the paper.
fn main() {
    let start = SystemTime::now();

    let model_name = "benchmark_models/small_example/model-small-example.aeon";

    let aeon_string = read_to_string(model_name).unwrap();
    let bn = BooleanNetwork::try_from(aeon_string.as_str()).unwrap();
    println!("Loaded model with {} vars.", bn.num_vars());

    let graph = get_extended_symbolic_graph(&bn, 3).unwrap();
    println!(
        "Model has {} symbolic parameters.",
        graph.symbolic_context().num_parameter_variables()
    );

    println!(
        "After applying update function properties, {} candidates remain.",
        graph.mk_unit_colors().approx_cardinality(),
    );

    // prior-knowledge dynamic properties only
    let prior_formula = "3{a}: (3{b}: (3{c}: (@{c}: ((EF {a}) & (EF {b}) & (@{a}: AG EF {a}) & (@{b}: (AG EF {b} & ~ EF {a}))))))";
    let intermediate_result = model_check_formula(prior_formula, &graph).unwrap();
    println!(
        "After applying prior-knowledge-based dynamic constraints, {} candidates remain.",
        intermediate_result.colors().approx_cardinality(),
    );

    // properties regarding both prior knowledge and data
    let whole_formula = "3{a}: (3{b}: (3{c}: (@{c}: ((EF {a}) & (EF {b}) & (@{a}: AG EF {a}) & (@{b}: (AG EF {b} & ~ EF {a})))))) & (3{x}:@{x}: ~v_1 & ~v_2 & v_3 & AG EF {x}) & (3{y}:@{y}: v_1 & v_2 & ~v_3 & AG EF {y})";
    let result = model_check_formula(whole_formula, &graph).unwrap();
    let res_color = result.colors();

    println!(
        "After applying all dynamic constraints, {} candidates remain.",
        res_color.approx_cardinality(),
    );

    println!(
        "Elapsed time from the start of this computation: {}ms",
        start.elapsed().unwrap().as_millis()
    );
    println!("-------");

    let witness_bn = graph.pick_witness(&res_color);
    println!("RESULTING NETWORK:");
    println!("{witness_bn}");
}

#[cfg(test)]
mod tests {
    use biodivine_hctl_model_checker::mc_utils::get_extended_symbolic_graph;
    use biodivine_hctl_model_checker::model_checking::model_check_formula;

    use biodivine_lib_param_bn::BooleanNetwork;

    use std::fs::read_to_string;

    #[test]
    /// Test BN inference regarding the small example from the paper.
    /// Use results computed by enumeration to check the results.
    fn test_small_example() {
        let model_name = "benchmark_models/small_example/model-small-example.aeon";
        let aeon_string = read_to_string(model_name).unwrap();

        let bn = BooleanNetwork::try_from(aeon_string.as_str()).unwrap();
        let graph = get_extended_symbolic_graph(&bn, 3).unwrap();

        assert_eq!(graph.symbolic_context().num_parameter_variables(), 8);

        assert_eq!(graph.mk_unit_colors().approx_cardinality(), 16.);

        let prior_formula = "3{a}: (3{b}: (3{c}: (@{c}: ((EF {a}) & (EF {b}) & (@{a}: AG EF {a}) & (@{b}: (AG EF {b} & ~ EF {a}))))))";
        let intermediate_result = model_check_formula(prior_formula, &graph).unwrap();
        assert_eq!(intermediate_result.colors().approx_cardinality(), 4.);

        let whole_formula = "3{a}: (3{b}: (3{c}: (@{c}: ((EF {a}) & (EF {b}) & (@{a}: AG EF {a}) & (@{b}: (AG EF {b} & ~ EF {a})))))) & (3{x}:@{x}: ~v_1 & ~v_2 & v_3 & AG EF {x}) & (3{y}:@{y}: v_1 & v_2 & ~v_3 & AG EF {y})";
        let result = model_check_formula(whole_formula, &graph).unwrap();
        assert_eq!(result.colors().approx_cardinality(), 1.);
    }
}
