#!/bin/bash

echo ">>>>>>>>>> COMPILE RUST BINARIES"
cargo build --release

# first case study

echo
echo ">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>"
echo ">>>>>>>>>> CASE STUDY 1, INITIAL VARIANT OF THE SKETCH"
echo ">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>"
echo
./target/release/case-study-tlgl

echo
echo ">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>"
echo ">>>>>>>>>> CASE STUDY 1, REFINED VARIANT OF THE SKETCH"
echo ">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>"
echo
./target/release/case-study-tlgl -r
echo

# second case study

echo
echo ">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>"
echo ">>>>>>>>>> CASE STUDY 2, VARIANT OF THE SKETCH WITH FIXED-POINT PROPERTIES ONLY"
echo ">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>"
echo
./target/release/case-study-arabidopsis

echo
echo ">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>"
echo ">>>>>>>>>> CASE STUDY 2, VARIANT OF THE SKETCH WITH COMPLEX PROPERTIES"
echo ">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>"
echo
./target/release/case-study-arabidopsis -m
echo

# scalability benchmarks

# all models are in the sub-folders in the same directory
BENCH_DIR="benchmark_models"
# all files in benchmark sub-folders are named in a same way
MODEL_FILE="model_parametrized.aeon"
GOAL_MODEL_FILE="model_concrete.aeon"
ATTRACTORS_FILE="attractor_states.txt"

echo
echo ">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>"
echo ">>>>>>>>>> START SCALABILITY BENCHMARKS RUN"
echo ">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>"
echo

# run computation for each scalability benchmark, from smallest to largest
for model in  celldivb_9v eprotein_35v nsp4_60v etc_84v interferon1_121v nsp9_252v macrophage_321v
do
    echo "=========================="
    echo "Model ${model}"
    echo "=========================="
    echo

    MODEL_DIR="${BENCH_DIR}/${model}"
    ./target/release/inference-with-attractors "${MODEL_DIR}/${MODEL_FILE}" "${MODEL_DIR}/${ATTRACTORS_FILE}" -g "${MODEL_DIR}/${GOAL_MODEL_FILE}"
    echo " "
done