# Bounded-integer types focused on ergonomics and safety.
The goal of this crate is to allow users to fearlessly use bounded integers without worrying about checking the operations performed on them. There is one exception: division by zero will still panic.

### Make sure to carefully read this README before using it.
### Since all bounded types behave equaly, this README is the entire documentation.

- No unsafe code.
- All operations are saturated, no overflow/underflow, no wrapping.
- Division by zero still panics.
- Focused on ergonomics(comfyness): operations with different integer types (e.g. i8 and i16) are allowed, those are evaluated using a larger type, then the result is saturated to the target type.
- Operations will always be at least as slow as the std::saturating operations. If you need speed, consider getting a copy of the inner value using `.get()` or the `deref` `*` operator.
- Mostly test-covered. Currently, the derived `Hash`/`PartialOrd`/`Ord` traits are not covered by unit tests.

---

## Features
All features are disabled by default.

- `div_assign_zero`: Enables unchecked division by zero. (Division by non-zero types (NonZeroI8, ...) is provided without this feature).
- `serde`: Enables serde Serialization/Deserialization support, all bounded types are serialized transparently as their inner value. 
  Example: `assert_eq!(serde_json::to_string(&Bound_i8::<0, 10>::new(5)).unwrap(), serde_json::to_string(&5_i8).unwrap())`.

---

# Quirks

## All rust signed/unsigned types are supported: i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize

```rs
use comfy_bounded_ints::{
    Bound_i8, Bound_i16, Bound_i32, Bound_i64, Bound_i128, Bound_isize,
    Bound_u8, Bound_u16, Bound_u32, Bound_u64, Bound_u128, Bound_usize
};
```

## Constraints are enforced at compile-time, you specify them using const params
Use the `new` function to create a new bounded type.
`cargo build` will not compile if MIN > MAX

```rs
use comfy_bounded_ints::Bound_i16;

let bounded = Bound_i16::<20, 50>::new(30);
                          ^^^^^^
```

## This crate guarantees that the inner value will always be within the specified range
No operations can violate this rule.

```rs
use comfy_bounded_ints::Bound_i32;

let bounded = Bound_i32::<20, 50>::new(80);
assert_eq!(bounded.get(), 50);
           ^^^^^^^^^^^^^^^^^

let bounded = Bound_i32::<20, 50>::new(10);
assert_eq!(bounded.get(), 20);
           ^^^^^^^^^^^^^^^^^
```

## You can get a copy of the inner value using the `.get()` method or the `deref` `*` operator

```rs
use comfy_bounded_ints::Bound_i64;

let bounded = Bound_i64::<20, 50>::new(30);
assert_eq!(bounded.get(), 30);
                  ^^^^^^
assert_eq!(*bounded, 30);
           ^
```

## Note that none of the bounded types implement `DerefMut`, as this would allow violating the constraints.
If you want to mutate the inner value, you can use the `.set()` method, or any of the assign operators.

```rs
use comfy_bounded_ints::Bound_isize;

let mut bounded = Bound_isize::<20, 50>::new(30);
bounded.set(40);
       ^^^^^^^^
assert_eq!(bounded.get(), 40);

bounded += 10;
        ^^
assert_eq!(bounded.get(), 50);

bounded.set(10);
       ^^^^^^^^
assert_eq!(bounded.get(), 20);

bounded.set(80);
       ^^^^^^^^
assert_eq!(bounded.get(), 50);

bounded -= 10;
        ^^
assert_eq!(bounded.get(), 40);

bounded /= 2;
        ^^
assert_eq!(bounded.get(), 20);

bounded *= 2;
        ^^
assert_eq!(bounded.get(), 40);

bounded %= 30;
        ^^
assert_eq!(bounded.get(), 20);
```

## `AddAssign`/`SubAssign`/`MulAssign`/`DivAssign`/`RemAssign`
Are allowed between different integer types, as well as bounded types with different constraints.
The regular `Add`/`Sub`/`Mul`/`Div`/`Rem` operations are not supported, use `.get` or `*` `deref` to perform them on the inner value instead.
Support for the regular operations is not provided due to ambiguity in which type should be returned.

```rs
use comfy_bounded_ints::{Bound_i8, Bound_i16, Bound_u8, Bound_u32, Bound_isize};

let mut bounded_i8 = Bound_i8::<20, 50>::new(30);
let bounded_i16 = Bound_i16::< -200, 500>::new(-200);
bounded_i8 += bounded_i16;
           ^^
assert_eq!(bounded_i8.get(), 20);

let mut bounded_i8 = Bound_i8::< -128, 127>::new(-128);
let bounded_i16 = Bound_i16::< -300, 500>::new(-300);
bounded_i8 -= bounded_i16;
           ^^
assert_eq!(bounded_i8.get(), 127);

let mut bounded_u8 = Bound_u8::< 5, 80>::new(70);
let bounded_i16 = Bound_i16::< -300, 500>::new(-1);
bounded_u8 *= bounded_i16;
           ^^
assert_eq!(bounded_u8.get(), 5);

let mut bounded_u32 = Bound_u8::< 2, 650>::new(800);
let bounded_i8 = Bound_i8::< -30, 10>::new(-1);
bounded_u32 /= bounded_i8;
            ^^
assert_eq!(bounded_u32.get(), 2);

let mut bounded_u32 = Bound_u32::< 3, 650>::new(800);
let bounded_i8 = Bound_i8::< -30, 10>::new(-1);
bounded_u32 %= bounded_i8;
            ^^
assert_eq!(bounded_u32.get(), 3);

let mut bounded_isize = Bound_isize::<0, 200>::new(80);
let int_i8 = 30_i8;
bounded_isize += int_i8;
              ^^
assert_eq!(bounded_isize.get(), 110);

let mut bounded_isize = Bound_isize::<0, 200>::new(80);
let int_i32 = -900_i32;
bounded_isize += int_i32;
              ^^
assert_eq!(bounded_isize.get(), 0);

...
```

## The `Default` trait is implemented for all bounded types
It creates a new bounded type with the minimum value as the inner value.

## All bounded types #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, Eq)]

## All bounded types implement DivAssign/RemAssign for their respective NonZero type
This **does not** require the `div_assign_zero` feature.

```rs
use comfy_bounded_ints::{Bound_i8, Bound_u128};

let mut bounded_i8 = Bound_i8::< -30, 80>::new(-120);
let non_zero_i8 = NonZeroI8::new(-2).unwrap();
bounded_i8 *= non_zero_i8;
           ^^
assert_eq!(bounded_i8.get(), 60);

let mut bounded_u128 = Bound_u128::< 3, 650>::new(800);
let non_zero_u128 = NonZeroU128::new(2).unwrap();
bounded_u128 /= non_zero_u128;
             ^^
assert_eq!(bounded_u128.get(), 325);
```

## All bounded types can be created from any integer type using the `From` trait
This trait also provides a blanket implementation allowing any integer to be converted to a given bounded type using the method `.into()`.

```rs
use comfy_bounded_ints::{Bound_i8, Bound_isize, Bound_u8, Bound_u32};

let bounded_i8 = Bound_i8::< -30, 80>::from(-120_i8);
                                       ^^^^^^^^^^^^^
assert_eq!(bounded_i8.get(), -30);

let bounded_u32 = Bound_u32::< 2, 650>::from(-800_i32);
                                        ^^^^^^^^^^^^^^
assert_eq!(bounded_u32.get(), 2);

let bounded_isize: Bound_isize<20, 100> = 50_u128.into();
                                                 ^^^^^^^
assert_eq!(bounded_isize.get(), 50);

let bounded_u8: Bound_u8<20, 100> = -50_i64.into();
                                           ^^^^^^^
assert_eq!(bounded_u8.get(), 20);
```

## PartialEq is implemented between bounded types with the same inner integer type
Constraints are irrelevant for this comparison.
For other comparisons, you can use the `.get()` method or the `deref` `*` operator, then compare using the inner values.

```rs
use comfy_bounded_ints::{Bound_i8, Bound_u16};

let a_i8 = Bound_i8::< -30, 80>::new(-120);
let b_i8 = Bound_i8::< -30, 80>::new(20);
assert!(a_i8 != b_i8);

let a_u16 = Bound_u16::< 2, 650>::new(800);
let b_u16 = Bound_u16::< 2, 650>::new(650);
assert!(a_u16 == b_u16);
```
