# Rust

## Style

* Rust code uses snake case as the conventional style for function and variable names.

## Scalar Types

### Integer Types

| Length  | Signed | Unsigned |
|---------|--------|----------|
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |

### Number Literals

| Number literals | Example     |
|-----------------|-------------|
| Decimal         | 98_222      |
| Hex             | 0xff        |
| Octal           | 0o77        |
| Binary          | 0b1111_0000 |
| Byte (u8 only)  | b'A'        |

Floats that are typed via inference are typed as `f64`.

```
fn main() {
    let x = 2.0; // f64 (double precision)

    let y: f32 = 3.0; // f32 (single precision)
}
```

### Characters

```
fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}
```

## Compound Types

### Tuples

Tuples have a fixed length: once declared, they cannot grow or shrink in size.

Access via destructuring:

```
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}
```

Access via dot notation:

```
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

### Arrays

Unlike a tuple, every element of an array must have the same type. Arrays in Rust are different from arrays in some other languages because arrays in Rust have a fixed length, like tuples.

```
let a: [i32; 5] = [1, 2, 3, 4, 5];

let a = [3; 5]; // [3, 3, 3, 3, 3]
```

## Functions

* In function signatures, you must declare the type of each parameter.

<!-- TODO: Statements/Expressions -->
