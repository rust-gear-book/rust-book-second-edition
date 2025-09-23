cargo clean
RUSTFLAGS="-Ctarget-cpu=native" cargo build --release
mv ./target/release/fasta ./target/release/fasta.org

rm -rf /tmp/pgo-data

RUSTFLAGS="-Ctarget-cpu=native -Cprofile-generate=/tmp/pgo-data" cargo build --release

./target/release/fasta 25000000

~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-profdata merge -o /tmp/pgo-data/merged.profdata /tmp/pgo-data

RUSTFLAGS="-Ctarget-cpu=native -Cprofile-use=/tmp/pgo-data/merged.profdata" cargo build --release
mv ./target/release/fasta ./target/release/fasta.pgo

hyperfine "./target/release/fasta.org 25000000" "./target/release/fasta.pgo 25000000"
