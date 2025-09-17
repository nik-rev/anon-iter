//! `anon_iter` is a much lighter alternative to the [`auto_enums`](https://crates.io/crates/auto_enums) crate,
//! being less general-purpose but solving the most common use-case of this pattern (`impl Iterator`),
//! without relying on macros - leading to much faster compile times and simpler syntax.
//!
//! It does this by providing generic wrapper types like [`AnonIter2`]
//! to allow to return different types of iterators
//! from a function that returns `-> impl Iterator`.
//!
//! # Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! anon_iter = "0.1"
//! ```
//!
//! Wrap each iterator in [`AnonIter2`] to return 2 different iterators from the same function:
//!
//! ```rust
//! use anon_iter::AnonIter2;
//!
//! fn foo(x: i32) -> impl Iterator<Item = i32> {
//!     match x {
//!         0 => AnonIter2::I1(1..10),
//!         _ => AnonIter2::I2(vec![5, 10].into_iter()),
//!     }
//! }
//! ```
//!
//! The crate [`Either`](https://docs.rs/either/latest/either/) allows similar functionality, as it too implements `Iterator` when
//! its type parameters are both `Iterator`.
//!
//! But this breaks down when you want to return 3 or more `Iterator`s because you now have to
//! do extra nesting (e.g. `Either::Left(Either::Right(Either::Left(my_iter)))`). With `anon_iter`, it's just `AnonIter8::I3(my_iter)`.
//!
//! Additionally, `anon_iter` makes code more readable because it may not be instantly obvious that we are using `Either` for this purpose, but with `AnonEnum`
//! the intent is apparent.
#![no_std]

use core::iter::{DoubleEndedIterator, ExactSizeIterator, FusedIterator};

macro_rules! create {
    ($count:literal, $AnonIter:ident, $($Variant:ident: $n:literal)*) => {
        #[doc = concat!("Wraps ", $count, " `impl Iterator`s which may be of different types")]
        ///
        /// Functions returning `-> impl Iterator` must have the same return type
        /// from all branches, but this is overly restrictive.
        ///
        /// We may want different iterators from the same function,
        /// and this type allows that by wrapping each unique iterator in a variant of
        /// this enum.
        ///
        /// See the [crate-level](crate) documentation for more info.
        pub enum $AnonIter<T, $($Variant,)*>
        where
            $($Variant: Iterator<Item = T>,)*
        {
            $(
                #[doc = concat!("The ", $n, " `impl Iterator`")]
                $Variant($Variant),
            )*
        }

        #[allow(non_snake_case)]
        impl<T, $($Variant: Iterator<Item = T>,)*> Iterator for $AnonIter<T, $($Variant,)*>
        {
            type Item = T;

            fn next(&mut self) -> Option<Self::Item> {
                match self {
                    $(
                        Self::$Variant($Variant) => $Variant.next(),
                    )*
                }
            }
        }

        #[allow(non_snake_case)]
        impl<T, $($Variant: DoubleEndedIterator<Item = T>,)*> DoubleEndedIterator for $AnonIter<T, $($Variant,)*>
        {
            fn next_back(&mut self) -> Option<Self::Item> {
                match self {
                    $(
                        Self::$Variant($Variant) => $Variant.next_back(),
                    )*
                }
            }
        }

        impl<T, $($Variant: FusedIterator<Item = T>,)*> FusedIterator for $AnonIter<T, $($Variant,)*> {}

        #[allow(non_snake_case)]
        impl<T, $($Variant: ExactSizeIterator<Item = T>,)*> ExactSizeIterator for $AnonIter<T, $($Variant,)*>
        {
            fn len(&self) -> usize {
                match self {
                    $(
                        Self::$Variant($Variant) => $Variant.len(),
                    )*
                }
            }
        }
    };
}

create!(2, AnonIter2, I1: "1st" I2: "2nd");
create!(3, AnonIter3, I1: "1st" I2: "2nd" I3: "3rd");
create!(4, AnonIter4, I1: "1st" I2: "2nd" I3: "3rd" I4: "4th");
create!(5, AnonIter5, I1: "1st" I2: "2nd" I3: "3rd" I4: "4th" I5: "5th");
create!(6, AnonIter6, I1: "1st" I2: "2nd" I3: "3rd" I4: "4th" I5: "5th" I6: "6th");
create!(7, AnonIter7, I1: "1st" I2: "2nd" I3: "3rd" I4: "4th" I5: "5th" I6: "6th" I7: "7th");
create!(8, AnonIter8, I1: "1st" I2: "2nd" I3: "3rd" I4: "4th" I5: "5th" I6: "6th" I7: "7th" I8: "8th");
create!(9, AnonIter9, I1: "1st" I2: "2nd" I3: "3rd" I4: "4th" I5: "5th" I6: "6th" I7: "7th" I8: "8th" I9: "9th");
create!(10, AnonIter10, I1: "1st" I2: "2nd" I3: "3rd" I4: "4th" I5: "5th" I6: "6th" I7: "7th" I8: "8th" I9: "9th" I10: "10th");
create!(11, AnonIter11, I1: "1st" I2: "2nd" I3: "3rd" I4: "4th" I5: "5th" I6: "6th" I7: "7th" I8: "8th" I9: "9th" I10: "10th" I11: "11th");
create!(12, AnonIter12, I1: "1st" I2: "2nd" I3: "3rd" I4: "4th" I5: "5th" I6: "6th" I7: "7th" I8: "8th" I9: "9th" I10: "10th" I11: "11th" I12: "12th");
