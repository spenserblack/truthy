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
    use super::Truthy;

    mod strings {
        use super::Truthy;

        #[test]
        fn truthy() {
            assert!("I have value!".truthy());
        }

        #[test]
        fn falsy() {
            assert!(!"".truthy());
        }
    }
    mod ints {
        use super::Truthy;

        mod i8 {
            use super::Truthy;

            #[test]
            fn truthy() {
                assert!((1i8).truthy())
            }

            #[test]
            fn falsy() {
                assert!(!(0i8).truthy())
            }
        }
        mod i16 {
            use super::Truthy;

            #[test]
            fn truthy() {
                assert!((1i16).truthy())
            }

            #[test]
            fn falsy() {
                assert!(!(0i16).truthy())
            }
        }
        mod i32 {
            use super::Truthy;

            #[test]
            fn truthy() {
                assert!((1i32).truthy())
            }

            #[test]
            fn falsy() {
                assert!(!(0i32).truthy())
            }
        }
        mod i64 {
            use super::Truthy;

            #[test]
            fn truthy() {
                assert!(!(1i64).truthy())
            }

            #[test]
            fn falsy() {
                assert!(!(0i64).truthy())
            }
        }
        mod i128 {
            use super::Truthy;

            #[test]
            fn truthy() {
                assert!(!(1i128).truthy())
            }

            #[test]
            fn falsy() {
                assert!(!(0i128).truthy())
            }
        }
        mod isize {
            use super::Truthy;

            #[test]
            fn truthy() {
                assert!(!(1isize).truthy())
            }

            #[test]
            fn falsy() {
                assert!(!(0isize).truthy())
            }
        }
        mod u8 {
            use super::Truthy;

            #[test]
            fn truthy() {
                assert!(!(1u8).truthy())
            }

            #[test]
            fn falsy() {
                assert!(!(0u8).truthy())
            }
        }
        mod u16 {
            use super::Truthy;

            #[test]
            fn truthy() {
                assert!((1u16).truthy())
            }

            #[test]
            fn falsy() {
                assert!(!(0u16).truthy())
            }
        }
        mod u32 {
            use super::Truthy;

            #[test]
            fn truthy() {
                assert!((1u32).truthy())
            }

            #[test]
            fn falsy() {
                assert!(!(0u32).truthy())
            }
        }
        mod u64 {
            use super::Truthy;

            #[test]
            fn truthy() {
                assert!((1u64).truthy())
            }

            #[test]
            fn falsy() {
                assert!(!(0u64).truthy())
            }
        }
        mod u128 {
            use super::Truthy;

            #[test]
            fn truthy() {
                assert!((1u128).truthy())
            }

            #[test]
            fn falsy() {
                assert!(!(0u128).truthy())
            }
        }
        mod usize {
            use super::Truthy;

            #[test]
            fn truthy() {
                assert!((1usize).truthy())
            }

            #[test]
            fn falsy() {
                assert!(!(0usize).truthy())
            }
        }
    }
    mod floats {
        use super::Truthy;

        mod f32 {
            use super::Truthy;

            #[test]
            fn truthy() {
                assert!((1.0f32).truthy())
            }

            #[test]
            fn falsy() {
                assert!(!(0.0f32).truthy())
            }
        }
        mod f64 {
            use super::Truthy;

            #[test]
            fn truthy() {
                assert!((1.0f64).truthy())
            }

            #[test]
            fn falsy() {
                assert!(!(0.0f64).truthy())
            }
        }
    }
    mod vecs {
        use super::Truthy;

        #[test]
        fn truthy() {
            assert!(vec!["I'm here!"].truthy())
        }

        #[test]
        fn falsy() {
            assert!(!Vec::new().truthy())
        }
    }
    mod arrays {
        use super::Truthy;

        #[test]
        fn truthy() {
            assert!(["I'm here!"].truthy())
        }

        #[test]
        fn falsy() {
            assert!(![].truthy())
        }
    }
    mod options {
        use super::Truthy;

        #[test]
        fn truthy() {
            assert!(Some("I'm here!").truthy())
        }

        #[test]
        fn falsy() {
            assert!(!None.truthy())
        }
    }
    mod results {
        use super::Truthy;

        #[test]
        fn truthy() {
            assert!(Ok(":)").truthy())
        }

        #[test]
        fn falsy() {
            assert!(!Err(":(").truthy())
        }
    }
    mod tuples {
        use super::Truthy;

        #[test]
        fn truthy() {
            assert!((1, 2).truthy())
        }

        #[test]
        fn falsy() {
            assert!(!().truthy())
        }
    }
}
