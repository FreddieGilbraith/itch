#!/bin/bash

env node generateTests.js > tests/generated.rs && cargo fmt && cargo test
