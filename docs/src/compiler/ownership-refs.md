# Ownership and References

**lemon** uses a **safe ownership model** that prevents **memory misuse errors** without needing explicit _lifetimes_ like Rust. This is possible because **function parameters cannot be manually freed (`free`)**, ensuring that all references are always valid.

---

### 1. How ownership works?

In **Lemon**, every value has **a single owner** and can only be **borrowed** in two ways:

- **Immutable Reference (`&self`)** → Allows multiple concurrent reads.
- **Mutable Reference (`&mut self`)** → Allows modification, but **only one mutable reference at a time**.

**What does this mean?**

- No **implicit copies** of large values (e.g., `struct`).
- If a value is passed by reference (`&self` or `&mut self`), it **cannot be freed (`free`) within the function**.
- **This eliminates `use-after-free`, `double-free`, and other issues** without needing explicit _lifetimes_.

---

### 2. Why Doesn’t lemon use lifetimes?

Unlike **Rust**, **Lemon** doesn’t need _lifetimes_ because:

1. **Parameters cannot be manually freed** → If `&self` or `&mut self` exists, it is valid as long as the function runs.
2. **Functions only return ownership explicitly** → If a function takes `&mut`, it **cannot move the value or transfer ownership**.
3. **References never escape their scope** → Since `free` is **not allowed** inside functions, references always point to valid memory.

This simplifies the language **without sacrificing safety**.

---

### 3. How Does the Type Checker Validate Ownership and References?

Before generating **IR**, the compiler ensures:

- **`&mut` is used correctly** → Only one mutable reference can exist at a time.
- **`&` allows multiple reads** → Shared reading is safe as long as no `&mut` exists.
- **Return values preserve ownership correctly** → A function **can only return ownership (`Person`) if it received ownership or created a new object.**

**Invalid Example (Ownership Error)**:

```lemon
fn invalid(p: &mut Person): &Person = {
  return p; // Error! Cannot return &Person while `p` is `&mut`.
}
```

This fails because `&mut` **cannot be shared**.

**Correct Example**:

```lemon
fn valid(p: &Person): i64 = {
  return p.born; // allowed! only reading.
}
```

---

### 4. How Ownership and References Work in Practice?

#### Creating an Object (Ownership)

```lemon
let person = Person::new(21, 2004);  // `person` owns the object
```

#### Immutable Reference (`&self`)

```lemon
let age = person.get_age(); // `&self` → Can be called multiple times
```

#### Mutable Reference (`&mut self`)

```lemon
fn update_born(p: &mut Person, year: i64) = {
  p.born = year; // Can modify `p`, but only one mutable reference is allowed
}
```

---

### 5. summary: How lemon ensures safety without lifetimes?

| **Concept**                    | **How It Works in Lemon?**                           |
| ------------------------------ | ---------------------------------------------------- |
| **ownership**                  | only **one owner** per value.                        |
| **immutable Reference (`&`)**  | allows multiple concurrent reads.                    |
| **mutable Reference (`&mut`)** | only **one active at a time**.                       |
| **no `free` on parameters**    | prevents `use-after-free`.                           |
| **no implicit `move`**         | objects cannot be moved without permission.          |
| **no `lifetimes`**             | because ownership already prevents invalid accesses. |
