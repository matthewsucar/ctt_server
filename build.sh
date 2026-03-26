#!/bin/bash

cargo install cargo-generate-rpm
#RUSTC_BOOTSTRAP to trick the stable compiler into working
#if using nightly you don't need that and shouldn't use it
RUSTC_BOOTSTRAP=1 cargo build --features slack,pbs --release
cargo generate-rpm --auto-req disabled
