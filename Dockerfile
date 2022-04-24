# Build Stage
FROM --platform=linux/amd64 rustlang/rust:nightly as builder

## Install build dependencies.
RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y cmake clang

## Add source code to the build stage.
ADD . /trie
WORKDIR /trie/trie-db/fuzz

## TODO: ADD YOUR BUILD INSTRUCTIONS HERE.
RUN cargo +nightly rustc -- \
    -C passes='sancov-module' \
    -C llvm-args='-sanitizer-coverage-level=3' \
    -C llvm-args='-sanitizer-coverage-inline-8bit-counters' \
    -Z sanitizer=address

# Package Stage
FROM --platform=linux/amd64 ubuntu:20.04


## TODO: Change <Path in Builder Stage>
COPY --from=builder trie/trie-db/fuzz/target/x86_64-unknown-linux-gnu/release/trie_root_new /
COPY --from=builder trie/trie-db/fuzz/target/x86_64-unknown-linux-gnu/release/trie_root /
COPY --from=builder trie/trie-db/fuzz/target/x86_64-unknown-linux-gnu/release/trie_root_fix_len /
COPY --from=builder trie/trie-db/fuzz/target/x86_64-unknown-linux-gnu/release/no_ext_insert /
COPY --from=builder trie/trie-db/fuzz/target/x86_64-unknown-linux-gnu/release/no_ext_insert_rem /
COPY --from=builder trie/trie-db/fuzz/target/x86_64-unknown-linux-gnu/release/prefix_iter /
COPY --from=builder trie/trie-db/fuzz/target/x86_64-unknown-linux-gnu/release/seek_iter /
COPY --from=builder trie/trie-db/fuzz/target/x86_64-unknown-linux-gnu/release/trie_proof_valid /
COPY --from=builder trie/trie-db/fuzz/target/x86_64-unknown-linux-gnu/release/trie_codec_proof /
COPY --from=builder trie/trie-db/fuzz/target/x86_64-unknown-linux-gnu/release/trie_proof_invalid /
