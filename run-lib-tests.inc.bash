#!/bin/bash

cargo watch -c -x "test -- --nocapture && cargo clippy"

# cargo watch -q -c -x 'test solo -- --nocapture --quiet && cargo test -- --nocapture --quiet && cargo clippy --quiet && cargo run --quiet'

