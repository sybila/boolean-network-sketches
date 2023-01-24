# Boolean Network Sketches: A Unifying Framework for Logical Model Inference

This is a small Rust library focusing on logical model inference through Boolean network sketches. 
It contains the high-level implementation of the framework, as well as benchmark models and data used for experiments and case studies. 
This readme describes the repository contents and also contains instructions on how to replicate the main experimental results of the paper.
The whole repository is available at [github](https://github.com/sybila/boolean-network-sketches), as well as at [zenodo](https://doi.org/10.5281/zenodo.7490408).

## Requirements and Dependencies

We recommend to run the benchmarks on a machine with at least 16GB RAM. 
Otherwise, the largest benchmarks (models with 100+ variables) may take a long time to evaluate (however, even with less memory, it should be fine to run the code regarding both case studies or smaller benchmarks). 
All displayed computation times were acquired on a standard laptop with an 11th Gen Intel i5 CPU and 16GB RAM.

To run the experiments, you will need the Rust compiler. 
We recommend following the instructions on [rustlang.org](https://www.rust-lang.org/learn/get-started) (default configuration should be sufficient) instead of using a package manager, however either method should work ok. 
When using the official installer, everything is stored in `~/.cargo`, so admin privilages are not necessary. 
Once you are done with the experiments, you can uninstall the compiler by running `rustup self uninstall`. 
The tested version of Rust is `1.64.0` (Sep 2022).
Rust will automatically download and cache all other libraries necessary to compile the project. 
You should therefore have internet access while running the commands for the first time. 
You can force rust do download all dependencies by running `cargo fetch`.

To successfully compile and run the binaries, on some systems, you will also need to get the Z3 library (for the purposes of linking).
We recommend downloading the Z3 release directly from [their GitHub](https://github.com/Z3Prover/z3/releases) (we have used z3-4.11.0).
The binaries will look for the relevant `z3.ddl` or `libz3.so` (or some other similar version your system requires), or it can also be linked statically. 
The easiest way should be to just add `-L path_to_z3/bin` to the environment variable `RUSTFLAGS`.
Some systems, such as Ubuntu 20.04.5 LTS, should not require this step.

Moreover, for convenience, we have prepared Bash scripts to wrap all the commands needed. If you want to use them, you will need a Unix-based system with Bash. 
On Windows, it should be sufficient to use [WSL](https://learn.microsoft.com/en-us/windows/wsl/install) (we have tested the Ubuntu-20.04 WSL). 
However, note that these scripts are not needed, as you can execute the Rust code directly (as described below).

## Sketch Format

Boolean network sketch represents a combination of prior knowledge, experimental data, and hypotheses regarding the modelled system.
BN sketch is given by following components:
- influence graph
- partially specified Boolean network
- properties of update functions
- dynamic properties of the system

The first three components of the sketch are all covered by the intuitive aeon model format (details [here](https://biodivine.fi.muni.cz/aeon/aeon-manual.pdf)). 
This format allows description of:
- possible influences between variables
- kinds of influences (such as inhibiting/activating or observable/non-observable), from which we can derive update function properties
- partially specified update functions 

The dynamic properties are given in a form of HCTL formulae (our formulae format is described in detail [here](https://github.com/sybila/biodivine-hctl-model-checker)). 
Several types of formulae can be generated from data on the run (via utilities in `src/create_inference_formulae.rs`).

## Benchmark Models and Pre-Computed Raw Results

All the models used for the evaluation (and some more) are in the `benchmark_models` directory. 
Each model has its own subdirectory.

Models used for the case studies are in the sub-folders `case_study_TLGL` and `case_study_arabidopsis`.
These sub-folders contain all the model variants in aeon format, encoded attractor states, and metadata. 
They also contain files with the raw outputs from corresponding binaries and summarization of update functions of resulting candidates.

The running example from the paper is in the `small_example` subdirectory. 
There is the starting aeon model, the HCTL formulae used, and the results (including the single consistent network).

The sub-folders with models used for scalability evaluation contain four files each:
- `model_concrete.aeon` - a fully specified model with which we started
- `model_parametrized.aeon` - a parametrized version of the same model, used for the sketch
- `attractor_states.txt` - a collection of encoded synthetic attractor data
- `results.txt` - resulting output from the corresponding binary (see below)
- `metadata.txt` - links to the model's original source and publication

## Binaries and scripts for experiments

You can either use prepared Bash wrapper scripts to re-run the experiments, or run them directly by compiling and executing the Rust binaries.

### Bash wrapper scripts

We have prepared four Bash scripts which encompass the compilation and execution of the underlying Rust code.
Scripts `run_case_study_1.sh` and `run_case_study_2.sh` run the corresponding case studies - for each case study, two computations are executed, corresponding to the two respective sketch variants from the paper.
To execute the evaluation of all scalability benchmarks one by one, use `run_scalability_benchmarks.sh`.
The all-encompassing script `run_all_experiments.sh` runs all these experiments (both case studies and all benchmarks), one after another.
Each script prints the result summarization and all the relevant progress on the standard output.
Scripts can be executed for example as:

```
bash ./run_case_study_1.sh
bash ./run_case_study_2.sh
bash ./run_scalability_benchmarks.sh
bash ./run_all_experiments.sh
```

The scripts should work on any classical Unix-based system, and we have also tested them on Windows Subsystem for Linux (WSL), particularly on Ubuntu-20.04 WSL.
If you have a Unix-based system but have a problem running Bash scripts, you can try the Python version of the encompassing script.
You will need a Python 3 (we have used Python 3.8 and 3.10).

```
python3 run_all_experiments.py
```

### Individual binaries

To directly re-run individual benchmarks or case studies, compile the code first (on Windows, we recommend using Powershell 5+ to run the experiments):
```
cargo build --release
```

The Rust code regarding the two case studies from the paper can be found in `src/bin/` folder (particularly, `case_study_arabidopsis.rs` and `case_study_tlgl.rs`). 
The resulting binaries will be in `target/release/`. 
Note that on Windows, the path is usually `target\release\` and binaries have `.exe` suffix. 

Each case study consists of two parts - one regarding the initial version of the sketch, and the other regarding the refined (modified) variant of the sketch. 
You can choose the desired variant using an option, as shown below.
To re-run the desired variant, execute one of following binaries either with or without the option:

```
./target/release/case-study-tlgl [--refined-sketch-variant] 
./target/release/case-study-arabidopsis [--modified-sketch-variant]
```

To run the experiments regarding scalability, use the following binary.
It is a general method for the inference using network sketches with attractor data, so you can choose an arbitrary model. 

````
./target/release/inference-with-attractors [OPTIONS] <MODEL_PATH> <ATTRACTOR_DATA_PATH>
````
- `MODEL_PATH` is a path to a file with a parametrized model in aeon format
- `ATTRACTOR_DATA_PATH` is a path to a file with attractor data (one encoded state per line)

You do not need to add any options to replicate the experimental results, but if you want to know more, use:
````
./target/release/inference-with-attractors -h
````


## Availability

This repository is available on [Github](https://github.com/sybila/boolean-network-sketches) and also at [zenodo](https://doi.org/10.5281/zenodo.7490408).

The implementation is based mainly on two of our Rust libraries: 
- [biodivine-hctl-model-checker](https://github.com/sybila/biodivine-hctl-model-checker) for the underlying symbolic coloured model checking
- [biodivine-lib-param-bn](https://github.com/sybila/biodivine-lib-param-bn) which facilitate the symbolic encoding of Boolean networks

The implementation itself is fairly minimal (includes i.e., high-level framework or encoding of dynamic properties) and contains basic comments that explain the function of individual components.

Everything is open-source and available with the permissive MIT License.

To visualize and modify the partially specified BN models (aeon files) included in this repository, you can use the online interface of our tool [AEON](https://biodivine.fi.muni.cz/aeon). This tool can also be used to further analyse the attractors of Boolean network models.


## Tests 
To run the set of tests, use `cargo test` command.