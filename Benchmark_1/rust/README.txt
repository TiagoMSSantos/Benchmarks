# Compile
cargo build --manifest-path=Benchmark_1/rust/Cargo.toml --release;

# Run
cd Benchmark_1/rust; time cargo run --manifest-path=Cargo.toml --release; cd ../..;
