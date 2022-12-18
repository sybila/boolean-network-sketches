use biodivine_hctl_model_checker::analysis::{get_extended_symbolic_graph, model_check_formula};

use biodivine_lib_param_bn::BooleanNetwork;

use std::fs::read_to_string;
use std::time::SystemTime;

fn main() {
    let start = SystemTime::now();

    let model_name = ".\\benchmark_models\\small_example\\model-small-example.aeon";
    let prior_formula = "3{a}: (3{b}: (3{c}: (@{c}: ((EF {a}) & (EF {b}) & (@{a}: AG EF {a}) & (@{b}: (AG EF {b} & ~ EF {a}))))))";
    let whole_formula = "3{a}: (3{b}: (3{c}: (@{c}: ((EF {a}) & (EF {b}) & (@{a}: AG EF {a}) & (@{b}: (AG EF {b} & ~ EF {a})))))) & (3{x}:@{x}: ~v_1 & ~v_2 & v_3 & AG EF {x}) & (3{y}:@{y}: v_1 & v_2 & ~v_3 & AG EF {y})";

    let aeon_string = read_to_string(model_name).unwrap();
    let bn = BooleanNetwork::try_from(aeon_string.as_str()).unwrap();
    println!("Loaded model with {} vars.", bn.num_vars());

    let graph = get_extended_symbolic_graph(&bn, 3);
    println!(
        "Model has {} parameters.",
        graph.symbolic_context().num_parameter_variables()
    );

    println!(
        "After applying static constraints, {} concretizations remain.",
        graph.mk_unit_colors().approx_cardinality(),
    );

    let intermediate_result = model_check_formula(prior_formula.to_string(), &graph).unwrap();
    println!(
        "After applying prior-knowledge-based dynamic constraints, {} concretizations remain.",
        intermediate_result.colors().approx_cardinality(),
    );

    let result = model_check_formula(whole_formula.to_string(), &graph).unwrap();
    let res_color = result.colors();

    println!(
        "After applying all dynamic constraints, {} concretizations remain.",
        res_color.approx_cardinality(),
    );

    println!("Elapsed time: {}ms", start.elapsed().unwrap().as_millis());
    println!("----------");

    let witness_bn = graph.pick_witness(&res_color);
    println!("WITNESS:");
    println!("{}", witness_bn.to_string());
}
