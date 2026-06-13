# Day 1 - Primitive Data Types

Date: 13 June 2026

## What I Learned

Today I started learning Rust and studied primitive data types.

Primitive data types are the basic building blocks used to store different kinds of values in Rust.

---

## Signed Integers

Signed integers can store both positive and negative numbers.

Types:
- i8
- i16
- i32
- i64
- i128

Example:

```rust
let age: i32 = 20;
let temperature: i32 = -5;
```

---

## Unsigned Integers

Unsigned integers can store only positive numbers and zero.

Types:
- u8
- u16
- u32
- u64
- u128

Example:

```rust
let score: u32 = 100;
```

---

## Floating Point Numbers

Used for decimal values.

Types:
- f32
- f64

Example:

```rust
let pi: f64 = 3.14159;
```

---

## Boolean

Stores either true or false.

Example:

```rust
let is_student: bool = true;
```

---

## Character

Stores a single Unicode character.

Example:

```rust
let grade: char = 'A';
```

---

## Key Takeaways

- Rust has different integer types based on size.
- Signed integers can store negative values.
- Unsigned integers store only positive values and zero.
- Floating-point types are used for decimal numbers.
- Boolean values are either true or false.
- Character values are enclosed in single quotes.

---

## Today's Code

```rust
fn main() {
    let age: i32 = 20;
    let height: f64 = 5.9;
    let is_student: bool = true;
    let grade: char = 'A';

    println!("Age: {}", age);
    println!("Height: {}", height);
    println!("Student: {}", is_student);
    println!("Grade: {}", grade);
}
```

---

## Next Topic

Variables and Mutability