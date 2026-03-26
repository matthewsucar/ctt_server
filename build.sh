#!/bin/bash

cargo install cargo-generate-rpm
RUSTC_BOOTSTRAP=1 cargo build --release
cargo generate-rpm --auto-req disabled
