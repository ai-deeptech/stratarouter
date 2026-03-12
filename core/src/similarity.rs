// This file is intentionally empty.
//
// The `similarity.rs` module was an earlier prototype that exposed
// cosine-similarity functions via PyO3.  That functionality now lives in:
//   - `algorithms/vector_ops.rs`  (pure Rust, used internally)
//   - `ffi.rs`                    (PyO3 surface, compiled with --features python)
//
// It is NOT declared as a module in lib.rs and is NOT compiled.
//
// Safe to delete in a future clean-up PR.
