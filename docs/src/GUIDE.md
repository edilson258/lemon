# **Lemon in X minutes – The Ultimate Guide!**

Lemon is a **low-level, safe programming language**. It aims to be **as fast as C** and **as safe as Rust**, but with a **simpler syntax**. No garbage collector, no `use-after-free`, no `double-free` – **everything is prevented by design**.

Lemon has **built-in ownership**, which means the language itself **prevents memory errors before compilation**. If you've used C or Rust, it will feel familiar. If not, don't worry – this guide will teach you everything you need to get started!

If you want a **deep technical dive**, check out the [The Lemon Compiler](./compiler/architecture.md). But if you just want to **start coding right now**, this guide is for you. Let’s go!

---

## Installation

### Install dependencies

#### On Linux

```sh
# Install Rust if you haven't it already.
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# make sure you have libc installed, we use gcc
sudo apt install gcc
```

#### On Mac

```sh
# Install Rust if you haven't it already.
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# make sure you have libc installed, probably we don't need to install gcc
brew install gcc

```

Also, you can install Lemon by running:

```
curl -fsSL https://lemonlang.org/install | sh

```

### On Windows

- [Install Rust](https://forge.rust-lang.org/infra/other-installation-methods.html#other-ways-to-install-rustup)

### Install Lemon

```sh
cargo install lemonlang

lemon --version
```

## **Hello, World!**

Lemon is **low-level**, but its syntax is **easy to read**. Here’s a `"Hello, world!"` program:

```rs

extern fn printf(fmt: str, ...) = {}

fn println(value: str) = {
  printf("%s\n", value); // use libc
}


fn main() = {
  println("Hello, world!");
}
```

To run it:

```sh
lemon compile hello.ln && ./hello
```

Simple, right? Now, let’s learn the basics!

---

## **Types and Functions**

Lemon **has static types**, but you **don’t always need to write them** because the compiler **can infer them**.

### **Basic function example**

```rs
fn sum(a: i32, b: i32): i32 = {
  return a + b;
}
```

You can call this function like this:

```rs
let result = sum(5, 7);
println(result); // Output: 12
```

## Primitive types

| Type    | Size (bytes)         | Alignment (bytes) | Description                                                  |
| ------- | -------------------- | ----------------- | ------------------------------------------------------------ |
| `void`  | 0                    | 1                 | Zero sized unit type, default function return type           |
| `i8`    | 1                    | 1                 | Signed 8 bit integer                                         |
| `i16`   | 2                    | 2                 | Signed 16 bit integer                                        |
| `i32`   | 4                    | 4                 | Signed 32 bit integer                                        |
| `i64`   | 8                    | 8                 | Signed 64 bit integer                                        |
| `u8`    | 1                    | 1                 | Unsigned 8 bit integer                                       |
| `u16`   | 2                    | 2                 | Unsigned 16 bit integer                                      |
| `u32`   | 4                    | 4                 | Unsigned 32 bit integer                                      |
| `u64`   | 8                    | 8                 | Unsigned 64 bit integer                                      |
| `isize` | Same as a pointer    | Same as a pointer | Signed pointer sized integer                                 |
| `usize` | Same as a pointer    | Same as a pointer | Unsigned pointer sized integer                               |
| `f32`   | 4                    | 4                 | 32 bit floating point number                                 |
| `f64`   | 8                    | 8                 | 64 bit floating point number                                 |
| `bool`  | 1                    | 1                 | Boolean value type, `true` or `false`                        |
| `str`   | Same as two pointers | Same as a pointer | String slice, has `pointer` and `length`                     |
| `fstr`  | Same as two pointers | Same as a pointer | Format string slice, describes a string formatting operation |

## Pointer types

| Type     | Size (bytes)  | Alignment (bytes) | Description                                                                     |
| -------- | ------------- | ----------------- | ------------------------------------------------------------------------------- |
| `&T`     | 8 (on 64-bit) | 8 (on 64-bit)     | Pointer to value of type `T` which _cannot_ be used to mutate the pointed value |
| `&mut T` | 8 (on 64-bit) | 8 (on 64-bit)     | Pointer to value of type `T` which _can_ be used to mutate the pointed value    |

## Primitive keyword values

| Keyword | Description                 |
| ------- | --------------------------- |
| `void`  | Unit value of type `void`   |
| `true`  | Truthy value of type `bool` |
| `false` | Falsy value of type `bool`  |

---

## **Structs**

Structs allow you to **group data together**:

```rs
type Person = {
  name: str,
  age: i32,
}
```

To create and use structs:

```rs
let p = Person { name: "Alice", age: 25 };
println(p.name);
println(p.age);
```

And you can add **methods** inside `impl`:

```rs
impl Person = {
  fn new(name: str, age: i32): Person = {
    return Person { name, age };
  }

  fn grow_older(&mut self) = {
    self.age += 1;
  }
}
```

---

## **Ownership and Borrowing**

Lemon **has no garbage collector**, but also **prevents `use-after-free` automatically**.
This works because **the compiler tracks who owns what**.

### **Ownership**

Every value in Lemon **has a single owner**:

```rs
fn main() = {
  let name = "Lemon"; // `name` owns the string
  println(name);       // Ok!
}
```

If we try to use a value **after moving it**, we get an error:

```rs
fn main() = {
  let name = "Lemon";
  let another = name; // `name` is now INVALID
  println(name); // error! `name` no longer exists
}
```

### **Borrowing**

To avoid moving ownership, we can **borrow** the value (`&`):

```rs
fn print_name(&self name: str) = {
  println(name);
}

fn main() = {
  let name = "Lemon";
  print_name(&name); // Ok! `name` is still valid
  println(name);     // still works!
}
```

If we need to modify a borrowed value, we use `&mut`:

```rs
fn increase(age: &mut i32) = {
  age += 1;
}

fn main() = {
  let mut age = 20;
  increase(&mut age);
  println(age); // output: 21
}
```

**Borrowing rules**:

1.  You can have **many immutable references (`&T`)** at the same time.
2.  You can have **only ONE mutable reference (`&mut T`)** at a time.

This prevents **data races and undefined behavior**.

---

## **Control Flow**

Lemon has `if`, `while`, and `match` statements.

### **If / Else**

```rs
fn check_age(age: i32) = {
  if (age >= 18) {
    println("Adult");
  } else {
    println("Minor");
  }
}
```

```rs
fn check_age(age: i32) = {
  if (age >= 18) {
    println("Adult");
    return;
  }
  println("Minor");
}
```

### Other Control Flow

Coming soon!

### **Match (Pattern Matching)**

Coming soon!

---

#### Some examples

```rs
fn max(a: i32, b: i32): i32 = {
  if (a > b) return a;
  return b;
}

```

```rs

fn factorial(n: i32): i32 = {
  if (n == 0) return 1;
  return n * factorial(n - 1);
}

```

```rs

 fn fibonacci(n: i32): i32 = {
  if (n <= 1) return n;
  return fibonacci(n - 1) + fibonacci(n - 2);
}
```

## **What's Next?**

Lemon is **actively being developed**, and there’s **a lot to improve**!

---

- See [The Lemon Compiler](./compiler/architecture.md) for more information about the compiler.
