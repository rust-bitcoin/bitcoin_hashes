#!/bin/bash -ex

# Combination of features to test
# note std has a comma in the end so that following regex avoid matching serde-std
FEATURES=("" "std," "std,serde" "serde" "use-core2,serde" "use-core2" "use-core2-std" "std,use-core2" "std,serde-std" "use-core2,serde-std")

# Use toolchain if explicitly specified
if [[ -n "$TOOLCHAIN" ]]; then
    alias cargo="cargo +$TOOLCHAIN"
fi

cargo --version
rustc --version

# Make all cargo invocations verbose
export CARGO_TERM_VERBOSE=true

# Defaults / sanity checks
cargo build --all
cargo test --all

if [[ "$DO_FEATURE_MATRIX" = true ]]; then
  for feature in "${FEATURES[@]}"
  do
      # On rust 1.29.0 we are only testing with std lib and without use-core2
      if [[ "$ON_1_29_0" = false || (${feature} =~ "std," && ! ${feature} =~ "use-core2") ]]; then
          echo "--------------$feature----------------"
          cargo build --no-default-features --features="$feature"
          if [[ ${feature} =~ "std," ]] ; then
              cargo test --no-default-features --features="$feature"
          fi
          cargo doc --no-default-features --features="$feature"
      fi
  done
fi

if [[ "$ON_1_29_0" = false ]]; then
    (cd extended_tests/schemars && cargo test)
fi

# Webassembly stuff
if [[ "$DO_WASM" = true ]]; then
    clang --version &&
    CARGO_TARGET_DIR=wasm cargo install --force wasm-pack &&
    printf '\n[lib]\ncrate-type = ["cdylib", "rlib"]\n' >> Cargo.toml &&
    CC=clang-9 wasm-pack build &&
    CC=clang-9 wasm-pack test --node;
fi

# Address Sanitizer
if [[ "$DO_ASAN" = true ]]; then
    cargo clean
    CC='clang -fsanitize=address -fno-omit-frame-pointer'                                        \
    RUSTFLAGS='-Zsanitizer=address -Clinker=clang -Cforce-frame-pointers=yes'                    \
    ASAN_OPTIONS='detect_leaks=1 detect_invalid_pointer_pairs=1 detect_stack_use_after_return=1' \
    cargo test --lib --all --all-features -Zbuild-std --target x86_64-unknown-linux-gnu
    cargo clean
    CC='clang -fsanitize=memory -fno-omit-frame-pointer'                                         \
    RUSTFLAGS='-Zsanitizer=memory -Zsanitizer-memory-track-origins -Cforce-frame-pointers=yes'   \
    cargo test --lib --all --all-features -Zbuild-std --target x86_64-unknown-linux-gnu
fi

# Bench
if [[ "$DO_BENCH" = true ]]; then
    cargo bench --all --features="unstable"
fi
