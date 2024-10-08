# easing-function

<!-- This file is generated by `rustme`. Ensure you're editing the source in the .rustme/ directory --!>
<!-- markdownlint-disable first-line-h1 -->

![easing-function is considered alpha](https://img.shields.io/badge/status-alpha-orange)
[![crate version](https://img.shields.io/crates/v/muse.svg)](https://crates.io/crates/easing-function)
[![Documentation for `main`](https://img.shields.io/badge/docs-main-informational)](https://khonsulabs.github.io/easing-function/main/easing_function/)

A Rusty implementation of easing functions.

## What is an Easing Function?

An easing function is a function that describes how to apply a change of a value
over time. This library's easing functions expect input values between 0.0 and
1.0 and will return values corresponding to that range but may be outside of the
range depending on the function.

Easing functions are typically used in animation systems to customize values
produced between keyframes (tweening).

## Why another easing functions crate?

When creating [Cushy](https://github.com/khonsulabs/cushy), no maintained
library seemed to offer a trait-based solution to allow for custom easing
functions in addition to the standard ones inspired by Robert Penner's original
collection.

## Using this crate

The [`EasingFunction`][easingfunction] type is the central type of this crate. It allows
defining an easing function in two ways:

- [`EasingFunction::from_fn`][from_fn]: Creates an easing function from a `fn(f32) -> f32` function.
- [`EasingFunction::new`][new]: Creates an easing function from an [`Easing`][easing]
      implementor.

This crate also provides the standard set of easing functions in the
[easings][easings] module (e.g., [`EaseInOutSine`][sine]). Finally, the
[`StandardEasings`][standard] enum provides access to the standard easing
functions through an enumeration. This enum also supports `serde` when enabling
the `serde` feature of this crate.

[easing]: https://khonsulabs.github.io/easing-function/main/easing_function/trait.Easing.html
[easingfunction]: https://khonsulabs.github.io/easing-function/main/easing_function/struct.EasingFunction.html
[new]: https://khonsulabs.github.io/easing-function/main/easing_function/struct.EasingFunction.html#method.new
[from_fn]: https://khonsulabs.github.io/easing-function/main/easing_function/struct.EasingFunction.html#method.from_fn
[sine]: https://khonsulabs.github.io/easing-function/main/easing_function/easings/struct.EaseInOutSine.html
[standard]: https://khonsulabs.github.io/easing-function/main/easing_function/easings/enum.StandardEasing.html


## Open-source Licenses

This project, like all projects from [Khonsu Labs](https://khonsulabs.com/), is open-source.
This repository is available under the [MIT License](./LICENSE-MIT) or the
[Apache License 2.0](./LICENSE-APACHE).

To learn more about contributing, please see [CONTRIBUTING.md](./CONTRIBUTING.md).
