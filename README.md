# Boolean Network Sketches: A Unifying Framework for Logical Model Inference

This is a small Rust package focusing on logical model inference through Boolean network sketches. 
It contains the high-level implementation of the framework, as well as benchmark models and data used for experiments and case studies. 
This readme then describes the repository contents and also contains instructions on how to replicate the main experimental results of the paper.

## Boolean network sketches

A Boolean network sketch represents a combination of prior knowledge, data, and hypotheses regarding the modelled system.
Sketch is given by following components: 
- influence graph
- partially specified Boolean network 
- static properties of update functions
- dynamic properties of the system

Given a sketch, we can compute all consistent candidate Boolean networks. 
We can analyze their attractors symbolically, get witness networks, or analyze differences in a candidate set.

## Installing Dependencies

To run the case studies and experiments, you will need the Rust compiler. 
We recommend following the instructions on [rustlang.org](https://www.rust-lang.org/learn/get-started) (default configuration should be sufficient) instead of using a package manager, however either method should work ok. 
When using the official installer, everything is stored in `~/.cargo`, so admin privilages are not necessary. 
Once you are done, you can uninstall the compiler by running `rustup self uninstall`. 
The tested version of Rust is `1.64.0` (Sep 2022).


## Sketch format

The first three components of the sketch are all covered by the AEON model format. This format describes:
- influences between variables
- types of influences (such as monotonicity or essentiality), from which we can derive properties of update functions
- partially specified update functions 

The dynamic properties are given in a form of a HCTL formula (syntax is defined in [this repository](https://github.com/sybila/biodivine-hctl-model-checker)). Some formulae can be generated from data.

## Benchmark models and pre-computed results

All the models used for the evaluation (and some more) are in `benchmark_models` folder. 
Each model has its own sub-folder.

Models used for case studies are in sub-folders `case_study_arabidopsis` and `case_study_TLGL`.
These sub-folders contain all the model variants in aeon format, metadata, encoded attractor states, raw outputs from corresponding binaries and summarization of resulting candidates.

The running example from the paper is in `small_example`. There is the starting model, the formulae, and results (including the single consistent network).

The sub-folders with models used for scalability evaluation contain four files each:
- `model_concrete.aeon` - a fully specified model with which we started
- `model_parametrized.aeon` - parametrized version of the same model used in sketch
- `attractor_states.txt` - collection of encoded synthetic attractor data
- `results.txt` - resulting output from the corresponding binary

## Binaries and scripts for experiments

You can either use the prepared Bash script to re-run all the experiments at once, or run them individually using compiled binaries.

### All-encompassing script

The following Bash script runs all the experiments, one after another. 
```
./run_benchmarks_scalability_unix.sh
```

The script prints the results and all the relevant progress on the standard output. The experiments are run in the following order:
- the first case study (two versions of the sketch)
- the second case study (two versions of the sketch)
- scalability benchmarks (several sketches, one after another, from the smallest)

The script should work on classical Unix-based systems, and we have also tested it on Windows Subsystem for Linux (WSL).

### Individual binaries

To directly re-run individual experiments or case studies, compile the code first:
```
cargo build --release
```

Code regarding the two case studies from the paper can be found in `src/bin` folder (particularly, `case_study_arabidopsis.rs` and `case_study_tlgl.rs`). 
The resulting binaries will be in `target/release` folder and can be run as follows. 
Note that on Windows, the path is usually `target\release` and binaries have `.exe` suffix. 
```
./target/release/case-study-tlgl [--refined-sketch] 
./target/release/case-study-arabidopsis [--fixed-points] [--prohibit-extra-attrs]
```

To see other existing options for each of these case studies, run 
```
./target/release/<CASE-STUDY-NAME> -h
```

To run the experiments regarding scalability, use the following binary.
It is a general method for inference using network sketches with attractor data, so you can choose arbitrary model. 

````
./target/release/inference-with-attractors [OPTIONS] <MODEL_PATH> <ATTRACTOR_DATA_PATH>
````
- `MODEL_PATH` is a path to the file with BN model in aeon format
- `ATTRACTOR_DATA_PATH` is a path to the file with attractor data (one encoded state per line)
- most important `OPTIONS` are
  - `--prohibit-extra-attrs`: prohibit all additional attractors not containing any specified states
  - `--fixed-points`: reason with fixed-point attractors only (instead of general attractors)
  - `--goal-model <GOAL_MODEL>`: a concrete BN model to look for in the resulting set of candidates


## Availibility and Reusability

The implementation is based mainly on two of our Rust libraries: 
- [biodivine-hctl-model-checker](https://github.com/sybila/biodivine-hctl-model-checker) for the underlying symbolic coloured model checking
- [biodivine-lib-param-bn](https://github.com/sybila/biodivine-lib-param-bn) which facilitate the symbolic encoding of Boolean networks

The implementation itself is fairly minimal (includes i.e., high-level framework or encoding of dynamic properties) and contains basic comments that explain the function of individual components.

Everything is open-source and available with the permissive MIT License.

To visualize and modify the BN models (aeon files) included in this repository, you can use the online interface of our tool [AEON](https://biodivine.fi.muni.cz/aeon). This tool can also be used to further analyse the attractors of Boolean network models.



## Tests 
To run the set of tests, use `cargo test` command.