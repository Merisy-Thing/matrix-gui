#![cfg_attr(not(test), no_std)]
#![allow(clippy::needless_doctest_main)]
#![allow(clippy::doc_nested_refdefs)]
#![cfg_attr(not(doctest), doc = include_str!("../README.md"))]

pub mod animation;
pub mod fill_rect;
pub mod helper;
pub mod i18n;
pub mod prelude;
pub mod region;
pub mod style;
pub mod ui;
pub mod widget_state;
pub mod widgets;

#[cfg(feature = "focus")]
pub mod focus;

#[cfg(feature = "framebuffer")]
pub mod framebuf;

#[cfg(feature = "popup")]
pub mod modal;

pub mod ui_font {
    pub type UiFont<'a> = multi_mono_font::MultiMonoFontList<'a>;
    pub type UiTextStyle<'a, C> = multi_mono_font::MultiMonoTextStyle<'a, C>;

    pub const DEFAULT_FONT_LATIN_6: UiFont = &[&multi_mono_font::ascii::FONT_10X20];
    pub const DEFAULT_FONT_ASCII: UiFont = &[&multi_mono_font::ascii::FONT_9X15];
}
