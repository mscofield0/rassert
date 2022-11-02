# rassert

Simple macro for expressing Result-returning assertions and notifying, hard-error assertions (useful for functions where you can't propagate an error upstream so you want to log it).

[![Crates.io][crates-badge]][crates-url]
[![Documentation][docs-badge]][docs-url]
[![MIT licensed][mit-badge]][mit-url]

[crates-badge]: https://img.shields.io/crates/v/rassert-rs
[crates-url]: https://crates.io/crates/rassert-rs
[docs-badge]: https://img.shields.io/docsrs/rassert-rs
[docs-url]: https://docs.rs/rassert-rs
[docs-master-badge]: https://img.shields.io/badge/docs-master-blue
[docs-master-url]: https://tracing-rs.netlify.com
[mit-badge]: https://img.shields.io/crates/l/rassert-rs
[mit-url]: LICENSE

## Usage

```rust
use rassert_rs::{rassert, rassert_notify};

enum MyError {
    NotAnswerToLife,
}

struct SomeOutput;

pub fn foo(input: usize) -> Result<SomeOutput, MyError> {
    rassert!(input == 42, MyError::NotAnswerToLife);

    let output = ...;
    Ok(output)
}

pub fn bar(input: usize) {
    rassert_notify!(1 != 1, error!("Well, that's not true."));
    
    println!("Hi everyone"); // Never reached since the above rassert_notify fails and returns
}
```

## Why

Because the alternative is rather ugly and does not obviously express that the expression is a precondition.

```rust
enum MyError {
    NotAnswerToLife,
}

struct SomeOutput;

pub fn foo(input: usize) -> Result<SomeOutput, MyError> {
    if input != 42 {
        return Err(MyError::NotAnswerToLife);
    }

    let output = ...;
    Ok(output)
}
```

I found myself just copy-pasting the same rassert macro over and over in my projects, so might as well put it on Cargo.

