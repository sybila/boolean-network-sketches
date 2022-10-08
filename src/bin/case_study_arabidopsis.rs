use bn_inference_tool::attractor_inference::perform_inference_with_attractors_specific;

use clap::Parser;

use std::fs::read_to_string;
use std::time::SystemTime;

/// Structure to collect CLI arguments
#[derive(Parser)]
#[clap(author="Ondrej Huvar", about="Inference case study regarding A. Thaliana.")]
struct Arguments {
    /// Consider only fixed-point attractors (simpler dynamical property)
    #[clap(short, long, takes_value = false)]
    fixed_point_version: bool,
}

/// Analysis of the A. thaliana Sepal Primordium Polarity
/// Infers BNs from sketch including attractor data
fn case_study(fixed_point_version: bool) {
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

/// Run the case study regarding inference of A. Thaliana model
fn main() {
    let args = Arguments::parse();
    let start = SystemTime::now();
    case_study(args.fixed_point_version);
    println!("Elapsed time: {}ms", start.elapsed().unwrap().as_millis());
}
