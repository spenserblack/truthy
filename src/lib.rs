//! Mimic the "truthy" behavior of languages like JavaScript and Python.
//!
//! In other words,
//!
//! ```
//! # use truthy::Truthy;
//! # let my_value = true;
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
//!
//! enable the `and-or` feature to get access to `truthy_and` and `truthy_or`.
//!
//! # Behavior
//! ```
//! # use truthy::Truthy;
//! let truthy_option = Some(1u32);
//! let falsy_options = [None, Some(0u32)];
//!
//! assert!(truthy_option.truthy());
//! for falsy_option in &falsy_options {
//!     assert!(falsy_option.falsy());
//! }
//!
//! let truthy_result: Result<_, ()> = Ok(1u32);
//! let falsy_results = [Ok(false), Err(false), Err(true)];
//!
//! assert!(truthy_result.truthy());
//! for falsy_result in &falsy_results {
//!     assert!(falsy_result.falsy());
//! }
//!
//! let not_empty = vec![()];
//! let empty: [();0] = [];
//!
//! assert!(not_empty.truthy());
//! assert!(empty.falsy());
//! ```
//!
//! # Example Usage
//!
//! ```
//! use truthy::Truthy;
//!
//! fn do_something<T: Truthy>(value: T) {
//!     if value.truthy() {
//!         println!("It's truthy :)");
//!     }
//! }
//! ```
#[cfg(feature = "and-or")]
use and_or::{Either, Left, Right};

/// Convert to a `bool`.
pub trait Truthy {
    /// Converts `&self` to a `bool`.
    fn truthy(&self) -> bool;
    fn falsy(&self) -> bool {
        !self.truthy()
    }
    #[cfg(feature = "and-or")]
    /// `Left(self)` if `self` is truthy, else `Right(other)`
    ///
    /// ```rust
    /// # use truthy::Truthy;
    /// assert_eq!(true.truthy_or('t').left(), Some(true));
    /// assert_eq!(false.truthy_or('t').right(), Some('t'));
    /// ```
    fn truthy_or<T>(self, other: T) -> Either<Self, T> where Self: Sized {
        if self.truthy() {
            Left(self)
        } else {
            Right(other)
        }
    }
    #[cfg(feature = "and-or")]
    /// `Some(other)` if `self` is truthy, else `None`
    ///
    /// ```rust
    /// # use truthy::Truthy;
    /// assert_eq!(true.truthy_and('t'), Some('t'));
    /// assert_eq!(false.truthy_and('t'), None);
    /// ```
    fn truthy_and<T>(self, other: T) -> Option<T> where Self: Sized {
        if self.truthy() {
            Some(other)
        } else {
            None
        }
    }
}

macro_rules! impl_truthy_num {
    ($type:ty) => {
        impl $crate::Truthy for $type {
            fn truthy(&self) -> bool {
                let falsy: $type = 0;
                !self.eq(&falsy)
            }
        }
    };
}

macro_rules! impl_truthy_tuple {
    ($($G:ident),+) => {
        impl<$($G),+> $crate::Truthy for ($($G),+,) {
            fn truthy(&self) -> bool {
                true
            }
        }
    }
}

impl_truthy_num!(i8);
impl_truthy_num!(i16);
impl_truthy_num!(i32);
impl_truthy_num!(i64);
impl_truthy_num!(i128);
impl_truthy_num!(isize);
impl_truthy_num!(u8);
impl_truthy_num!(u16);
impl_truthy_num!(u32);
impl_truthy_num!(u64);
impl_truthy_num!(u128);
impl_truthy_num!(usize);

impl_truthy_tuple! {T1}
impl_truthy_tuple! {T1, T2}
impl_truthy_tuple! {T1, T2, T3}
impl_truthy_tuple! {T1, T2, T3, T4}
impl_truthy_tuple! {T1, T2, T3, T4, T5}
impl_truthy_tuple! {T1, T2, T3, T4, T5, T6}
impl_truthy_tuple! {T1, T2, T3, T4, T5, T6, T7}
impl_truthy_tuple! {T1, T2, T3, T4, T5, T6, T7, T8}
impl_truthy_tuple! {T1, T2, T3, T4, T5, T6, T7, T8, T9}
impl_truthy_tuple! {T1, T2, T3, T4, T5, T6, T7, T8, T9, T10}
impl_truthy_tuple! {T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11}
impl_truthy_tuple! {T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12}

impl Truthy for bool {
    fn truthy(&self) -> bool {
        *self
    }
}

impl Truthy for f32 {
    fn truthy(&self) -> bool {
        !self.eq(&0f32)
    }
}

impl Truthy for f64 {
    fn truthy(&self) -> bool {
        !self.eq(&0f64)
    }
}

impl Truthy for () {
    fn truthy(&self) -> bool {
        false
    }
}

impl Truthy for &str {
    fn truthy(&self) -> bool {
        !self.is_empty()
    }
}

impl<T> Truthy for Option<T> where T: Truthy {
    fn truthy(&self) -> bool {
        if let Some(v) = self {
            v.truthy()
        } else {
            false
        }
    }
}

impl<T, E> Truthy for Result<T, E> where T: Truthy {
    fn truthy(&self) -> bool {
        if let Ok(v) = self {
            v.truthy()
        } else {
            false
        }
    }
}

impl<T> Truthy for [T] {
    fn truthy(&self) -> bool {
        self.len() > 0
    }
}

#[cfg(test)]
mod tests {
    use super::Truthy;

    #[test]
    fn truthy_bool() {
        assert!(true.truthy());
    }

    #[test]
    fn falsy_bool() {
        assert!(false.falsy());
    }
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
                assert!((1i64).truthy())
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
                assert!((1i128).truthy())
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
                assert!((1isize).truthy())
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
                assert!((1u8).truthy())
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
            let empty_vec: Vec<()> = Vec::new();

            assert!(empty_vec.falsy())
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
            let empty_array: [();0] = [];

            assert!(empty_array.falsy())
        }
    }
    mod options {
        use super::Truthy;

        #[test]
        fn truthy_inner() {
            assert!(Some("I'm here!").truthy())
        }

        #[test]
        fn falsy() {
            let none: Option<()> = None;

            assert!(!none.truthy())
        }

        #[test]
        fn falsy_inner() {
            assert!(Some(()).falsy())
        }
    }
    mod results {
        use super::Truthy;

        #[test]
        fn truthy_inner() {
            let ok: Result<_, ()> = Ok(":)");

            assert!(ok.truthy())
        }

        #[test]
        fn falsy() {
            let err: Result<(), _> = Err(":(");

            assert!(!err.truthy())
        }

        #[test]
        fn falsy_inner() {
            let ok: Result<_, ()> = Ok(());

            assert!(ok.falsy())
        }
    }
    mod tuples {
        use super::Truthy;

        #[test]
        fn truthy() {
            assert!((1, "2", '3').truthy())
        }

        #[test]
        fn falsy() {
            assert!(!().truthy())
        }
    }
}
