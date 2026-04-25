//! Utility functions for matrix_gui framework.
//!
//! This module provides common utility functions used throughout the framework
//! for creating and manipulating graphical elements.
//!
//! # Functions
//!
//! - [`make_rounded_rect`]: Creates a rounded rectangle with equal corner radii
//! - [`make_text`]: Creates a text element with specified styling
//! - [`text_align_translate`]: Aligns and positions text within a rectangle
//! - [`select`]: Selects one of two values based on a condition
//!
//! # Example
//!
//! ```ignore
//! use matrix_gui::prelude::*;
//! use embedded_graphics::primitives::Rectangle;
//!
//! // Create a rounded rectangle
//! let rect = Rectangle::new(Point::new(10, 10), Size::new(100, 50));
//! let rounded = matrix_utils::make_rounded_rect(&rect, 5);
//!
//! // Create and align text
//! let mut text = matrix_utils::make_text("Hello", font, Rgb565::WHITE);
//! matrix_utils::text_align_translate(&mut text, &rect, HorizontalAlign::Center);
//! ```

use crate::ui::HorizontalAlign;
use crate::ui_font::{UiFont, UiTextStyle};
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::RoundedRectangle;
use embedded_graphics::text::Baseline;
use embedded_graphics::{
    prelude::{PixelColor, Point},
    primitives::Rectangle,
    text::Text,
};

/// Creates a rounded rectangle with equal corner radii.
///
/// This is a convenience function for creating rounded rectangles where all
/// corners have the same radius.
///
/// # Arguments
///
/// * `area` - The rectangle defining the bounds of the rounded rectangle.
/// * `corner_radius` - The radius for all corners.
///
/// # Returns
///
/// A `RoundedRectangle` with equal corner radii.
///
/// # Example
///
/// ```rust
/// use matrix_gui::prelude::*;
/// use embedded_graphics::primitives::Rectangle;
///
/// let rect = Rectangle::new(Point::new(10, 10), Size::new(100, 50));
/// let rounded = matrix_utils::make_rounded_rect(&rect, 5);
/// ```
#[inline]
pub const fn make_rounded_rect(area: &Rectangle, corner_radius: u32) -> RoundedRectangle {
    RoundedRectangle::with_equal_corners(*area, Size::new(corner_radius, corner_radius))
}

/// Creates a text element with specified styling.
///
/// This is a convenience function for creating text elements with a given
/// label, font, and color. The text is positioned at the origin (0, 0)
/// with top baseline alignment.
///
/// # Type Parameters
///
/// * `'a` - The lifetime of the label string.
/// * `COL` - The pixel color type implementing [`PixelColor`].
///
/// # Arguments
///
/// * `label` - The text string to display.
/// * `font` - The font to use for rendering.
/// * `text_color` - The color of the text.
///
/// # Returns
///
/// A `Text` element with the specified styling.
///
/// # Example
///
/// ```ignore
/// use matrix_gui::prelude::*;
///
/// let text = matrix_utils::make_text("Hello", my_font, Rgb565::WHITE);
/// ```
#[inline]
pub const fn make_text<'a, COL: PixelColor>(
    label: &'a str,
    font: UiFont<'a>,
    text_color: COL,
) -> Text<'a, UiTextStyle<'a, COL>> {
    Text::with_baseline(
        label,
        Point::new(0, 0),
        UiTextStyle::new(font, text_color),
        Baseline::Top,
    )
}

/// Aligns and positions text within a rectangle.
///
/// This function calculates the appropriate offset to center the text within
/// the rectangle based on the specified horizontal alignment. The text is
/// positioned at the top-left corner of the rectangle plus the calculated offset.
///
/// # Type Parameters
///
/// * `COL` - The pixel color type implementing [`PixelColor`].
///
/// # Arguments
///
/// * `text` - Mutable reference to the text element to align.
/// * `rect` - The rectangle to align the text within.
/// * `align` - The horizontal alignment strategy.
///
/// # Example
///
/// ```ignore
/// use matrix_gui::prelude::*;
///
/// let mut text = matrix_utils::make_text("Hello", font, Rgb565::WHITE);
/// let rect = Rectangle::new(Point::new(10, 10), Size::new(100, 50));
/// matrix_utils::text_align_translate(&mut text, &rect, HorizontalAlign::Center);
/// ```
pub fn text_align_translate<COL: PixelColor>(
    text: &mut Text<'_, UiTextStyle<'_, COL>>,
    rect: &Rectangle,
    align: HorizontalAlign,
) {
    let text_size = text.bounding_box().size;
    let offset_x = match align {
        HorizontalAlign::Left => 0,
        HorizontalAlign::Center | HorizontalAlign::Justify => {
            (rect.size.width as i32 - text_size.width as i32) / 2
        }
        HorizontalAlign::Right => rect.size.width as i32 - text_size.width as i32,
    };
    let offset_y = (rect.size.height as i32 - text_size.height as i32) / 2;

    text.position = rect.top_left;
    text.translate_mut(Point::new(offset_x, offset_y));
}

/// Selects one of two values based on a condition.
///
/// This is a convenience function for conditional value selection, similar to
/// the ternary operator in other languages.
///
/// # Type Parameters
///
/// * `T` - The type of values to select from.
///
/// # Arguments
///
/// * `condition` - If `true`, returns `true_val`; otherwise returns `false_val`.
/// * `true_val` - The value to return if condition is `true`.
/// * `false_val` - The value to return if condition is `false`.
///
/// # Returns
///
/// `true_val` if `condition` is `true`, otherwise `false_val`.
///
/// # Example
///
/// ```rust
/// use matrix_gui::prelude::*;
///
/// let value = matrix_utils::select(true, 10, 20); // Returns 10
/// let value2 = matrix_utils::select(false, 10, 20); // Returns 20
/// ```
#[inline]
pub fn select<T>(condition: bool, true_val: T, false_val: T) -> T {
    if condition { true_val } else { false_val }
}
