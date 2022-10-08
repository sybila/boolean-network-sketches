# Boolean network inference library

Work in progress. 

Small library focused on the Boolean network inference through Network Sketches.

Boolean network sketch is given by 4 components: 
- influence graph
- partially specified Boolean network 
- static properties of update functions (such as monotonicity or observability)
- dynamic properties of the system (given by HCTL formula)

Given a sketch, we can compute all consistent candidate Boolean networks.

## Case studies and binaries
Code regarding case studies can be found in `src/bin` folder. To run the individual case studies, first compile the code
```
cargo build --release
```
and then run one of the binaries:
```
.\target\release\case-study-tlgl [--basic-sketch-version]
.\target\release\case-study-arabidopsis [--fixed-point-version]
.\target\release\case-study-cns
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
- `OPTIONS` are
  - `--forbid-extra-attrs`: forbid all additional attractors not containing specified states
  - `--goal-model <GOAL_MODEL>`: a goal model to check for in the resulting ensemble of candidates
  - `--steady-states`: compute with steady state attractors only