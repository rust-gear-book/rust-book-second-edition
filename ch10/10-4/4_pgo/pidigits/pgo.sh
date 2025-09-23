cargo clean
cargo build --release
mv ./target/release/pidigits ./target/release/pidigits.org

rm -rf /tmp/pgo-data

RUSTFLAGS="-Cprofile-generate=/tmp/pgo-data" cargo build --release

./target/release/pidigits 10000

~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-profdata merge -o /tmp/pgo-data/merged.profdata /tmp/pgo-data

RUSTFLAGS="-Cprofile-use=/tmp/pgo-data/merged.profdata" cargo build --release
mv ./target/release/pidigits ./target/release/pidigits.pgo

hyperfine "./target/release/pidigits.org 10000" "./target/release/pidigits.pgo 10000"
