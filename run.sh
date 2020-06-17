#!/bin/bash

TEST=${1:-Benchmark_1}

# Compile
javac -cp ${TEST}/java -d ${TEST}/java/build ${TEST}/java/src/BenchMark.java;
cargo build --manifest-path=${TEST}/rust/Cargo.toml --release;
scalac -cp ${TEST}/scala -d ${TEST}/scala/build ${TEST}/scala/src/BenchMark.scala;
g++ -O3 ${TEST}/cpp/BenchMark.cpp -o ${TEST}/cpp/build/BenchMark;

# Run
cd ${TEST}/java; time java -cp ./build BenchMark; cd ../..; printf "\n\n";
cd ${TEST}/rust; time cargo run --manifest-path=Cargo.toml --release; cd ../..; printf "\n\n";
cd ${TEST}/scala; time scala -cp ./build BenchMark; cd ../..; printf "\n\n";
cd ${TEST}/cpp; time ./build/BenchMark; cd ../..; printf "\n\n";
