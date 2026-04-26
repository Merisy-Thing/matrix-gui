//! Helper utilities for the matrix_gui framework.
//!
//! This module provides utility types and functions for handling optional
//! styling parameters in the GUI framework. It includes:
//!
//! - [`OptionColor`]: Wrapper for optional color values with fallback to style defaults
//! - [`OptionFont`]: Wrapper for optional font values with fallback to style defaults
//! - [`OptionU8`]: Wrapper for optional numeric values with fallback to style defaults
//!
//! # Purpose
//!
//! These helper types are used throughout the framework to allow widgets to have
//! optional styling parameters that fall back to default values from the global
//! style configuration when not explicitly set.
//!
//! # Example
//!
//! ```rust
//! use matrix_gui::prelude::*;
//!
//! // Create an optional color
//! let mut color = OptionColor::none();
//! color.set_color(Rgb565::RED);
//! ```

pub mod lw_geometry;
pub mod lw_primitives;
pub mod matrix_utils;

use crate::{style::Style, ui_font::UiFont};
use embedded_graphics::prelude::PixelColor;

/// Wrapper for optional color values with style fallback.
///
/// This struct provides a convenient way to handle optional color parameters
/// in widgets. When a color is not explicitly set, it falls back to the
/// corresponding color from the global style configuration.
///
/// # Type Parameters
///
/// * `COL` - The pixel color type implementing [`PixelColor`]
///
/// # Example
///
/// ```rust
/// use matrix_gui::prelude::*;
///
/// // Create with no color set
/// let mut color = OptionColor::none();
///
/// // Set a specific color
/// color.set_color(Rgb565::RED);
/// ```
#[derive(Debug)]
pub struct OptionColor<COL> {
    /// The optional color value.
    pub color: Option<COL>,
}

impl<COL: PixelColor> OptionColor<COL> {
    /// Creates a new `OptionColor` with no color set.
    ///
    /// # Returns
    ///
    /// A new `OptionColor` instance with `color` set to `None`.
    pub const fn none() -> Self {
        Self { color: None }
    }

    /// Sets the color value.
    ///
    /// # Arguments
    ///
    /// * `color` - The color to set.
    pub const fn set_color(&mut self, color: COL) {
        self.color = Some(color);
    }

    /// Returns the text color, falling back to the style default if not set.
    ///
    /// # Arguments
    ///
    /// * `style` - Reference to the style configuration.
    ///
    /// # Returns
    ///
    /// The set color if present, otherwise `style.text_color`.
    pub fn text_color(&self, style: &Style<COL>) -> COL {
        self.color.unwrap_or(style.text_color)
    }

    /// Returns the background color, falling back to the style default if not set.
    ///
    /// # Arguments
    ///
    /// * `style` - Reference to the style configuration.
    ///
    /// # Returns
    ///
    /// The set color if present, otherwise `style.background_color`.
    pub fn background_color(&self, style: &Style<COL>) -> COL {
        self.color.unwrap_or(style.background_color)
    }

    /// Returns the border color, falling back to the style default if not set.
    ///
    /// # Arguments
    ///
    /// * `style` - Reference to the style configuration.
    ///
    /// # Returns
    ///
    /// The set color if present, otherwise `style.border_color`.
    pub fn border_color(&self, style: &Style<COL>) -> COL {
        self.color.unwrap_or(style.border_color)
    }
}

/// Converts an `Option<COL>` into an `OptionColor<COL>`.
impl<COL: PixelColor> From<Option<COL>> for OptionColor<COL> {
    fn from(color: Option<COL>) -> Self {
        Self { color }
    }
}

/// Wrapper for optional font values with style fallback.
///
/// This struct provides a convenient way to handle optional font parameters
/// in widgets. When a font is not explicitly set, it falls back to the
/// default font from the global style configuration.
///
/// # Type Parameters
///
/// * `'a` - The lifetime of the font reference
///
/// # Example
///
/// ```ignore
/// use matrix_gui::prelude::*;
///
/// // Create with no font set
/// let mut font = OptionFont::none();
///
/// // Get font with fallback to style
/// let actual_font = font.font(&style);
/// ```
#[derive(Debug)]
pub struct OptionFont<'a> {
    /// The optional font value.
    pub font: Option<UiFont<'a>>,
}

impl<'a> OptionFont<'a> {
    /// Creates a new `OptionFont` with no font set.
    ///
    /// # Returns
    ///
    /// A new `OptionFont` instance with `font` set to `None`.
    pub const fn none() -> Self {
        Self { font: None }
    }

    /// Sets the font value.
    ///
    /// # Arguments
    ///
    /// * `font` - The font to set.
    pub const fn set_font(&mut self, font: UiFont<'a>) {
        self.font = Some(font);
    }

    /// Returns the font, falling back to the style default if not set.
    ///
    /// # Type Parameters
    ///
    /// * `COL` - The pixel color type implementing [`PixelColor`]
    ///
    /// # Arguments
    ///
    /// * `style` - Reference to the style configuration.
    ///
    /// # Returns
    ///
    /// The set font if present, otherwise `style.default_font`.
    pub fn font<COL: PixelColor>(&self, style: &Style<COL>) -> UiFont<'a> {
        self.font.unwrap_or(style.default_font)
    }
}

/// Converts an `Option<UiFont<'a>>` into an `OptionFont<'a>`.
impl<'a> From<Option<UiFont<'a>>> for OptionFont<'a> {
    fn from(font: Option<UiFont<'a>>) -> Self {
        Self { font }
    }
}

/// Wrapper for optional numeric values with style fallback.
///
/// This struct provides a convenient way to handle optional numeric parameters
/// in widgets, such as corner radius and border width. When a value is not
/// explicitly set, it falls back to corresponding value from the global style
/// configuration.
///
/// # Example
///
/// ```rust
/// use matrix_gui::prelude::*;
///
/// // Create with no value set
/// let mut value = OptionU8::none();
///
/// // Set a specific value
/// value.set_value(5);
/// ```
#[derive(Debug)]
pub struct OptionU8 {
    /// The optional numeric value.
    pub value: Option<u8>,
}

impl OptionU8 {
    /// Creates a new `OptionU8` with no value set.
    ///
    /// # Returns
    ///
    /// A new `OptionU8` instance with `value` set to `None`.
    pub const fn none() -> Self {
        Self { value: None }
    }

    /// Sets the numeric value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to set.
    pub const fn set_value(&mut self, value: u8) {
        self.value = Some(value);
    }

    /// Returns the corner radius, falling back to the style default if not set.
    ///
    /// # Type Parameters
    ///
    /// * `COL` - The pixel color type implementing [`PixelColor`]
    ///
    /// # Arguments
    ///
    /// * `style` - Reference to the style configuration.
    ///
    /// # Returns
    ///
    /// The set value if present, otherwise `style.corner_radius` as `u32`.
    pub fn corner_radius<COL: PixelColor>(&self, style: &Style<COL>) -> u32 {
        self.value.unwrap_or(style.corner_radius) as u32
    }

    /// Returns the border width, falling back to the style default if not set.
    ///
    /// # Type Parameters
    ///
    /// * `COL` - The pixel color type implementing [`PixelColor`]
    ///
    /// # Arguments
    ///
    /// * `style` - Reference to the style configuration.
    ///
    /// # Returns
    ///
    /// The set value if present, otherwise `style.border_width` as `u32`.
    pub fn border_width<COL: PixelColor>(&self, style: &Style<COL>) -> u32 {
        self.value.unwrap_or(style.border_width) as u32
    }
}

/// Converts an `Option<u8>` into an `OptionU8`.
impl From<Option<u8>> for OptionU8 {
    fn from(value: Option<u8>) -> Self {
        Self { value }
    }
}
