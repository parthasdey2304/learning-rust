<h1 align="center">
  <img src="https://skillicons.dev/icons?i=rust" alt="Rust Logo" height="150px" width="150px">
  <br>
 
</h1>

<p>
   <b>RUST</b> is a systems programming language that is focused on performance, reliability, and safety. It was designed by Mozilla and first announced in 2010. The development of Rust was largely motivated by the desire to create a language that could provide low-level control over system resources without sacrificing safety.
</p>

# History Of Rust
## Origin and Development:

Rust was initially a personal project of Graydon Hoare, a Mozilla employee, who started working on it in 2006.
The first public announcement of Rust was made in 2010.
Mozilla officially sponsored the project, and it became a part of the Mozilla Research division.

## Rust 1.0 Release:

Rust 1.0, the first stable version, was released in May 2015.
The release marked a commitment to stability, meaning that code written in earlier versions would be compatible with future versions.

## Key Features:

Memory Safety: Rust's borrow checker ensures memory safety without the need for a garbage collector.
Concurrency: Rust has built-in support for concurrent programming with ownership and borrowing mechanisms.
Zero-cost Abstractions: Rust provides high-level abstractions without incurring runtime overhead.
No Null or Dangling Pointers: Rust eliminates the possibility of null or dangling pointer references.
## Community and Adoption:

Rust has gained popularity in both industry and open-source communities.
It is often used for systems programming, game development, and other performance-critical applications.

## Tooling:

Rust comes with a package manager called Cargo, which makes it easy to manage dependencies and build projects.
Rust has a strong focus on documentation, and tools like Rustdoc generate documentation directly from the source code comments.

## Ongoing Development:

Rust has a six-week release cycle, allowing for regular updates and improvements.
The language continues to evolve, addressing user feedback and adding new features.

# Basics
Let's start with a classic "Hello, World!" program in Rust. If you haven't installed Rust yet, you can follow the instructions on <a href="https://www.rust-lang.org/learn/get-started">Rust's official website</a> to get it set up.
``` rust
// This is a simple "Hello, World!" program in Rust.
fn main() {
    // The println! macro is used to print text to the console.
    println!("Hello, World!");
}
```

### Explanation
+ `fn main()` defines the main function. In Rust, the execution of the program starts from the main function.
+ `println!` is a macro used for printing text to the console. The ! indicates that it's a macro, not a regular function.

## Variables and Concept of Mutability
``` rust
fn main() {
    // Variables are immutable by default.
    let x = 5;
    println!("The value of x is: {}", x);

    // To make a variable mutable, use the 'mut' keyword.
    let mut y = 10;
    println!("The value of y is: {}", y);

    // You can reassign a value to a mutable variable.
    y = 15;
    println!("Now, the value of y is: {}", y);
}
```
### Explanation
+ Variables are immutable by default in Rust. Once a valie is assigned, it cannot be changed.
+ The `let` keyword is used to create a variable in Rust.
+ The `mut` keyword makes a variable mutable, allowing you the change its value.

## Variable Shadowing
+ You can shadow a variable by reusing the same variable name. This is different from mutation.
``` rust
fn main() {
    let x = 5;
    let x = x + 1; // Shadowing 'x', creates a new variable with the same name
}
```
+ This is useful for changing the type or value of a variable while keeping the same name.

## Constants
+ Constants are a special kind of variable in Rust whose values cannot be changed.
+ They are declared using the `const` keyword and must have a type annotation.
``` rust
const PI: f32 = 3.14;
```
+ Constants are evaluated at compile-time and can be used in any scope, including global scope.

## Data Types

Rust has a rich set of data types that can be categorized into a few main groups: scalar types, compound types, and user-defined types. Here's an overview of the different data types in Rust:
### Scaler Types
#### Integers:
+ Signed Integers: `i8`, `i16`, `i32`, `i64`, `i128`
+ Unsigned Integers: `u8`, `u16`, `u32`, `u64`, `u128`
+ Example :
``` rust
let signed_integer: i32 = -42;
let unsigned_integer: u64 = 100;
```

#### Floating-Point:
+ Rust has two floating-point types: `f32` and `f64`
+ Example :
``` rust
let signed_integer: i32 = -42;
let unsigned_integer: u64 = 100;
```

#### Boolean:
+ Represented by the `bool` ype with values `true` and `false`
+ Example :
``` rust
let is_rust_cool: bool = true;
```

#### Characters:
+ Denoted by the `char` keyword and the values are written within single quotes.
+ Example :
``` rust
let letter: char = 'A';
```

### Compound Types
#### Tuples:
+ A fixed sized ordered list of elements of different types.
+ Written with parentheses and comma-seperated values.
+ Examples :
``` rust
let tuple: (i32, f64, char) = (42, 3.14, 'A');
```

#### Arrays:
+ A fixed sized array of elements of the same types.
+ Written with square brackets and a specified size.
+ Examples :
``` rust
let array: [i32; 5] = [1, 2, 3, 4, 5];
```

### User-Defined Types
#### Structs:
+ Allow you to define your own data structures with named fields.
+ Examples :
``` rust
struct Person {
    name: String,
    age: u32,
}

let person = Person {
    name: String::from("Alice"),
    age: 30,
};
```

#### Enums:
+ Used to define a type that can have different values, each called a variant.
+ Examples :
``` rust
enum Color {
    Red,
    Green,
    Blue,
}

let color: Color = Color::Green;
```

#### References:
+ Allows you to refer to a value without taking the ownership of it.
+ Written using the `&` symbol.
+ Example :
``` rust
let x = 42;
let reference_to_x: &i32 = &x;
```

#### Pointers:
+ Raw pointers(`const T` and `mut T`) provide more flexibility than references but come with added responsibility.
+ Example :
``` rust
let x = 42;
let raw_pointer: *const i32 = &x;
```
These are the main data types in Rust. Each type serves a specific purpose, and understanding them is crucial for effective Rust programming.

## Concept of Ownership and Borrowing
+ Rust's ownership system ensures memory safety by tracking ownership of variables.
+ When a variable is passed to a function or assigned to another variable, ownership may be transferred or borrowed, ensuring no two parts of the code attempt to modify the same data simultaneously.
+ These concepts contribute to Rust's focus on safety without sacrificing performance. Understanding ownership is particularly important as you delve into more complex programs.

## Conditional Statements
Rust has the `if`, `else if`, and `else` constructs are used from conditional programming and branching. Here's the code snippet to demonstrate the usage:
### Basic `if` Statement:
The basic `if` statement is used to execute a block of code if a condition is true:
``` rust
fn main() {
    let number = 42;

    if number > 0 {
        println!("The number is positive.");
    }
}
```
In this example, the `println!()` statement will be executed only if the `number` is greater than 0.

### `if` with `else`:
You can use the `else` keyword to specify a block of code to be executed when the condition in the `if` statement is false:
``` rust
fn main() {
    let number = -5;

    if number > 0 {
        println!("The number is positive.");
    } else {
        println!("The number is non-positive.");
    }
}
```

Here, if `number` is greater than 0, the first `println!()` statement will be executed; otherwise, the `println!()` statement in the `else` block will be executed.

### `if` with `else if`:
You can chain multiple conditions using `else if` to check multiple cases:
``` rust
fn main() {
    let number = -5;

    if number > 0 {
        println!("The number is positive.");
    } else {
        println!("The number is non-positive.");
    }
}
```

In this example, the code checks whether `number` is positive, negative, or zero, and the appropriate block of code is executed.

### Using `if` in Variable Assignments:
You can use the `if` expression for variable assignments:
``` rust
fn main() {
    let condition = true;
    let number = if condition { 42 } else { -42 };

    println!("The number is: {}", number);
}
```
Here, the value assigned to `number` depends on the condition. `If` condition is true, `number` will be 42; otherwise, it will be -42.

These examples cover the basic usage of `if`, `else if`, and `else` in Rust. Understanding these constructs is fundamental for writing conditional logic in your programs.
