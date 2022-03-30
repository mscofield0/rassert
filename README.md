# rassert

Simple macro for expressing Result-returning assertions.

## Usage

```rust
use rassert::rassert;

enum MyError {
    NotAnswerToLife,
}

struct SomeOutput;

pub fn foo(input: usize) -> Result<SomeOutput, MyError> {
    rassert!(input == 42, MyError::NotAnswerToLife);

    let output = ...;
    Ok(output)
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

