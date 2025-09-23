#cargo clean
#cargo build --release
#mv ./target/release/mandelbrot ./target/release/mandelbrot.org
#
#rm -rf /tmp/pgo-data
#
#RUSTFLAGS="-Cprofile-generate=/tmp/pgo-data" cargo build --release
#
#./target/release/mandelbrot 16000
#
#~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-profdata merge -o /tmp/pgo-data/merged.profdata /tmp/pgo-data
#
#RUSTFLAGS="-Cprofile-use=/tmp/pgo-data/merged.profdata" cargo build --release
#mv ./target/release/mandelbrot ./target/release/mandelbrot.pgo

hyperfine "./target/release/mandelbrot.org 16000" "./target/release/mandelbrot.pgo 16000"
