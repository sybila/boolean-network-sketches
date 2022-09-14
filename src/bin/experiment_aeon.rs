use biodivine_aeon_server::scc::algo_interleaved_transition_guided_reduction::interleaved_transition_guided_reduction;
use biodivine_aeon_server::scc::algo_xie_beerel::xie_beerel_attractors;
use biodivine_aeon_server::scc::Classifier;
use biodivine_aeon_server::GraphTaskContext;
use biodivine_lib_param_bn::symbolic_async_graph::SymbolicAsyncGraph;
use biodivine_lib_param_bn::{BooleanNetwork, FnUpdate, VariableId};

use rand::seq::SliceRandom;
use rand::thread_rng;

use std::fs::{read_to_string, File};
use std::io::Write;
use std::time::SystemTime;

const MODEL_ID: &str = "001";

/// Compute the network input variables.
fn network_inputs(network: &BooleanNetwork) -> Vec<VariableId> {
    network
        .variables()
        .filter(|v| network.regulators(*v).is_empty())
        .collect()
}

/// Create a copy of the given network with all input variables fixed to given constants.
fn fix_network_inputs(network: &BooleanNetwork, bool_values: Vec<bool>) -> BooleanNetwork {
    let mut result = network.clone();
    let mut i = 0;
    for v in network_inputs(network) {
        result
            .set_update_function(v, Some(FnUpdate::Const(bool_values[i])))
            .unwrap();
        i += 1;
    }
    result
}

/// Returns binary vector incremented by 1
fn next_bool_val(mut bool_vec: Vec<bool>) -> Result<Vec<bool>, String> {
    let mut i = 0;
    while i < bool_vec.len() {
        if bool_vec[i] {
            bool_vec[i] = false;
        } else {
            bool_vec[i] = true;
            return Ok(bool_vec);
        }
        i += 1;
    }
    return Err("finished".to_string());
}

/// Converts vector of bools to the corresponding binary string
#[allow(dead_code)]
fn bool_vec_to_string(bool_vec: Vec<bool>) -> String {
    bool_vec.into_iter().fold("".to_string(), |mut s, b| {
        if b {
            s.push_str("1");
            s
        } else {
            s.push_str("0");
            s
        }
    })
}

#[allow(dead_code)]
fn export_network(input_values: Vec<bool>, network: &BooleanNetwork) {
    let input_val_string = bool_vec_to_string(input_values);
    let bnet_file = format!(
        "experiment\\{}\\fixed_networks\\{}.bnet",
        MODEL_ID, input_val_string
    );
    std::fs::write(bnet_file, network.to_bnet(false).unwrap()).unwrap();

    /*
    let aeon_file = format!(
        "experiment\\{}\\fixed_networks\\{}.aeon",
        MODEL_ID, input_val_string
    );
    std::fs::write(aeon_file, network.to_string()).unwrap();
     */
}

#[allow(dead_code)]
fn int_to_bool_vec(mut decimal: i32, inputs_num: i32) -> Vec<bool> {
    let mut bits: Vec<bool> = Vec::new();

    while decimal > 0 {
        bits.push(decimal % 2 == 1);
        decimal /= 2;
    }
    let mut missing_bits = inputs_num - i32::try_from(bits.len()).unwrap();
    while missing_bits > 0 {
        bits.push(false);
        missing_bits -= 1;
    }
    bits
}

#[allow(dead_code)]
fn analyze_random_instantiations() {
    let attractor_file = File::create(format!("experiment\\{}\\attractors.txt", MODEL_ID)).unwrap();
    let times_file = File::create(format!("experiment\\{}\\aeon_results.txt", MODEL_ID)).unwrap();

    let aeon_string =
        read_to_string(format!("experiment\\{}\\{}.aeon", MODEL_ID, MODEL_ID)).unwrap();
    let network = BooleanNetwork::try_from(aeon_string.as_str()).unwrap();

    let inputs_num = network_inputs(&network).len();

    let mut values: Vec<i32> = (0..2_i32.pow(u32::try_from(inputs_num).unwrap())).collect();
    values.shuffle(&mut thread_rng());

    let values_file =
        File::create(format!("experiment\\{}\\chosen_instantiations.txt", MODEL_ID)).unwrap();
    for val in values.clone().iter().take(10000) {
        writeln!(&values_file, "{}", val).unwrap();
    }

    for val in values.clone().iter().take(10000) {
        let input_values = int_to_bool_vec(*val, i32::try_from(inputs_num).unwrap());

        let fixed_network = fix_network_inputs(&network, input_values.clone());
        //export_network(input_values.clone(), &fixed_network);

        let start = SystemTime::now();

        let graph = SymbolicAsyncGraph::new(fixed_network, 0).unwrap();
        let classifier = Classifier::new(&graph);
        let task_context = GraphTaskContext::new();
        task_context.restart(&graph);

        // First, perform ITGR reduction.
        let (universe, active_variables) = interleaved_transition_guided_reduction(
            &task_context,
            &graph,
            graph.mk_unit_colored_vertices(),
        );

        // Then run Xie-Beerel to actually detect the components.
        xie_beerel_attractors(
            &task_context,
            &graph,
            &universe,
            &active_variables,
            |component| {
                // println!("Component {}", component.approx_cardinality());
                classifier.add_component(component, &graph);
            },
        );
        let components = classifier.export_components();
        let time_elapsed = start.elapsed().unwrap().as_millis();
        writeln!(&times_file, "{}", time_elapsed).unwrap();
        for (comp, _) in components {
            write!(&attractor_file, "{}, ", comp.vertices().approx_cardinality()).unwrap();
        }
        write!(&attractor_file, "\n").unwrap();
    }
}

#[allow(dead_code)]
fn main2() {
    analyze_random_instantiations();
}

fn main() {
    let attractor_file = File::create(format!("experiment\\{}\\attractors.txt", MODEL_ID)).unwrap();
    let times_file = File::create(format!("experiment\\{}\\aeon_results.txt", MODEL_ID)).unwrap();

    let aeon_string =
        read_to_string(format!("experiment\\{}\\{}.aeon", MODEL_ID, MODEL_ID)).unwrap();
    let network = BooleanNetwork::try_from(aeon_string.as_str()).unwrap();

    let input_nums = network_inputs(&network).len();
    let mut input_values = Vec::with_capacity(input_nums);
    input_values.resize(input_nums, false);

    // TODO: do not skip 00..00 vector for the loop
    while let Ok(next_input_values) = next_bool_val(input_values) {
        input_values = next_input_values.clone();
        let fixed_network = fix_network_inputs(&network, input_values.clone());
        //export_network(input_values.clone(), &fixed_network);

        let start = SystemTime::now();

        let graph = SymbolicAsyncGraph::new(fixed_network, 0).unwrap();
        let classifier = Classifier::new(&graph);
        let task_context = GraphTaskContext::new();
        task_context.restart(&graph);

        // First, perform ITGR reduction.
        let (universe, active_variables) = interleaved_transition_guided_reduction(
            &task_context,
            &graph,
            graph.mk_unit_colored_vertices(),
        );

        // Then run Xie-Beerel to actually detect the components.
        xie_beerel_attractors(
            &task_context,
            &graph,
            &universe,
            &active_variables,
            |component| {
                // println!("Component {}", component.approx_cardinality());
                classifier.add_component(component, &graph);
            },
        );
        let components = classifier.export_components();
        let time_elapsed = start.elapsed().unwrap().as_millis();
        writeln!(&times_file, "{}", time_elapsed).unwrap();
        for (comp, _) in components {
            write!(&attractor_file, "{}, ", comp.vertices().approx_cardinality()).unwrap();
        }
        write!(&attractor_file, "\n").unwrap();
    }
}
