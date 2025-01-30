# Data Race in Rust

This repository demonstrates a common error in Rust: creating a data race by having multiple mutable references to the same variable. The `bug.rs` file contains code that exhibits this race condition. The `bugSolution.rs` file offers a corrected version that avoids the race condition.  Understanding ownership and borrowing is crucial for writing safe and concurrent Rust code.