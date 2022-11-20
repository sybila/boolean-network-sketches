# Boolean network inference library

Small Rust library focusing on Boolean network inference through network sketches.

Boolean network sketch is a framework that represents a combination of prior knowledge, data, and hypotheses regarding the system.
Sketch is given by following components: 
- influence graph
- partially specified Boolean network 
- static properties of update functions (such as monotonicity or observability)
- dynamic properties of the system

The first three components are all covered by the AEON model format. 
The dynamic properties are given in a form of a HCTL formula.

Given a sketch, we can compute all consistent candidate Boolean networks.

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

## Tests 
To run set of tests, run `cargo test`.