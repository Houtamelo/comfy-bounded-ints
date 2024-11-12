# Warning: This specific branch requires the Unchained version of Rust.

# Bounded-integer types focused on ergonomics and safety.
The goal of this crate is to allow users to fearlessly use bounded integers without worrying about checking the operations performed on them.

There is one exception: division by zero will still panic.

- No unsafe code.
- All operations are saturated, no overflow/underflow, no wrapping.
- Division by zero still panics.
- Focused on ergonomics(comfyness): operations with different integer types (e.g. i8 and i16) are allowed, with the result being the larger type. 
  type.
- Operations will always be at least as slow as the std::saturating operations. If you need speed, consider getting a copy of the inner value using `.get()` or the `deref` `*` operator.
- Mostly test-covered.

---

## Features
All features are disabled by default.
- `serde`: Enables serde Serialization/Deserialization support, all bounded types are serialized transparently as their inner value.

---
