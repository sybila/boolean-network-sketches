# Boolean Network Sketches: A Unifying Framework for Logical Model Inference

This is a small Rust library focusing on logical model inference through Boolean network sketches. 
It contains the high-level implementation of the framework, as well as benchmark models and data used for experiments and case studies. 
This readme describes the repository contents and also contains instructions on how to replicate the main experimental results of the paper.

## Boolean Network Sketches

A Boolean network sketch represents a combination of prior knowledge, data, and hypotheses regarding the modelled system.
Sketch is given by following components: 
- influence graph
- partially specified Boolean network 
- static properties of update functions
- dynamic properties of the system

Given a sketch, all consistent Boolean network candidates can be computed. 
We can then analyze their attractors symbolically, obtain witness networks, or analyze between candidates.

## Requirements and Dependencies

We recommend to run the benchmarks on a machine with at least 16GB RAM. 
Otherwise, the largest benchmarks (models with 100+ variables) may take a long time to evaluate (however, it should be fine to run the code regarding case studies or smaller benchmarks). 
All displayed computation times were acquired on a standard laptop with an 11th Gen Intel i5 CPU and 16GB RAM.

To run the experiments, you will need the Rust compiler. 
We recommend following the instructions on [rustlang.org](https://www.rust-lang.org/learn/get-started) (default configuration should be sufficient) instead of using a package manager, however either method should work ok. 
When using the official installer, everything is stored in `~/.cargo`, so admin privilages are not necessary. 
Once you are done with the experiments, you can uninstall the compiler by running `rustup self uninstall`. 
The tested version of Rust is `1.64.0` (Sep 2022).
Rust will automatically download and cache all other libraries necessary to compile the project. 
You should therefore have internet access while running the commands for the first time. 
You can force rust do download all dependencies by running `cargo fetch`.

Moreover, for convenience, we have prepared Bash scripts to wrap all the commands needed. If you want to use them, you will need a Unix-based system with Bash. 
On Windows, it should be sufficient to use [WSL](https://learn.microsoft.com/en-us/windows/wsl/install). 
However, these scripts are generally not needed, as you can run the Rust code directly.


## Sketch Format

The first three components of the sketch are all covered by the intuitive aeon model format (details [here](https://biodivine.fi.muni.cz/aeon/aeon-manual.pdf)). This format describes:
- influences between variables
- types of influences (such as monotonicity or essentiality), from which we can derive properties of update functions
- partially specified update functions 

The dynamic properties are given in a form of a HCTL formula (formulae format is described in detail [here](https://github.com/sybila/biodivine-hctl-model-checker)). Formulae used in experiments are (partially) generated from data on the run.

## Benchmark Models and Pre-Computed Results

All the models used for the evaluation (and some more) are in the `benchmark_models` directory. 
Each model has its own subdirectory.

Models used for the case studies are in the sub-folders `case_study_TLGL` and `case_study_arabidopsis`.
These sub-folders contain all the model variants in aeon format, metadata, encoded attractor states. 
They also contain files with the raw outputs from corresponding binaries and summarization of update functions of resulting candidates.

The running example from the paper is in the `small_example` subdirectory. 
There is the starting aeon model, the HCTL formulae used, and the results (including the single consistent network).

The sub-folders with models used for scalability evaluation contain four files each:
- `model_concrete.aeon` - a fully specified model with which we started
- `model_parametrized.aeon` - a parametrized version of the same model used for the sketch
- `attractor_states.txt` - a collection of encoded synthetic attractor data
- `results.txt` - resulting output from the corresponding binary (see below)
- `metadata.txt` - links to the model's source and publication

## Binaries and scripts for experiments

You can either use prepared Bash wrapper scripts to re-run the experiments, or run them directly by compiling and executing the Rust programmes.

### Bash scripts

We have prepared four Bash scripts which encompass the compilation and execution of the underlying Rust code.
Scripts `run_case_study_1.sh` and `run_case_study_2.sh` run the corresponding case studies - for each case study, two computations are executed, corresponding to the two respective sketch variants from the paper.
To execute the evaluation of all scalability benchmarks one by one, use `run_scalability_benchmarks.sh`.
The all-encompassing script `run_all_experiments.sh` runs all these experiments (both case studies and all benchmarks), one after another. Scripts are executed as usual:

```
./run_case_study_1.sh
./run_case_study_2.sh
./run_benchmarks_scalability_unix.sh
./run_all_experiments.sh
```

Each script prints the result summarization and all the relevant progress on the standard output.

The scripts should work on any classical Unix-based system, and we have also tested them on Windows Subsystem for Linux (WSL).

### Individual binaries

To directly re-run individual benchmarks or case studies, compile the code first (on Windows, we recommend Powershell 5+ to run the experiments):
```
cargo build --release
```

The Rust code regarding the two case studies from the paper can be found in `src/bin` folder (particularly, `case_study_arabidopsis.rs` and `case_study_tlgl.rs`). 
The resulting binaries will be in `target/release` folder and can be executed as shown below. 
Note that on Windows, the path is usually `target\release` and binaries have `.exe` suffix. 
```
./target/release/case-study-tlgl [--refined-sketch] 
./target/release/case-study-arabidopsis [--fixed-points] [--prohibit-extra-attrs]
```

To see detailed options for each of these binaries, run 
```
./target/release/<CASE-STUDY-NAME> -h
```

To run the experiments regarding scalability, use the following binary.
It is a general method for inference using network sketches with attractor data, so you can choose arbitrary model. 

````
./target/release/inference-with-attractors [OPTIONS] <MODEL_PATH> <ATTRACTOR_DATA_PATH>
````
- `MODEL_PATH` is a path to the file with a model in aeon format
- `ATTRACTOR_DATA_PATH` is a path to the file with attractor data (one encoded state per line)
- most important `OPTIONS` are
  - `--prohibit-extra-attrs`: add property prohibiting all additional attractors not containing any specified attractor state
  - `--fixed-points`: use properties with fixed-point attractors only (instead of general attractors)
  - `--goal-model <GOAL_MODEL>`: a fully specified BN model to look for in the resulting set of candidates


## Reusability

The implementation is based mainly on two of our Rust libraries: 
- [biodivine-hctl-model-checker](https://github.com/sybila/biodivine-hctl-model-checker) for the underlying symbolic coloured model checking
- [biodivine-lib-param-bn](https://github.com/sybila/biodivine-lib-param-bn) which facilitate the symbolic encoding of Boolean networks

The implementation itself is fairly minimal (includes i.e., high-level framework or encoding of dynamic properties) and contains basic comments that explain the function of individual components.

Everything is open-source and available with the permissive MIT License.

To visualize and modify the BN models (aeon files) included in this repository, you can use the online interface of our tool [AEON](https://biodivine.fi.muni.cz/aeon). This tool can also be used to further analyse the attractors of Boolean network models.


## Tests 
To run the set of tests, use `cargo test` command.