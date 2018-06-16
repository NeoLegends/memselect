//! No-std compatible memoizing selector composer library.
//!
//! Memselect allows you to create efficient selectors for memoizing expensive
//! computations. The selectors can be composed to create higher-level selectors
//! that benefit from memoization all the way down. Monomorphization ensures
//! efficient runtime behavior.
//!
//! ## Example
//! ```rust
//! use memselect::{new1, new2, Selector2};
//!
//! let mut computations = 0;
//!
//! {
//!     let base = new1(|num: u32| num, |num| num * 2);
//!
//!     let mut selector = new2(
//!         base, // You can nest selectors
//!         |num: u32| num * 3,
//!         |num1, num2| { // This function gets the output of `base` and the fn above
//!             computations += 1;
//!             (*num1, *num2)
//!         },
//!     );
//!
//!     assert_eq!(selector.select(2, 3), (4, 9));
//!     assert_eq!(selector.select(2, 3), (4, 9));
//! }
//!
//! // Value was computed only once
//! assert_eq!(computations, 1);
//! ```
//!
//! This library is heavily inspired by `reselect` for redux.

#![no_std]

mod select1;
mod select2;
mod select3;

pub use select1::{new as new1};
pub use select2::{new as new2};
pub use select3::{new as new3};

/// A selector accepting a single parameter.
pub trait Selector1<A> {
    /// The type of the computed value.
    type Output;

    /// Computes the value based on argument arg1.
    fn select(&mut self, arg1: A) -> Self::Output;
}

/// A selector accepting a two parameters.
pub trait Selector2<A1, A2> {
    type Output;

    /// Computes the value based on arguments arg1 and arg2.
    fn select(&mut self, arg1: A1, arg2: A2) -> Self::Output;
}

/// A selector accepting three parameters.
pub trait Selector3<A1, A2, A3> {
    type Output;

    /// Computes the value based on arguments arg1, arg2 and arg3.
    fn select(&mut self, arg1: A1, arg2: A2, arg3: A3) -> Self::Output;
}

impl<A1, R, T: FnMut(A1) -> R> Selector1<A1> for T {
    type Output = R;

    #[inline]
    fn select(&mut self, arg1: A1) -> Self::Output {
        (self)(arg1)
    }
}

impl<A1, A2, R, T: FnMut(A1, A2) -> R> Selector2<A1, A2> for T {
    type Output = R;

    #[inline]
    fn select(&mut self, arg1: A1, arg2: A2) -> Self::Output {
        (self)(arg1, arg2)
    }
}

impl<A1, A2, A3, R, T: FnMut(A1, A2, A3) -> R> Selector3<A1, A2, A3> for T {
    type Output = R;

    #[inline]
    fn select(&mut self, arg1: A1, arg2: A2, arg3: A3) -> Self::Output {
        (self)(arg1, arg2, arg3)
    }
}
