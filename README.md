This repository contains a simple Rust program that demonstrates the common error of creating multiple mutable references to the same variable. The code attempts to create two mutable references (`y` and `z`) to the variable `x`. This is not allowed in Rust because it could lead to data races and unpredictable behavior. The solution demonstrates how to use a single mutable reference or cloning to avoid this error. This is crucial to understanding Rust's ownership and borrowing system.