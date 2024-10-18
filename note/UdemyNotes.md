# Learning Notes from Udemy Course

# Rust Orientation

```bash
# Compile rust with rustc
rustc Example1.rs

# Using cargo
cargo new learning_rust
cargo run
cargo build
cargo build --release
```

## Quick Startup

```rust
let x: i16 = 10; // immutable
let mut y = 5; // mutable
const MAX_VALUE: u32 = 100; // constant

//Shadowing
let t = 10;
let t = t * 3;

// Type aliasing
type Age = u8;
let peter_age: Age = 42;

// Type Conversion
let a = 10;
let b = a as f64;

// Strings
let fixed_str = "Fixed length string"; // &str 
let mut flexible_str = String::from("This string will grow");
flexible_str.push('s');

// Data types for series
let mut array_1 = [4, 5, 6, 7];
let vec_1: Vec<i32> = vec![4, 5, 6, 7];

let my_info = ("Salary", 4000, "Age", 40);
let salary_value = my_info.1;
let (salary, salary_value, age, age_value) = my_info

// Unit type
let unit = ();
```
