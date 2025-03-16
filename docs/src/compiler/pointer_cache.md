# Pointer Cache Management in Lemon IR(LNR)

The **lnr** implements a **pointer cache** to optimize memory accesses and reduce unnecessary reads. This mechanism replaces direct memory accesses with a local cache when possible, ensuring **efficiency** and **safety**.

---

### 1. Why use a cache?

Normally, accessing a field in memory requires a **direct RAM read**, which can be **inefficient**.

💡 To solve this, **lnr** maintains a **local cache** in each pointer. This cache stores the last known value, avoiding redundant memory reads.

---

### 2. How does the cache work?

Each pointer in **lnr** has a cache field that can be read or written, allowing temporary storage without memory access.

- **Cache rules**:

- **Writing to cache** happens when a value is updated (`set_cache`).
- **Reading from cache** replaces `load` when the value is known to be fresh (`get_cache`).
- If a value may have changed externally, a `load` updates the cache.

This ensures memory is accessed **only when necessary**, improving efficiency.

---

### 3. Usage in IR

#### Storing and accessing cache

```rs
fn Person.set_age(r10: mutref<Person>, r11: i64): void = {
  l1: r12 = Person getptr mutref<Person> r10 1
      set_cache r12, i64 r11  // update cache
      store i64 r11, r12       // update memory
}
```

- `set_cache` writes the value **only to cache**.
- `store` writes the value **to actual memory**.

#### Avoiding unnecessary reads

```rs
fn Person.get_age(r10: ref<Person>): i64 = {
  l1: r11 = Person getptr ref<Person> r10 1
      r12 = get_cache r11     // uses cache if valid
      ret i64 r12
}
```

- `get_cache` returns the cached value **without accessing memory**.

---

### 4. Cache Benefits

- **Fewer memory reads** → Better performance.
- **Avoids redundancy** → Fewer `load` and `store` operations.
- **Maintains safety** → The cache is refreshed with `load` when needed.
