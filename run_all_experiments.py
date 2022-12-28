import os


print(">>>>>>>>>> COMPILE RUST BINARIES")
os.system("cargo build --release")


# first case study

print("\n>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>")
print(">>>>>>>>>> CASE STUDY 1, INITIAL VARIANT OF THE SKETCH")
print(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>\n")
os.system("./target/release/case-study-tlgl")
print("./target/release/case-study-tlgl")

print("\n>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>")
print(">>>>>>>>>> CASE STUDY 1, REFINED VARIANT OF THE SKETCH")
print(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>\n")
os.system("./target/release/case-study-tlgl -r")


# second case study

print("\n>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>")
print(">>>>>>>>>> CASE STUDY 2, VARIANT OF THE SKETCH WITH FIXED-POINT PROPERTIES ONLY")
print(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>\n")
os.system("./target/release/case-study-arabidopsis")

print("\n>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>")
print(">>>>>>>>>> CASE STUDY 2, VARIANT OF THE SKETCH WITH COMPLEX PROPERTIES")
print(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>\n")
os.system("./target/release/case-study-arabidopsis -m")


# scalability benchmarks

# all models are in the sub-folders in the same directory
BENCH_DIR = "benchmark_models"
# all files in benchmark sub-folders are named in a same way
MODEL_FILE = "model_parametrized.aeon"
GOAL_MODEL_FILE = "model_concrete.aeon"
ATTRACTORS_FILE = "attractor_states.txt"

print("\n>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>")
print(">>>>>>>>>> START SCALABILITY BENCHMARKS RUN")
print(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>\n")

# run computation for each scalability benchmark, from smallest to largest
for model in ["celldivb_9v", "eprotein_35v", "nsp4_60v", "etc_84v", "interferon1_121v", "nsp9_252v", "macrophage_321v"]:
    print("==========================")
    print(f"Model {model}")
    print("==========================\n")

    MODEL_DIR=f"{BENCH_DIR}/{model}"
    os.system(f"./target/release/inference-with-attractors {MODEL_DIR}/{MODEL_FILE} {MODEL_DIR}/{ATTRACTORS_FILE} -g {MODEL_DIR}/{GOAL_MODEL_FILE}")
    print()
