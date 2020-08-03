#!/bin/bash

RUSTFLAGS="$RUSTFLAGS -A dead_code -A unused_imports -A non_snake_case -A unused_mut -A unused_must_use" cargo run
