//! Lightweight geometry types for the matrix_gui framework.
//!
//! This module provides memory-efficient geometry types that are smaller than
//! the standard `embedded_graphics` types. These types use smaller integer
//! types (i8, i16, u8, u16) to reduce memory footprint in embedded systems.
//!
//! # Types
//!
//! - [`LwPoint<T>`]: Lightweight point type with configurable coordinate type
//! - [`LwSize<T>`]: Lightweight size type with configurable dimension type
//! - [`DeltaResize`]: Enum for specifying resize operations with anchor points
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
//! use embedded_graphics::prelude::*;
//!
//! // Convert from embedded_graphics Point to LwPoint
//! let point = Point::new(100, 200);
//! let lw_point: LwPoint<i16> = point.into();
//!
//! // Convert back to Point
//! let point2: Point = lw_point.into();
//! ```

use embedded_graphics::geometry::AnchorPoint;
use embedded_graphics::geometry::Point;
use embedded_graphics::geometry::Size;
use saturating_cast::SaturatingCast;

/// Lightweight point type with configurable coordinate type.
///
/// This struct represents a 2D point with x and y coordinates using a
/// configurable integer type. It's designed to be more memory-efficient
/// than the standard `embedded_graphics::Point` for embedded systems.
///
/// # Type Parameters
///
/// * `T` - The coordinate type (typically i8 or i16)
///
/// # Example
///
/// ```rust
/// use matrix_gui::prelude::*;
///
/// // Create a point
/// let point = LwPoint::new(10, 20);
///
/// // Convert from embedded_graphics Point
/// let eg_point = embedded_graphics::prelude::Point::new(100, 200);
/// let lw_point: LwPoint<i16> = eg_point.into();
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct LwPoint<T> {
    /// The x-coordinate.
    pub x: T,
    /// The y-coordinate.
    pub y: T,
}

impl<T> LwPoint<T> {
    /// Creates a new `LwPoint` with the specified coordinates.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate.
    /// * `y` - The y-coordinate.
    ///
    /// # Returns
    ///
    /// A new `LwPoint` instance.
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
/// Offset the point by the given amount.
impl LwPoint<i8> {
    pub const fn offset(&self, dx: i8, dy: i8) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

/// Offset the point by the given amount.
impl LwPoint<i16> {
    pub const fn offset(&self, dx: i16, dy: i16) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

/// Converts an `embedded_graphics::Point` to `LwPoint<i8>` using saturating cast.
impl From<Point> for LwPoint<i8> {
    fn from(point: Point) -> Self {
        Self {
            x: point.x.saturating_cast(),
            y: point.y.saturating_cast(),
        }
    }
}

/// Converts an `embedded_graphics::Point` to `LwPoint<i16>` using saturating cast.
impl From<Point> for LwPoint<i16> {
    fn from(point: Point) -> Self {
        Self {
            x: point.x.saturating_cast(),
            y: point.y.saturating_cast(),
        }
    }
}

/// Converts `LwPoint<i8>` to `embedded_graphics::Point`.
impl From<LwPoint<i8>> for Point {
    fn from(lw_point: LwPoint<i8>) -> Self {
        Point::new(lw_point.x as i32, lw_point.y as i32)
    }
}

/// Converts `LwPoint<i16>` to `embedded_graphics::Point`.
impl From<LwPoint<i16>> for Point {
    fn from(lw_point: LwPoint<i16>) -> Self {
        Point::new(lw_point.x as i32, lw_point.y as i32)
    }
}

/// Lightweight size type with configurable dimension type.
///
/// This struct represents a 2D size with width and height dimensions using a
/// configurable integer type. It's designed to be more memory-efficient
/// than standard `embedded_graphics::Size` for embedded systems.
///
/// # Type Parameters
///
/// * `T` - The dimension type (typically u8 or u16)
///
/// # Example
///
/// ```rust
/// use matrix_gui::prelude::*;
///
/// // Create a size
/// let size = LwSize::new(100, 200);
///
/// // Convert from embedded_graphics Size
/// let eg_size = embedded_graphics::prelude::Size::new(100, 200);
/// let lw_size: LwSize<u16> = eg_size.into();
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct LwSize<T> {
    /// The width dimension.
    pub width: T,
    /// The height dimension.
    pub height: T,
}

impl<T> LwSize<T> {
    /// Creates a new `LwSize` with specified dimensions.
    ///
    /// # Arguments
    ///
    /// * `width` - The width dimension.
    /// * `height` - The height dimension.
    ///
    /// # Returns
    ///
    /// A new `LwSize` instance.
    pub const fn new(width: T, height: T) -> Self {
        Self { width, height }
    }
}

/// Converts an `embedded_graphics::Size` to `LwSize<u8>` using saturating cast.
impl From<Size> for LwSize<u8> {
    fn from(size: Size) -> Self {
        Self {
            width: size.width.saturating_cast(),
            height: size.height.saturating_cast(),
        }
    }
}

/// Converts an `embedded_graphics::Size` to `LwSize<u16>` using saturating cast.
impl From<Size> for LwSize<u16> {
    fn from(size: Size) -> Self {
        Self {
            width: size.width.saturating_cast(),
            height: size.height.saturating_cast(),
        }
    }
}

/// Converts `LwSize<u8>` to `embedded_graphics::Size`.
impl From<LwSize<u8>> for Size {
    fn from(lw_size: LwSize<u8>) -> Self {
        Size::new(lw_size.width as u32, lw_size.height as u32)
    }
}

/// Converts `LwSize<u16>` to `embedded_graphics::Size`.
impl From<LwSize<u16>> for Size {
    fn from(lw_size: LwSize<u16>) -> Self {
        Size::new(lw_size.width as u32, lw_size.height as u32)
    }
}

/// Delta resize handle with anchor point specification.
///
/// This enum represents different resize operations for widgets, specifying
/// both the size delta and the anchor point to use during resizing.
/// Each variant represents a different corner or edge of a widget.
///
/// # Variants
///
/// - `TopLeft(w, h)`: Resize from top-left corner
/// - `TopCenter(w, h)`: Resize from top edge (centered)
/// - `TopRight(w, h)`: Resize from top-right corner
/// - `CenterLeft(w, h)`: Resize from left edge (centered)
/// - `Center(w, h)`: Resize from center
/// - `CenterRight(w, h)`: Resize from right edge (centered)
/// - `BottomLeft(w, h)`: Resize from bottom-left corner
/// - `BottomCenter(w, h)`: Resize from bottom edge (centered)
/// - `BottomRight(w, h)`: Resize from bottom-right corner
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum DeltaResize {
    /// Resize from top-left corner with width and height deltas.
    TopLeft(i16, i16),
    /// Resize from top edge (centered) with width and height deltas.
    TopCenter(i16, i16),
    /// Resize from top-right corner with width and height deltas.
    TopRight(i16, i16),
    /// Resize from left edge (centered) with width and height deltas.
    CenterLeft(i16, i16),
    /// Resize from center with width and height deltas.
    Center(i16, i16),
    /// Resize from right edge (centered) with width and height deltas.
    CenterRight(i16, i16),
    /// Resize from bottom-left corner with width and height deltas.
    BottomLeft(i16, i16),
    /// Resize from bottom edge (centered) with width and height deltas.
    BottomCenter(i16, i16),
    /// Resize from bottom-right corner with width and height deltas.
    BottomRight(i16, i16),
}

impl DeltaResize {
    pub const fn unwrap(&self) -> (i32, i32, AnchorPoint) {
        let (delta_w, delta_h, ap) = match *self {
            DeltaResize::TopLeft(w, h) => (w, h, AnchorPoint::TopLeft),
            DeltaResize::TopCenter(w, h) => (w, h, AnchorPoint::TopCenter),
            DeltaResize::TopRight(w, h) => (w, h, AnchorPoint::TopRight),
            DeltaResize::CenterLeft(w, h) => (w, h, AnchorPoint::CenterLeft),
            DeltaResize::Center(w, h) => (w, h, AnchorPoint::Center),
            DeltaResize::CenterRight(w, h) => (w, h, AnchorPoint::CenterRight),
            DeltaResize::BottomLeft(w, h) => (w, h, AnchorPoint::BottomLeft),
            DeltaResize::BottomCenter(w, h) => (w, h, AnchorPoint::BottomCenter),
            DeltaResize::BottomRight(w, h) => (w, h, AnchorPoint::BottomRight),
        };

        (delta_w as i32, delta_h as i32, ap)
    }

    /// Applies the resize deltas to the given size and returns
    /// the new size along with the appropriate anchor point.
    ///
    /// # Arguments
    ///
    /// * `size` - The original size to resize.
    ///
    /// # Returns
    ///
    /// A tuple containing:
    /// - The new size as `Size`
    /// - The anchor point as `AnchorPoint`
    pub const fn transform(&self, size: &Size) -> (Size, AnchorPoint) {
        let (delta_w, delta_h, anchor_point) = self.unwrap();
        let width = size.width as i32 + delta_w;
        let height = size.height as i32 + delta_h;

        (Size::new(width as u32, height as u32), anchor_point)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lw_point_i8_from_point() {
        let point = Point::new(10, -20);
        let lw_point: LwPoint<i8> = point.into();
        assert_eq!(lw_point.x, 10);
        assert_eq!(lw_point.y, -20);
    }

    #[test]
    fn test_lw_point_i8_to_point() {
        let lw_point = LwPoint { x: 5i8, y: -10i8 };
        let point: Point = lw_point.into();
        assert_eq!(point, Point::new(5, -10));
    }

    #[test]
    fn test_lw_point_i8_overflow() {
        let point = Point::new(200, -200);
        let lw_point: LwPoint<i8> = point.into();
        assert_eq!(lw_point.x, 127);
        assert_eq!(lw_point.y, -128);
    }

    #[test]
    fn test_lw_point_i16_from_point() {
        let point = Point::new(1000, -2000);
        let lw_point: LwPoint<i16> = point.into();
        assert_eq!(lw_point.x, 1000);
        assert_eq!(lw_point.y, -2000);
    }

    #[test]
    fn test_lw_point_i16_to_point() {
        let lw_point = LwPoint {
            x: 500i16,
            y: -1000i16,
        };
        let point: Point = lw_point.into();
        assert_eq!(point, Point::new(500, -1000));
    }

    #[test]
    fn test_lw_point_i16_large_value() {
        let point = Point::new(i32::MAX, i32::MIN);
        let lw_point: LwPoint<i16> = point.into();
        assert_eq!(lw_point.x, i16::MAX);
        assert_eq!(lw_point.y, i16::MIN);
    }

    #[test]
    fn test_lw_point_zero() {
        let point = Point::new(0, 0);
        let lw_point_i8: LwPoint<i8> = point.into();
        let lw_point_i16: LwPoint<i16> = point.into();
        assert_eq!(lw_point_i8.x, 0);
        assert_eq!(lw_point_i16.y, 0);
    }

    #[test]
    fn test_lw_point_into_i8() {
        let point = Point::new(10, -20);
        let lw_point: LwPoint<i8> = point.into();
        assert_eq!(lw_point.x, 10);
        assert_eq!(lw_point.y, -20);
    }

    #[test]
    fn test_lw_point_into_i16() {
        let point = Point::new(1000, -2000);
        let lw_point: LwPoint<i16> = point.into();
        assert_eq!(lw_point.x, 1000);
        assert_eq!(lw_point.y, -2000);
    }

    #[test]
    fn test_lw_point_from_i8() {
        let lw_point = LwPoint { x: 5i8, y: -10i8 };
        let point: Point = lw_point.into();
        assert_eq!(point, Point::new(5, -10));
    }

    #[test]
    fn test_lw_point_from_i16() {
        let lw_point = LwPoint {
            x: 500i16,
            y: -1000i16,
        };
        let point: Point = lw_point.into();
        assert_eq!(point, Point::new(500, -1000));
    }

    #[test]
    fn test_lw_size_u8_from_size() {
        let size = Size::new(100, 200);
        let lw_size: LwSize<u8> = size.into();
        assert_eq!(lw_size.width, 100);
        assert_eq!(lw_size.height, 200);
    }

    #[test]
    fn test_lw_size_u8_to_size() {
        let lw_size = LwSize {
            width: 50u8,
            height: 100u8,
        };
        let size: Size = lw_size.into();
        assert_eq!(size, Size::new(50, 100));
    }

    #[test]
    fn test_lw_size_u8_overflow() {
        let size = Size::new(300, 300);
        let lw_size: LwSize<u8> = size.into();
        assert_eq!(lw_size.width, u8::MAX);
        assert_eq!(lw_size.height, u8::MAX);
    }

    #[test]
    fn test_lw_size_u16_from_size() {
        let size = Size::new(1000, 2000);
        let lw_size: LwSize<u16> = size.into();
        assert_eq!(lw_size.width, 1000);
        assert_eq!(lw_size.height, 2000);
    }

    #[test]
    fn test_lw_size_u16_to_size() {
        let lw_size = LwSize {
            width: 500u16,
            height: 1000u16,
        };
        let size: Size = lw_size.into();
        assert_eq!(size, Size::new(500, 1000));
    }

    #[test]
    fn test_lw_size_u16_large_value() {
        let size = Size::new(u32::MAX, u32::MAX);
        let lw_size: LwSize<u16> = size.into();
        assert_eq!(lw_size.width, u16::MAX);
        assert_eq!(lw_size.height, u16::MAX);
    }

    #[test]
    fn test_lw_size_zero() {
        let size = Size::new(0, 0);
        let lw_size_u8: LwSize<u8> = size.into();
        let lw_size_u16: LwSize<u16> = size.into();
        assert_eq!(lw_size_u8.width, 0);
        assert_eq!(lw_size_u16.height, 0);
    }
}
