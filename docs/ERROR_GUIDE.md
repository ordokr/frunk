# OrdoFP Error Messages & Solutions

## 1. "Cannot find `impl` of trait `Generic`"

### Error
```
error[E0277]: the trait bound `MyStruct: ordofp::Generic` is not satisfied
   --> src/main.rs:5:25
    |
5   | let hlist = ordofp::from_generic(my_struct);
    |                     ^^^^^^^^^^^
```

### Cause
Your struct doesn't derive `Generic` or `LabelledGeneric`.

### Solution
```rust
use ordofp::Generic;

#[derive(Generic)]  // ← Add this
struct MyStruct {
    name: String,
    age: u32,
}
```

### Why
OrdoFP needs to see the struct definition at compile-time to generate conversion code. Derive macros handle this.

---

## 2. "Mismatched types: expected `HList![...]`"

### Error
```
error[E0308]: mismatched types
   --> src/main.rs:10:15
    |
10  | let (name, age, email): (String, u32, String) = from_generic(user);
    | ^^^^^^^^^^^^^^^^^^^^^^^^ expected HList, found tuple
```

### Cause
- Mixing HList and tuple types
- Field order mismatch
- Wrong field types

### Solution (if you want tuples)
```rust
let hlist = ordofp::from_generic(user);
let tuple: (String, u32) = hlist.into();  // ← Use .into()
```

### Solution (if you want different fields)
Use `sculpt()` to reshape:
```rust
#[derive(Generic)]
struct Minimal { name: String, age: u32 }

#[derive(Generic)]
struct Full { name: String, age: u32, email: String }

let full = Full { /* ... */ };
let (minimal, remainder): (HList![String, u32], _) = 
    ordofp::from_generic(full).sculpt();
```

### Why
HLists maintain order and type-safety. Field reordering/filtering requires explicit operations.

---

## 3. "Cannot convert between types with different field names"

### Error
```
error[E0599]: no method named `convert_from` found
   --> src/main.rs:15:20
    |
15  | let api_user: ApiUser = ordofp::convert_from(db_user);
    |                               ^^^^^^^^^^^
```

### Cause
You're trying to convert between types with mismatched field names. `Generic` is lenient, but `LabelledGeneric` requires exact name matches.

### Solution
Use `Generic` (ignores field names, just matches type order):
```rust
#[derive(Generic)]  // ← Not LabelledGeneric
struct ApiUser {
    FirstName: String,  // Note: different naming
    LastName: String,
}

#[derive(Generic)]
struct DbUser {
    first_name: String,  // Note: different naming
    last_name: String,
}

let db_user = DbUser { /* ... */ };
let api_user: ApiUser = ordofp::convert_from(db_user);  // ← Works!
```

Or use `transform_from()` for smart reordering:
```rust
let api_user: ApiUser = ordofp::transform_from(db_user);
```

### Why
- `Generic`: Works by position only
- `LabelledGeneric`: Works by field name (must match exactly)
- `transform_from`: Smart reshaping with type-safety

---

## 4. "Cannot implement type-level computation"

### Error (Type Mismatch in Monoid)
```
error[E0277]: the trait bound `MyStruct: Monoid` is not satisfied
   --> src/main.rs:20:20
    |
20  | let result = combine_all(&structures);
    |                          ^^^^^^^^^^
```

### Cause
Your type doesn't implement `Monoid` or fields don't all implement `Monoid`.

### Solution
```rust
use ordofp::monoid::Monoid;

#[derive(Clone)]
struct Config {
    name: String,          // Strings are Monoid (concat)
    count: u32,            // u32 is Monoid (addition)
    tags: Vec<String>,     // Vecs are Monoid (append)
}

impl Monoid for Config {
    fn empty() -> Self {
        Config {
            name: String::new(),
            count: 0,
            tags: vec![],
        }
    }
    
    fn combine(&self, other: &Self) -> Self {
        Config {
            name: self.name.combine(&other.name),
            count: self.count.combine(&other.count),
            tags: self.tags.combine(&other.tags),
        }
    }
}

let configs = vec![config1, config2, config3];
let merged = ordofp::monoid::combine_all(&configs);  // ← Works!
```

### Why
Monoid requires both an identity (`empty()`) and an associative operation (`combine()`).

---

## 5. "Expected mutable reference in Validated"

### Error
```
error[E0308]: mismatched types
   --> src/main.rs:25:25
    |
25  | let v: Validated = get_name().into_validated() + get_age();
    |                    ^
    | expected `Validated`, found `Result`
```

### Cause
You haven't imported the `Validated` extension trait.

### Solution
```rust
use ordofp::prelude::*;  // ← Imports Validated + extensions

// Now Result can become Validated
let validation = get_name().into_validated() 
                 + get_age() 
                 + get_street();

// Convert back to Result and pattern-match
let person: Result<Person, _> = validation
    .into_result()
    .map(|hlist_pat!(name, age, street)| {
        Person { name, age, street }
    });
```

### Why
`into_validated()` is provided by the `Validated` trait from `ordofp::prelude`.

---

## 6. "Cannot use HList operations on wrong type"

### Error
```
error[E0599]: no method named `pluck` found
   --> src/main.rs:30:25
    |
30  | let (value, rest) = my_hlist.pluck::<String>();
    |                                ^^^^^
    | method not found
```

### Cause
The type you're trying to pluck doesn't exist in the HList.

### Solution
```rust
let hlist = hlist![1, "hello", true, 42f32];

// This works (bool is in the list)
let (b, rest): (bool, _) = hlist.pluck();

// This fails (i64 is NOT in the list)
// let (i, rest): (i64, _) = hlist.pluck();  // ← Compile error!

// Use sculpt() instead to safely extract subset
let (subset, remainder): (HList![f32, i32], _) = hlist.sculpt();
```

### Why
`pluck()` requires the type to exist. `sculpt()` is more forgiving (checks subset at compile-time).

---

## 7. Quick Testing Pattern

### Verify your setup works:
```rust
#[cfg(test)]
mod tests {
    use ordofp::Generic;
    use ordofp::prelude::*;
    
    #[derive(Generic, Debug, PartialEq)]
    struct Point { x: i32, y: i32 }
    
    #[test]
    fn test_generic_roundtrip() {
        let p = Point { x: 1, y: 2 };
        let hlist = ordofp::from_generic(p);
        let p2: Point = ordofp::into_generic(hlist);
        assert_eq!(p, p2);
    }
}
```

---

## Debugging Checklist

- [ ] Did you add `#[derive(Generic)]`?
- [ ] Are field types correct?
- [ ] Are you importing `ordofp::prelude::*`?
- [ ] Did you use `.into()` for type conversions?
- [ ] Are you using the right method (`pluck` vs `sculpt` vs `transform_from`)?
- [ ] Do all struct fields implement required traits (Monoid, etc.)?

---

## When to Use What

| Operation | Use This | Example |
|-----------|----------|---------|
| Convert to HList | `from_generic()` | Struct → HList |
| Convert from HList | `into_generic()` | HList → Struct |
| Rename fields | `LabelledGeneric` + `convert_from()` | API ↔ Domain models |
| Reorder/filter | `sculpt()` | Keep only some fields |
| Smart convert | `transform_from()` | Nested struct conversion |
| Extract one field | `pluck()` | Get bool from `HList![i32, bool, f32]` |
| Extract all matching types | `sculpt()` | Get all strings from mixed HList |
| Combine multiple | `combine_all()` | Merge configs |
| Validate all | `into_validated() + ... + into_result()` | Form validation |

---

## References

- [OrdoFP Docs](https://docs.rs/ordofp/)
- [Examples](https://github.com/lloydmeta/frunk/tree/master/examples)
- [GitHub Issues](https://github.com/lloydmeta/frunk/issues)
