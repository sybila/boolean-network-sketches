# Boolean network inference library

Small Rust library focusing on logical model inference through Boolean network sketches.

Boolean network sketches represent a combination of prior knowledge, data, and hypotheses regarding the system.
Sketch is given by following components: 
- influence graph
- partially specified Boolean network 
- static properties of update functions
- dynamic properties of the system

Given a sketch, we can compute all consistent candidate Boolean networks. 
We can analyze their attractors symbolically, get witness networks, or analyze differences in a candidate set.

## Installing Dependencies

To run the case studies, you will need the Rust compiler. We recommend following the instructions on [rustlang.org](https://www.rust-lang.org/learn/get-started) (default configuration should be sufficient) instead of using a package manager, however either method should work ok. When using the official installer, everything is stored in `~/.cargo`, so admin privilages are not necessary. Once you are done, you can uninstall the compiler by running `rustup self uninstall`. The tested version of Rust is `1.64.0` (Sep 2022).

TODO

## Input format

The first three components of the sketch are all covered by the AEON model format. This format describes:
- influences between variables
- types of influences (such as monotonicity or essentiality), from which we can derive properties of update functions)
- partially specified update functions 

The dynamic properties are given in a form of a HCTL formula. Some formulae can be generated from data.

## Benchmark models

TODO

## Case studies
Code regarding case studies can be found in `src/bin` folder. To run the individual case studies, first compile the code
```
cargo build --release
```
and then run one of the binaries:
```
.\target\release\case-study-tlgl [--fully-unspecified-logic] 
.\target\release\case-study-arabidopsis [--fixed-points] [--prohibit-extra-attrs]
.\target\release\case-study-cns
```

To see other existing options for each of these case studies, run 
```
.\target\release\<CASE-STUDY-NAME> -h
```

You can also use more general binary for inference using network sketches with attractor data.
In this case, you can choose arbitrary model. 
However, dynamical properties are limited to attractor data only (for now). 
You can specify states that must be contained in candidate's attractors, and you can forbid additional attractors.

````
.\target\release\inference-with-attractors [OPTIONS] <MODEL_PATH> <ATTRACTOR_DATA_PATH>
````
- `MODEL_PATH` is a path to the file with BN model in aeon format
- `ATTRACTOR_DATA_PATH` is a path to the file with attractor data
- most important `OPTIONS` are
  - `--prohibit-extra-attrs`: prohibit all additional attractors not containing specified states
  - `--goal-model <GOAL_MODEL>`: a goal model to check for in the resulting ensemble of candidates
  - `--fixed-points`: compute with steady state attractors only

## Precomputed results

TODO

## Availibility, Extendability and Reusability

TODO

## Tests 
To run set of tests, run `cargo test`.