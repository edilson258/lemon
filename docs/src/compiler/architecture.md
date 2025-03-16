### Lemon Compiler Architecture

The **Lemon** compiler follows a **safe ownership model**, ensuring that errors like `use-after-free` and `double-free` are avoided **directly in the IR**. The goal is to allow **low-level, safe programming** without requiring an external _borrow checker_.

---

### 1. Core Concept: Ownership in IR

The idea comes from **Stacked Borrows**, where each value or pointer has **a single owner** and can only be **borrowed** under strict rules:

- `own<T>` → Exclusive ownership of a value.
- `ref<T>` → Immutable reference (multiple reads allowed).
- `mutref<T>` → Mutable reference (only one active at a time).
- `drop` → can only be called when there are no active references.

This ensures **memory safety by design**, with no need for _runtime garbage collection_.

---

### 2. Compiler Pipeline

The **Lemon** compiler consists of the following stages:

1. **Parsing** → Converts source code into an **Abstract Syntax Tree (AST)**.
2. **Semantic Analysis** → Validates types, ownership, and borrows.
3. **Lowering to IR** → Transforms the AST into **Lemon IR**, embedding ownership.
4. **Optimizations** → Removes unnecessary accesses and improves efficiency.
5. **Code Generation** → Converts IR to assembly or bytecode for execution.

Each phase ensures **ownership is preserved**, preventing memory errors.

---

### 3. Safe IR with Ownership

In **Lemon IR**, safety is built-in from the start. Example of a safe IR:

```rs
fn Person.new(r4: i32, r5: i64): own<Person> = {
  l1: r6 = heap 16
      r7 = Person getptr own<Person> r6 0
      set r7, i32 r4
      r8 = Person getptr own<Person> r6 1
      set r8, i64 r5
      ret own<Person> r6
}
```

- Key features of IR:

- `own<T>` → Only one owner at a time.
- `borrow ref<T>` → Read-only, no modifications allowed.
- `mutref<T>` → Only one mutable borrow allowed.
- `release` → Releases a borrow before `drop`.
- `drop` → Only allowed when no active references exist.

This makes **IR inherently safe**, requiring no additional verification.

---

The **Lemon** compiler applies **ownership principles from AST analysis to code generation**, ensuring execution **without memory leaks or invalid accesses**. This approach provides:
✔ **Memory safety with no GC overhead**
✔ **Efficient and predictable execution**
✔ **Low-level safe code, easy to optimize**

- Read abou [Pointer Cache Management](./pointer_cache.md).
