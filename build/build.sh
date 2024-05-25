#!/bin/bash

# Build for each target
targets=("x86_64-apple-darwin" "aarch64-apple-darwin" "x86_64-unknown-linux-gnu")

for target in "${targets[@]}"; do
    cargo build --release --target $target
done

# Prepare output directory
rm -rf ./build/output
mkdir -p ./build/output

# Package and generate shasums
for target in "${targets[@]}"; do
    tar -czvf ./build/output/wiper-$target.tar.gz -C ./target/$target/release wiper
    shasum -a 256 ./build/output/wiper-$target.tar.gz > ./build/output/wiper-$target.sha256
done

