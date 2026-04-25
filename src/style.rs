use crate::ui_font::{DEFAULT_FONT_ASCII, DEFAULT_FONT_LATIN_6, UiFont};
use embedded_graphics::pixelcolor::{BinaryColor, PixelColor, Rgb565};
use embedded_graphics::prelude::*;

/// Macro to create Rgb565 colors from 24-bit RGB values, eg. `rgb565!(0xRRGGBB)`
#[macro_export]
macro_rules! rgb565 {
    ($rgb:expr) => {
        Rgb565::new(
            (($rgb >> (3 + 16)) & 0xFF) as u8,
            (($rgb >> (2 + 8)) & 0xFF) as u8,
            (($rgb >> 3) & 0xFF) as u8,
        )
    };
}

pub fn rgb565_debug_style() -> Style<Rgb565> {
    Style {
        background_color: Rgb565::BLACK,
        border_color: Rgb565::RED,
        text_color: Rgb565::WHITE,
        border_width: 1,
        default_font: DEFAULT_FONT_LATIN_6,
        default_padding: Size::new(4, 4),
        corner_radius: 5,
    }
}

pub fn rgb565_basic_style() -> Style<Rgb565> {
    Style {
        background_color: Rgb565::new(0x4, 0x8, 0x4),
        border_color: Rgb565::RED,
        text_color: Rgb565::WHITE,
        border_width: 1,
        default_font: DEFAULT_FONT_LATIN_6,
        default_padding: Size::new(2, 2),
        corner_radius: 5,
    }
}

pub fn rgb565_light_style() -> Style<Rgb565> {
    Style {
        background_color: Rgb565::CSS_WHITE,
        border_color: Rgb565::CSS_BLACK,
        text_color: Rgb565::CSS_BLACK,
        border_width: 1,
        default_font: DEFAULT_FONT_LATIN_6,
        default_padding: Size::new(2, 2),
        corner_radius: 5,
    }
}

pub fn rgb565_binary_style() -> Style<BinaryColor> {
    Style {
        background_color: BinaryColor::Off,
        border_color: BinaryColor::On,
        text_color: BinaryColor::On,
        border_width: 1,
        default_font: DEFAULT_FONT_ASCII,
        default_padding: Size::new(2, 2),
        corner_radius: 0,
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Style<COL: PixelColor> {
    pub default_font: UiFont<'static>,
    pub background_color: COL,
    pub border_color: COL,
    pub text_color: COL,
    pub default_padding: Size,
    pub corner_radius: u8,
    pub border_width: u8,
}
