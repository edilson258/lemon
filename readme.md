<!--

```rs
let std = import("std");
let fmt = import("fmt");
let http = import("http");

let server = http::server();

server::route("/hello/:name", fn(params) = {
  fn(request, response) = {
    params::method |> match {
      "GET" => fmt::format("Hello, {}!", params::name),
      _     => "Method not allowed"
    } |> response::send;
  }
});

server::start(3003);
```

```rs
let std = import("std");
let fmt = import("fmt");
let http = import("http");

let server = http::server();

server::route("/hello/:name", fn({ params, query }) = {
  let greeting = query::get("greeting")::unwrap_or("");
  fn(request, response) = {
    params::method |> match _ {
      "GET" => {
        let message = fmt::format("{} {}!", greeting, params::name);
        message |> response::send;
      },
      _ => response::send("Method not allowed"),
    }
  }
});

server::start(3003);
```

#### Lemon IR

- lemon

```rs
fn compute(a: u32, b: u32): u32 = {
  let mut sum = a + b;
  if sum > 100 {
    let diff = sum - 50;
    return diff;
  }
  sum
}

fn main(): u32 = {
  let x = 42;
  let y = 58;
  let result = compute(x, y);
  result
}
```

- ir

```rs
fn compute r0: u32, r1: u32 -> u32
l0: add r0, r1 -> r2
l1: cmp_gt r2, 100 -> r3
    jmp_if r3, l2, l3
l2: sub r2, 50 -> r4
    free r2
    ret r4
l3: ret r2

fn main -> u32
l0: own 42 -> r0
    own 58 -> r1
    call compute r0, r1 -> r2
    free r0
    free r1
    ret r2
```

- optimizer steps

1. remove unused variables

```rs
fn compute r0: u32, r1: u32 -> u32
l0: add r0, r1 -> r2
l1: cmp_gt r2, 100 -> r3
    jmp_if r3, l2, l3
l2: sub r2, 50 -> r4
    ret r4
l3: ret r2

fn main -> u32
l0: own 42 -> r0
    own 58 -> r1
    call compute r0, r1 -> r2
    free r0
    free r1
    ret r2

```

2. constant propagation

```rs
fn main -> u32
l0: add 42, 58 -> r0 # inline compute directly in main
l1: cmp_gt r0, 100 -> r1
    jmp_if r1, l2, l3
l2: sub r0, 50 -> r2
    ret r2
l3: ret r0

```

3. dead block elimination

```rs
fn main -> u32
l0: add 42, 58 -> r0 # constant propagation simplifies flow
l1: sub r0, 50 -> r1
    ret r1

```

4. fn inlining

```rs
fn main -> u32
l0: add 42, 58 -> r0 # compute body is directly inlined
l1: sub r0, 50 -> r1
    ret r1

```

5. constant folding

```rs
fn main -> u32
l0: own 50 -> r0 # compute constant values at compile time
    ret r0


```

- compiler

1. llvm
2. wasm
3. lemon runtime (dev mode)

-->
##### WIP
This is highly in progress, so it's accepted and even expected that at any point the main branch may be broken.

#### Versioning strategy

- Start at **0.0.1** and increment the last number for each new feature (e.g., `0.0.2` for adding enums).  
- Increase the middle number (e.g., `0.1.0`) after reaching a stable milestone with multiple improvements.  
- When the language is stable and production-ready, move to **1.0.0**.  
- Use Git tags like `v0.0.1`, `v0.1.0`, and maintain a changelog for tracking updates.  
- No rush for version 1.0 – focus on steady growth and reliability.  
