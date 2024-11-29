list:
    @just --list

lint:
    cargo clippy

score:
    @cp ../create2crunch/efficient_addresses.txt .
    cargo run --release ./efficient_addresses.txt
    @rm efficient_addresses.txt
