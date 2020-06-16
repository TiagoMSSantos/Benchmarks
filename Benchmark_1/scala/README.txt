# Compile
scalac -cp Benchmark_1/scala -d Benchmark_1/scala/build Benchmark_1/scala/src/BenchMark.scala

# Run
cd Benchmark_1/scala; time scala -cp ./build BenchMark; cd ../..;
