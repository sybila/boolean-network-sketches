#!/bin/bash

echo ">>>>>>>>>> COMPILE RUST BINARIES"
cargo build --release

# first case study only

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
