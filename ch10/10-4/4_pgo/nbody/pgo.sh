cargo clean
cargo build --release
mv ./target/release/nbody ./target/release/nbody.org

rm -rf /tmp/pgo-data

RUSTFLAGS="-Cprofile-generate=/tmp/pgo-data" cargo build --release

./target/release/nbody 50000000

~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-profdata merge -o /tmp/pgo-data/merged.profdata /tmp/pgo-data

RUSTFLAGS="-Cprofile-use=/tmp/pgo-data/merged.profdata" cargo build --release
mv ./target/release/nbody ./target/release/nbody.pgo

hyperfine "./target/release/nbody.org 50000000" "./target/release/nbody.pgo 50000000"
