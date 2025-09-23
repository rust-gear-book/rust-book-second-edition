cargo clean
cargo build --release
mv ./target/release/spectralnorm ./target/release/spectralnorm.org

rm -rf /tmp/pgo-data

RUSTFLAGS="-Cprofile-generate=/tmp/pgo-data" cargo build --release

./target/release/spectralnorm 5500

~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-profdata merge -o /tmp/pgo-data/merged.profdata /tmp/pgo-data

RUSTFLAGS="-Cprofile-use=/tmp/pgo-data/merged.profdata" cargo build --release
mv ./target/release/spectralnorm ./target/release/spectralnorm.pgo

hyperfine "./target/release/spectralnorm.org 5500" "./target/release/spectralnorm.pgo 5500"
