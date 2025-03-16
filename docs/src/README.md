# Lemon Language Reference

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

## Binary operators

Coming soon!

A format string literal may contain the following escape sequences.

- `\n` -> newline
- `\r` -> carriage return
- `\t` -> horizontal tab
- `\\` -> backslash
- `\"` -> double quote
- `\0` -> null
- `\{` -> open curly brace

## Structures

Struct may be defined within any block, including within functions, will inherit any generic type parameters from their definition scope, and their fields are public by default. They take the following forms.

```
// Empty struct, has a byte size of `0`, byte alignment of `1`
type EmptyStruct = {}

// Struct with a single `u64` field
type PlayerId = {
    representation: u64
}

// Struct with two `f64` fields
type DoubleDouble = {
    a: f64
    b: f64
}
```
