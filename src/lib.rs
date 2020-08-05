#![no_std]

//! Mimic the "truthy" behavior of languages like JavaScript and Python.
//!
//! In other words,
//!
//! ```rust,ignore
//! // rust
//! my_value.truthy();
//! ```
//!
//! Should behave similarly to
//!
//! ```javascript
//! // javascript
//! !!myValue;
//! Boolean(myValue);
//! ```
//! or
//! ```python
//! # python
//! bool(my_value)
//! ```

/// Convert to a `bool`.
pub trait Truthy {
    /// Converts `&self` to a `bool`.
    fn truthy(&self) -> bool;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
