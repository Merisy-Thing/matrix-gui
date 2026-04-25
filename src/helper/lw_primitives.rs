//! Lightweight primitive types for the matrix_gui framework.
//!
//! This module provides memory-efficient primitive types that are smaller than
//! standard `embedded_graphics` primitive types. These types use smaller
//! integer types to reduce memory footprint in embedded systems.
//!
//! # Types
//!
//! - [`LwRectangle<T, U>`]: Lightweight rectangle type with configurable coordinate and dimension types
//!
//! # Conversions
//!
//! All lightweight types support bidirectional conversion with their
//! `embedded_graphics` counterparts using `From` and `Into` traits.
//! Conversions use saturating casts to prevent overflow.
//!
//! # Example
//!
//! ```rust
//! use matrix_gui::prelude::*;
//! use embedded_graphics::primitives::Rectangle;
//!
//! // Convert from embedded_graphics Rectangle to LwRectangle
//! let rect = Rectangle::new(Point::new(10, 20), Size::new(100, 200));
//! let lw_rect: LwRectangle<i16, u16> = rect.into();
//!
//! // Convert back to Rectangle
//! let rect2: Rectangle = lw_rect.into();
//! ```

use core::fmt::Debug;
use embedded_graphics::{geometry::AnchorPoint, prelude::Size, primitives::Rectangle};

use crate::prelude::DeltaResize;

use super::lw_geometry::{LwPoint, LwSize};

/// Lightweight rectangle type with configurable coordinate and dimension types.
///
/// This struct represents a 2D rectangle with a top-left point and size,
/// using configurable integer types for coordinates and dimensions. It's designed
/// to be more memory-efficient than standard `embedded_graphics::Rectangle`
/// for embedded systems.
///
/// # Type Parameters
///
/// * `T` - The coordinate type for the top-left point (typically i8 or i16)
/// * `U` - The dimension type for the size (typically u8 or u16)
///
/// # Example
///
/// ```rust
/// use matrix_gui::prelude::*;
///
/// // Create a rectangle
/// let rect = LwRectangle::new(
///     LwPoint::new(10, 20),
///     LwSize::new(100, 200)
/// );
///
/// // Convert from embedded_graphics Rectangle
/// let eg_rect = embedded_graphics::primitives::Rectangle::new(
///     embedded_graphics::geometry::Point::new(10, 20),
///     embedded_graphics::geometry::Size::new(100, 200)
/// );
/// let lw_rect: LwRectangle<i16, u16> = eg_rect.into();
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct LwRectangle<T, U> {
    /// The top-left corner of the rectangle.
    pub top_left: LwPoint<T>,
    /// The size of the rectangle.
    pub size: LwSize<U>,
}

impl<T, U> LwRectangle<T, U>
where
    T: Copy + Clone + Eq + PartialEq + Debug + Default,
    U: Copy + Clone + Eq + PartialEq + Debug + Default,
{
    /// Creates a new `LwRectangle` with the specified top-left point and size.
    ///
    /// # Arguments
    ///
    /// * `top_left` - The top-left corner point.
    /// * `size` - The size of the rectangle.
    ///
    /// # Returns
    ///
    /// A new `LwRectangle` instance.
    pub const fn new(top_left: LwPoint<T>, size: LwSize<U>) -> Self {
        Self { top_left, size }
    }
}

impl LwRectangle<i16, u16> {
    /// Scales the rectangle around its center by the given percentage.
    ///
    /// # Arguments
    ///
    /// * `percent` - The percentage to scale by (100% means no change).
    ///               200% means double the size, 50% means half the size, etc.
    pub fn center_scale(&self, horizontal_percent: u16, vertical_percent: u16) -> Self {
        let scale_width_factor = horizontal_percent as i16 - 100;
        let scale_height_factor = vertical_percent as i16 - 100;

        let delta_width = (self.size.width as i32 * scale_width_factor as i32 / 100) as i16;
        let delta_height = (self.size.height as i32 * scale_height_factor as i32 / 100) as i16;

        let mut area = self.rectangle();
        let (size, ap) = DeltaResize::Center(delta_width, delta_height).transform(&area.size);
        area = area.resized(size, ap);

        area.into()
    }

    /// Resizes the rectangle using the specified resize delta.
    pub fn delta_resize(&self, delta: DeltaResize) -> Self {
        let area = self.rectangle();
        let (size, ap) = delta.transform(&area.size);
        area.resized(size, ap).into()
    }

    /// Moves the rectangle by the specified amount.
    pub fn move_by(&self, dx: i16, dy: i16) -> Self {
        Self {
            top_left: self.top_left.offset(dx, dy),
            size: self.size,
        }
    }

    /// Converts the `LwRectangle` to an `embedded_graphics::Rectangle`.
    pub fn rectangle(&self) -> Rectangle {
        (*self).into()
    }

    /// Resizes the rectangle to the specified width and height, using the given anchor point.
    pub fn resized(&self, width: u16, height: u16, anchor_point: AnchorPoint) -> Self {
        let size = Size::new(width as u32, height as u32);

        self.rectangle().resized(size, anchor_point).into()
    }
}

/// Converts an `embedded_graphics::Rectangle` to `LwRectangle<i8, u8>` using saturating cast.
impl From<Rectangle> for LwRectangle<i8, u8> {
    fn from(rect: Rectangle) -> Self {
        Self {
            top_left: rect.top_left.into(),
            size: rect.size.into(),
        }
    }
}

/// Converts an `embedded_graphics::Rectangle` to `LwRectangle<i16, u16>` using saturating cast.
impl From<Rectangle> for LwRectangle<i16, u16> {
    fn from(rect: Rectangle) -> Self {
        Self {
            top_left: rect.top_left.into(),
            size: rect.size.into(),
        }
    }
}

/// Converts `LwRectangle<i8, u8>` to `embedded_graphics::Rectangle`.
impl From<LwRectangle<i8, u8>> for Rectangle {
    fn from(lw_rect: LwRectangle<i8, u8>) -> Self {
        Rectangle::new(lw_rect.top_left.into(), lw_rect.size.into())
    }
}

/// Converts `LwRectangle<i16, u16>` to `embedded_graphics::Rectangle`.
impl From<LwRectangle<i16, u16>> for Rectangle {
    fn from(lw_rect: LwRectangle<i16, u16>) -> Self {
        Rectangle::new(lw_rect.top_left.into(), lw_rect.size.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_graphics::geometry::Point;
    use embedded_graphics::geometry::Size;

    #[test]
    fn test_lw_rect_i8_u8_from_rectangle() {
        let rect = Rectangle::new(Point::new(10, -20), Size::new(100, 200));
        let lw_rect: LwRectangle<i8, u8> = rect.into();
        assert_eq!(lw_rect.top_left.x, 10);
        assert_eq!(lw_rect.top_left.y, -20);
        assert_eq!(lw_rect.size.width, 100);
        assert_eq!(lw_rect.size.height, 200);
    }

    #[test]
    fn test_lw_rect_i8_u8_to_rectangle() {
        let lw_rect = LwRectangle {
            top_left: LwPoint { x: 5i8, y: -10i8 },
            size: LwSize {
                width: 50u8,
                height: 100u8,
            },
        };
        let rect: Rectangle = lw_rect.into();
        assert_eq!(rect.top_left, Point::new(5, -10));
        assert_eq!(rect.size, Size::new(50, 100));
    }

    #[test]
    fn test_lw_rect_i16_u16_from_rectangle() {
        let rect = Rectangle::new(Point::new(1000, -2000), Size::new(5000, 10000));
        let lw_rect: LwRectangle<i16, u16> = rect.into();
        assert_eq!(lw_rect.top_left.x, 1000);
        assert_eq!(lw_rect.top_left.y, -2000);
        assert_eq!(lw_rect.size.width, 5000);
        assert_eq!(lw_rect.size.height, 10000);
    }

    #[test]
    fn test_lw_rect_i16_u16_to_rectangle() {
        let lw_rect = LwRectangle {
            top_left: LwPoint {
                x: 500i16,
                y: -1000i16,
            },
            size: LwSize {
                width: 500u16,
                height: 1000u16,
            },
        };
        let rect: Rectangle = lw_rect.into();
        assert_eq!(rect.top_left, Point::new(500, -1000));
        assert_eq!(rect.size, Size::new(500, 1000));
    }

    #[test]
    fn test_lw_rect_i16_u16_large_value() {
        let rect = Rectangle::new(
            Point::new(i32::MAX, i32::MIN),
            Size::new(u32::MAX, u32::MAX),
        );
        let lw_rect: LwRectangle<i16, u16> = rect.into();
        assert_eq!(lw_rect.top_left.x, i16::MAX);
        assert_eq!(lw_rect.top_left.y, i16::MIN);
        assert_eq!(lw_rect.size.width, u16::MAX);
        assert_eq!(lw_rect.size.height, u16::MAX);
    }

    #[test]
    fn test_lw_rect_i8_u8_overflow() {
        let rect = Rectangle::new(Point::new(200, -200), Size::new(300, 300));
        let lw_rect: LwRectangle<i8, u8> = rect.into();
        assert_eq!(lw_rect.top_left.x, 127);
        assert_eq!(lw_rect.top_left.y, -128);
        assert_eq!(lw_rect.size.width, u8::MAX);
        assert_eq!(lw_rect.size.height, u8::MAX);
    }

    #[test]
    fn test_lw_rect_zero() {
        let rect = Rectangle::new(Point::new(0, 0), Size::new(0, 0));
        let lw_rect_i8_u8: LwRectangle<i8, u8> = rect.into();
        let lw_rect_i16_u16: LwRectangle<i16, u16> = rect.into();
        assert_eq!(lw_rect_i8_u8.top_left.x, 0);
        assert_eq!(lw_rect_i16_u16.size.width, 0);
    }
}
