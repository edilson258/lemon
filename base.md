
## **1. Introduction**

This language prioritizes minimalism, safety, and efficiency by combining:

- **Compute-Time**: Robust support for compile-time computation, including dedicated syntax for values and functions.
- **Memory Safety**: Built on *ownership* and *borrowing*, with verification performed at the IR level.
- **Simplicity**: Clear syntax and rules to eliminate ambiguity while remaining powerful.

---

## **2. Declaration Types**

### **2.1 Runtime Declarations (`let`)**
- Used for values defined during runtime.
- Variables can be mutable or immutable:
    ```rust
    let x = 10;           // Immutable
    let mut y = 20;       // Mutable
    y = y + 5;
    ```

### **2.2 Compile-Time Declarations (`const`)**
- Used for values and functions resolved during compilation.
- Always immutable:
    ```rust
    const PI = 3.141592653589793;
    ```

---

## **3. Functions**

### **3.1 Runtime Functions**
- Declared for runtime logic:
    ```rust
    fn add(a: Int, b: Int): Int = {
        return a + b;
    };
    ```

### **3.2 Compile-Time Functions**
- Always anonymous and declared with `const`:
    ```rust
    const FACTORIAL = fn(n: Int) = {
        if n <= 1 {
            return 1;
        }
        return n * FACTORIAL(n - 1);
    };

    const FACT_5 = FACTORIAL(5);  // Computed at compile-time
    ```
- Restrictions:
  - Can only access other `const` values or functions.
  - Must include deterministic structures like `if` and `for`.

---

## **4. Memory Management**

### **4.1 Manual Allocation**
  ```rust
    let mem = import("mem");
    let buffer = mem::allocate(1024);
    mem::free(buffer);
  ```

### **4.2 Ownership and Borrowing**
  ```rust
    let owner = mem::allocate(50);
    let borrow = mem::borrow_mut(owner);
    *borrow = 100;                 // Modify via borrow
    mem::return_borrow(borrow);    // Return ownership
    mem::free(owner);
  ```

---

## **5. Compute-Time Details**

### **5.1 Values**
```rust
const SIZE = 10 * 2;
    let buffer = mem::allocate(SIZE);

```
### **5.2 Anonymous Functions**
```rust
    const SQUARE = fn(x: Int) = {
        return x * x;
    };

    const SQUARE_5 = SQUARE(5);  // Computed at compile-time
```

### **5.3 Precomputed Tables**
  ```rust
    const SIN_TABLE = fn() = {
        let table = [];
        for i in 0..360 {
            table.push(math::sin(i * math::PI / 180));
        }
        return table;
    };

    const SIN = SIN_TABLE();
    let sin_45 = SIN[45];  // Accessed during runtime
```


## **6. Intermediate Representation**

### **6.1 Computed Declarations**
```text
    const PI = 3.141592653589793
    const FACT_5 = 120
```

### **6.2 Compute-Time Functions**
```text
    const SQUARE = fn(x: Int): Int = {
        return x * x;
    };
    const SQUARE_5 = 25;
```

### **6.3 Memory Management**
```text
    entry:
        p0, m0 = mk_own 50, mem_init
        p1 = mut_mkbor p0
        store 100, p1, m0
        die p1
        v0 = load p0, m0
        assert v0 == 100
        free p0
        halt;
```


## **7. Macros**

### **7.1 Syntax**
```rust
    const MACRO_NAME = macro(params) {
        // Transform code at compile-time
    };
```

### **7.2 Examples**

#### Substitution
```rust
    const ASSERT_EQ = macro(a, b) {
        return `assert(${a} == ${b});`;
    };

    fn main() {
        ASSERT_EQ(5, 2 + 3);  // Expands to: assert(5 == 2 + 3);
    }
```

#### Code Generation
```rust
    const LOOP_UNROLL = macro(var, start, end, body) {
        let result = [];
        for i in start..end {
            result.push(`${var} = ${i}; ${body}`);
        }
        return result.join(" ");
    };

    fn main() {
        LOOP_UNROLL(i, 0, 3, `println(i);`);
        // Expands to:
        // i = 0; println(i);
        // i = 1; println(i);
        // i = 2; println(i);
    }
```


## **8. Restrictions**

### **8.1 Compute-Time Errors**
1. Using runtime variables in `const`:
```rust
    const INVALID = fn(x: Int) = {
        return x + runtime_value;  // Error: runtime_value is not const
    };
```

2. Infinite loops:
```rust
    const INFINITE_LOOP = fn() = {
        while true { }  // Error: Infinite execution in compile-time
    };
```

---

## **9. Key Advantages**

- **Minimal and Powerful**: Separation of `const` and `let` avoids ambiguity and enhances clarity.
- **Optimized Execution**: Compile-time computations reduce runtime overhead.
- **Consistent Design**: Macros and compute-time seamlessly integrate into the language.

