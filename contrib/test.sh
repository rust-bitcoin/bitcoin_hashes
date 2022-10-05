#!/bin/sh -ex

FEATURES="serde serde-std std core2"

if [ "$DO_ALLOC_TESTS" = true ]; then
	FEATURES="$FEATURES alloc"
fi

cargo --version
rustc --version

# Work out if we are using a nightly toolchain.
NIGHTLY=false
if cargo --version | grep nightly >/dev/null; then
    NIGHTLY=true
fi

# Make all cargo invocations verbose
export CARGO_TERM_VERBOSE=true

# Pin if using MSRV toolchain.
REQUIRE_VERSION_PINNING=false
if cargo --version | grep "cargo 1\.41"; then
    REQUIRE_VERSION_PINNING=true
fi

# Defaults / sanity checks
cargo build --all
cargo test --all


if [ "$REQUIRE_VERSION_PINNING" = true ]; then
    cargo update --package schemars --precise 0.8.3
    cargo update --package dyn-clone --precise 1.0.7
fi

if [ "$DO_FEATURE_MATRIX" = true ]; then
    cargo build --all --no-default-features
    cargo test --all --no-default-features

    # All features
    cargo build --all --no-default-features --features="$FEATURES"
    cargo test --all --no-default-features --features="$FEATURES"
    # Single features
    for feature in ${FEATURES}
    do
        cargo build --all --no-default-features --features="$feature"
        cargo test --all --no-default-features --features="$feature"
		# All combos of two features
		for featuretwo in ${FEATURES}; do
			cargo build --all --no-default-features --features="$feature $featuretwo"
			cargo test --all --no-default-features --features="$feature $featuretwo"
		done
    done

    # Other combos
    cargo test --all --no-default-features --features="std,schemars"
fi

if [ "$DO_SCHEMARS_TESTS" = true ]; then
    (
        cd extended_tests/schemar
        if [ "$REQUIRE_VERSION_PINNING" = true ]; then
            cargo update --package schemars --precise 0.8.3
            cargo update --package dyn-clone --precise 1.0.7
        fi

        cargo test
    )
fi

# Build the docs if told to (this only works with the nightly toolchain)
if [ "$DO_DOCS" = true ]; then
    RUSTDOCFLAGS="--cfg docsrs" cargo doc --all --features="$FEATURES"
fi

# Webassembly stuff
if [ "$DO_WASM" = true ]; then
    clang --version &&
    CARGO_TARGET_DIR=wasm cargo install --force wasm-pack &&
    printf '\n[lib]\ncrate-type = ["cdylib", "rlib"]\n' >> Cargo.toml &&
    CC=clang-9 wasm-pack build &&
    CC=clang-9 wasm-pack test --node;
fi

# Address Sanitizer
if [ "$DO_ASAN" = true ]; then
    cargo clean
    CC='clang -fsanitize=address -fno-omit-frame-pointer'                                        \
    RUSTFLAGS='-Zsanitizer=address -Clinker=clang -Cforce-frame-pointers=yes'                    \
    ASAN_OPTIONS='detect_leaks=1 detect_invalid_pointer_pairs=1 detect_stack_use_after_return=1' \
    cargo test --lib --all --no-default-features --features="$FEATURES" -Zbuild-std --target x86_64-unknown-linux-gnu
    cargo clean
    CC='clang -fsanitize=memory -fno-omit-frame-pointer'                                         \
    RUSTFLAGS='-Zsanitizer=memory -Zsanitizer-memory-track-origins -Cforce-frame-pointers=yes'   \
    cargo test --lib --all --no-default-features --features="$FEATURES" -Zbuild-std --target x86_64-unknown-linux-gnu
fi

# Bench if told to, only works with non-stable toolchain (nightly, beta).
if [ "$DO_BENCH" = true ]
then
    if [ "$NIGHTLY" = false ]
    then
        if [ -n "$RUSTUP_TOOLCHAIN" ]
        then
            echo "RUSTUP_TOOLCHAIN is set to a non-nightly toolchain but DO_BENCH requires a nightly toolchain"
        else
            echo "DO_BENCH requires a nightly toolchain"
        fi
        exit 1
    fi
    RUSTFLAGS='--cfg=bench' cargo bench
fi
