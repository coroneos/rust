# Rust

🦀Rustacean

bastian - A never-ending story.



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


### Boolean Type

Rust will not automatically convert non-Boolean types to a Boolean.


### Character Type

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


### Statements & Expressions

* Statements are instructions that perform some action and do not return a value. They end with a semicolon.
* Expressions evaluate to a resulting value. They do not end with a semicolon.



## Control Flow


### Using `if` in a `let` Statement

You can do this, but all of the returned values must be of the same type:

```
fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}
```


### `loop`

```
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
```


`for`

```
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
```

```
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
```

## Memory & Allocation

### Move

This causes a compiler error.

```
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);
```

### Clone

This creates a deep copy.

```
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

### Copy (Stack-Only Data)

```
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
```

This works because integers are stored entirely on the stack.

You can check the documentation, but as a general rule, any group of simple scalar values can be `Copy`, and nothing that requires allocation or is some form of resource is `Copy`. Here are some of the types that are `Copy`:

* All the integer types, such as `u32`.
* The Boolean type, `bool`, with values `true` and `false`.
* All the floating point types, such as `f64`.
* The character type, `char`.
* Tuples, if they only contain types that are also `Copy`. For example, `(i32, i32)` is `Copy`, but `(i32, String)` is not.


### Borrowing

We call having references as function parameters borrowing.

Just as variables are immutable by default, so are references. We’re not allowed to modify something we have a reference to.


### Mutable References

Mutable references have one big restriction: you can have only one mutable reference to a particular piece of data in a particular scope. This code will fail:

```
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{}, {}", r1, r2);
```

The benefit of having this restriction is that Rust can prevent data races at compile time. A data race is similar to a race condition and happens when these three behaviors occur:

* Two or more pointers access the same data at the same time.
* At least one of the pointers is being used to write to the data.
* There’s no mechanism being used to synchronize access to the data.

We can use curly brackets to create a new scope, allowing for multiple mutable references, just not simultaneous ones:

```
let mut s = String::from("hello");

{
    let r1 = &mut s;

} // r1 goes out of scope here, so we can make a new reference with no problems.

let r2 = &mut s;
```

A similar rule exists for combining mutable and immutable references. This code results in an error:

```
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM

println!("{}, {}, and {}", r1, r2, r3);
```

We also cannot have a mutable reference while we have an immutable one. Users of an immutable reference don’t expect the values to suddenly change out from under them! However, multiple immutable references are okay because no one who is just reading the data has the ability to affect anyone else’s reading of the data.

## The Slice Type