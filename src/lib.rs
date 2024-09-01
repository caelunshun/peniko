// Copyright 2022 the Peniko Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! A Rust 2D graphics type library
//!
//! The `peniko` library builds on top of [`kurbo`] and provides a set of generic types that define
//! styles for rendering and composition.
//!
//! The name "peniko" is Esperanto for "brush" which is one family of types that the library
//! contains.
//!
//! [`kurbo`]: https://crates.io/crates/kurbo

#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]
#![warn(unused_crate_dependencies)]

mod blend;
mod blob;
mod brush;
mod font;
mod gradient;
mod image;
mod style;

pub use blend::{BlendMode, Compose, Mix};
pub use blob::{Blob, WeakBlob};
pub use brush::{Brush, BrushRef, Extend};
pub use font::Font;
pub use gradient::{ColorStop, ColorStops, ColorStopsSource, Gradient, GradientKind};
pub use image::{Format, Image};
/// Re-export of the kurbo 2D curve library.
pub use kurbo;
/// Re-export of `palette` crate for color management.
pub use palette;
use palette::LinSrgba;
pub use style::{Fill, Style, StyleRef};

/// General-purpose color type. `LinSrgba`
/// uses a 32-bit float per channel, which ensures
/// we maintain as much precision as possible before rendering
/// to accommodate a wide range of output color spaces.
///
/// Unless otherwise stated, `peniko` and crates using it
/// expect colors to be provided unpremultiplied.
pub type Color = LinSrgba;

/// Predefined color constants.
pub mod colors {
    #[doc(inline)]
    pub use palette::named::*;
}
