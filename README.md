# truthy
[![Crates.io](https://img.shields.io/crates/v/truthy)](https://crates.io/crates/truthy)
[![Docs.rs](https://docs.rs/truthy/badge.svg)](https://docs.rs/truthy)
![Crates.io](https://img.shields.io/crates/d/truthy)
[![Build Status](https://travis-ci.com/spenserblack/truthy.svg?branch=master)](https://travis-ci.com/spenserblack/truthy)

Check if a value is "truthy"

In other words,

```rust
// rust
my_value.truthy();
```

Should behave similarly to

```javascript
// javascript
!!myValue;
Boolean(myValue);
```
or
```python
# python
bool(my_value)
```

## Behavior
```rust
// non-zero numbers are truthy
0u32.truthy() // false
0f32.truthy() // false
1u32.truthy() // true
1f32.truthy() // true

// empty strings are not truthy
"".truthy() // false
" ".truthy() // true

// Options are truthy if not None and their value is truthy
let none: Option<()> = None;
let falsy_inner = Some(false);
let truthy_inner = Some(true);
none.truthy() // false
falsy_inner.truthy() // false
truthy_inner.truthy() // true

// Results are truthy if Ok and value is truthy
let falsy_err: Result<(), _> = Err(false);
let truthy_err: Result<(), _> = Err(true);
let falsy_ok: Result<_, ()> = Ok(false);
let truthy_ok: Result<_, ()> = Ok(true);

falsy_err.truthy() // false
truthy_err.truthy() // false
falsy_ok.truthy() // false
truthy_ok.truthy() // true

// Empty vecs and arrays are falsy
let empty_array: [();0] = [];
let empty_vec: Vec<()> = Vec::new();

empty_array.truthy() // false
empty_vec.truthy() // false

// The truthy behavior of arrays and vecs also applies to tuples from size 0 to 12
let empty_tuple = ();
let not_empty_tuple = (1, "2", '3');
empty_tuple.truthy() // false
not_empty_tuple.truthy() // true
```
