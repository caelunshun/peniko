// Copyright 2022 the Peniko Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::{Color, Extend};

use kurbo::Point;
use smallvec::SmallVec;

use core::cmp::Ordering;
use core::hash::{Hash, Hasher};

/// Offset and color of a transition point in a [gradient](Gradient).
#[derive(Copy, Clone, Default, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ColorStop {
    /// Normalized offset of the stop.
    pub offset: f32,
    /// Color at the specified offset.
    pub color: Color,
}

impl PartialOrd for ColorStop {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.offset.partial_cmp(&other.offset)
    }
}

impl Hash for ColorStop {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.offset.to_bits().hash(state);
        hash_color(self.color, state);
    }
}

fn hash_color(color: Color, state: &mut impl Hasher) {
    color.red.to_bits().hash(state);
    color.green.to_bits().hash(state);
    color.blue.to_bits().hash(state);
    color.alpha.to_bits().hash(state);
}

// Override PartialEq to use to_bits for the offset to match with the Hash impl
impl PartialEq for ColorStop {
    fn eq(&self, other: &Self) -> bool {
        self.offset.to_bits() == other.offset.to_bits() && self.color == other.color
    }
}

impl Eq for ColorStop {}

impl ColorStop {
    /// Returns the color stop with the alpha component multiplied by the specified
    /// factor.
    #[must_use]
    pub fn with_alpha_factor(self, alpha: f32) -> Self {
        Self {
            offset: self.offset,
            color: {
                let mut color = self.color;
                color.alpha *= alpha;
                color
            },
        }
    }
}

impl From<(f32, Color)> for ColorStop {
    fn from(pair: (f32, Color)) -> Self {
        Self {
            offset: pair.0,
            color: pair.1,
        }
    }
}

/// Collection of color stops.
pub type ColorStops = SmallVec<[ColorStop; 4]>;

/// Properties for the supported [gradient](Gradient) types.
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GradientKind {
    /// Gradient that transitions between two or more colors along a line.
    Linear {
        /// Starting point.
        start: Point,
        /// Ending point.
        end: Point,
    },
    /// Gradient that transitions between two or more colors that radiate from an origin.
    Radial {
        /// Center of start circle.
        start_center: Point,
        /// Radius of start circle.
        start_radius: f32,
        /// Center of end circle.
        end_center: Point,
        /// Radius of end circle.
        end_radius: f32,
    },
    /// Gradient that transitions between two or more colors that rotate around a center
    /// point.
    Sweep {
        /// Center point.
        center: Point,
        /// Start angle of the sweep, counter-clockwise of the x-axis.
        start_angle: f32,
        /// End angle of the sweep, counter-clockwise of the x-axis.
        end_angle: f32,
    },
}

/// Definition of a gradient that transitions between two or more colors.
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Gradient {
    /// Kind and properties of the gradient.
    pub kind: GradientKind,
    /// Extend mode.
    pub extend: Extend,
    /// Color stop collection.
    pub stops: ColorStops,
}

impl Default for Gradient {
    fn default() -> Self {
        Self {
            kind: GradientKind::Linear {
                start: Point::default(),
                end: Point::default(),
            },
            extend: Default::default(),
            stops: Default::default(),
        }
    }
}

impl Gradient {
    /// Creates a new linear gradient for the specified start and end points.
    pub fn new_linear(start: impl Into<Point>, end: impl Into<Point>) -> Self {
        Self {
            kind: GradientKind::Linear {
                start: start.into(),
                end: end.into(),
            },
            extend: Default::default(),
            stops: Default::default(),
        }
    }

    /// Creates a new radial gradient for the specified center point and radius.
    pub fn new_radial(center: impl Into<Point>, radius: f32) -> Self {
        let center = center.into();
        Self {
            kind: GradientKind::Radial {
                start_center: center,
                start_radius: 0.0,
                end_center: center,
                end_radius: radius,
            },
            extend: Default::default(),
            stops: Default::default(),
        }
    }

    /// Creates a new two point radial gradient for the specified center points and radii.
    pub fn new_two_point_radial(
        start_center: impl Into<Point>,
        start_radius: f32,
        end_center: impl Into<Point>,
        end_radius: f32,
    ) -> Self {
        Self {
            kind: GradientKind::Radial {
                start_center: start_center.into(),
                start_radius,
                end_center: end_center.into(),
                end_radius,
            },
            extend: Default::default(),
            stops: Default::default(),
        }
    }

    /// Creates a new sweep gradient for the specified center point, start and
    /// end angles.
    pub fn new_sweep(center: impl Into<Point>, start_angle: f32, end_angle: f32) -> Self {
        Self {
            kind: GradientKind::Sweep {
                center: center.into(),
                start_angle,
                end_angle,
            },
            extend: Default::default(),
            stops: Default::default(),
        }
    }

    /// Builder method for setting the gradient extend mode.
    #[must_use]
    pub fn with_extend(mut self, mode: Extend) -> Self {
        self.extend = mode;
        self
    }

    /// Builder method for setting the color stop collection.
    #[must_use]
    pub fn with_stops(mut self, stops: impl ColorStopsSource) -> Self {
        self.stops.clear();
        stops.collect_stops(&mut self.stops);
        self
    }
}

/// Trait for types that represent a source of color stops.
pub trait ColorStopsSource {
    /// Append the stops represented within `self` into `vec`.
    fn collect_stops(&self, vec: &mut SmallVec<[ColorStop; 4]>);
}

impl<T> ColorStopsSource for &'_ [T]
where
    T: Into<ColorStop> + Copy,
{
    fn collect_stops(&self, vec: &mut SmallVec<[ColorStop; 4]>) {
        for &stop in *self {
            vec.push(stop.into());
        }
    }
}

impl<T, const N: usize> ColorStopsSource for [T; N]
where
    T: Into<ColorStop> + Copy,
{
    fn collect_stops(&self, vec: &mut SmallVec<[ColorStop; 4]>) {
        for stop in *self {
            vec.push(stop.into());
        }
    }
}

impl ColorStopsSource for &'_ [Color] {
    fn collect_stops(&self, vec: &mut SmallVec<[ColorStop; 4]>) {
        if !self.is_empty() {
            let denom = (self.len() - 1).max(1) as f32;
            vec.extend(self.iter().enumerate().map(|(i, c)| ColorStop {
                offset: (i as f32) / denom,
                color: *c,
            }));
        }
    }
}

impl<const N: usize> ColorStopsSource for [Color; N] {
    fn collect_stops(&self, vec: &mut SmallVec<[ColorStop; 4]>) {
        (&self[..]).collect_stops(vec);
    }
}
