# anon_iter

[![crates.io](https://img.shields.io/crates/v/anon_iter?style=flat-square&logo=rust)](https://crates.io/crates/anon_iter)
[![docs.rs](https://img.shields.io/badge/docs.rs-auto__enums-blue?style=flat-square&logo=docs.rs)](https://docs.rs/anon_iter)
[![license](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue?style=flat-square)](#license)
[![msrv](https://img.shields.io/badge/msrv-1.50-blue?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![githbu](https://img.shields.io/github/stars/nik-rev/anon_iter)]

`anon_iter` is a much lighter alternative to the [`auto_enums`](https://crates.io/crates/auto_enums) crate,
being less general-purpose but solving the most common use-case of this pattern (`impl Iterator`),
without relying on macros - leading to much faster compile times and simpler syntax.

It does this by providing generic wrapper types like [`AnonIter2`]
to allow to return different types of iterators
from a function that returns `-> impl Iterator`.

# Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
anon_iter = "0.1"
```

Wrap each iterator in [`AnonIter2`] to return 2 different iterators from the same function:

```rust
use anon_iter::AnonIter2;

fn foo(x: i32) -> impl Iterator<Item = i32> {
    match x {
        0 => AnonIter2::I1(1..10),
        _ => AnonIter2::I2(vec![5, 10].into_iter()),
    }
}
```

The crate [`Either`](https://docs.rs/either/latest/either/) allows similar functionality, as it too implements `Iterator` when
its type parameters are both `Iterator`.

But this breaks down when you want to return 3 or more `Iterator`s because you now have to
do extra nesting (e.g. `Either::Left(Either::Right(Either::Left(my_iter)))`). With `anon_iter`, it's just `AnonIter8::I3(my_iter)`.

Additionally, `anon_iter` makes code more readable because it may not be instantly obvious that we are using `Either` for this purpose, but with `AnonEnum`
the intent is apparent.
