cargo clean
cargo build --release
mv ./target/release/binarytrees ./target/release/binarytrees.org

rm -rf /tmp/pgo-data

RUSTFLAGS="-Cprofile-generate=/tmp/pgo-data" cargo build --release

./target/release/binarytrees 21

~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-profdata merge -o /tmp/pgo-data/merged.profdata /tmp/pgo-data

RUSTFLAGS="-Cprofile-use=/tmp/pgo-data/merged.profdata" cargo build --release
mv ./target/release/binarytrees ./target/release/binarytrees.pgo

hyperfine "./target/release/binarytrees.org 21" "./target/release/binarytrees.pgo 21"
