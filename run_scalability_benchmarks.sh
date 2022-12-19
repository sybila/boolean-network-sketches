#!/bin/bash

echo ">>>>>>>>>> COMPILE RUST BINARIES"
cargo build --release

# scalability benchmarks

# all models are in the sub-folders in the same directory
BENCH_DIR="benchmark_models"
# all files in benchmark sub-folders are named in a same way
MODEL_FILE="model_parametrized.aeon"
GOAL_MODEL_FILE="model_concrete.aeon"
ATTRACTORS_FILE="attractor_states.txt"

echo ">>>>>>>>>> START SCALABILITY BENCHMARKS RUN"

# run computation for each scalability benchmark, from smallest to largest
for model in  110_9v 115_35v 123_60v 114_84v 118_121v 124_252v 001_321v
do
    echo "==============================="
    echo ${model}
    echo "==============================="
    echo

    MODEL_DIR="${BENCH_DIR}/${model}"
    ./target/release/inference-with-attractors "${MODEL_DIR}/${MODEL_FILE}" "${MODEL_DIR}/${ATTRACTORS_FILE}" -g "${MODEL_DIR}/${GOAL_MODEL_FILE}" -p
    echo
done