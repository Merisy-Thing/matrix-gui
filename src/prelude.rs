//! Matrix GUI Prelude
//!
//! This module provides a convenient way to import commonly used matrix_gui items.
//!
//! Usage:
//! ```rust
//! use matrix_gui::prelude::*;
//! ```

// Core UI components
pub use crate::ui::{GuiError, GuiResult, HorizontalAlign, Response, Ui, Widget};
pub use crate::ui_font::{UiFont, UiTextStyle};

// Animation subsystem
pub use crate::animation::{
    ANIM_SCALE, Anim, AnimId, AnimInstance, AnimManager, AnimOptions, AnimState, AnimStatus, Easing,
};

#[cfg(feature = "interaction")]
pub use crate::{
    ui::Interaction,
    widgets::interact::{
        button::Button, checkbox::Checkbox, radiobutton::RadioButton, slider::Slider,
    },
};
// Focus management
#[cfg(feature = "focus")]
pub use crate::focus::{FocusState, Focused};

pub use crate::{helper::lw_geometry::*, helper::lw_primitives::*, helper::*, i18n::*};

// Widget state management
pub use crate::widget_state::{RenderState, RenderStatus, WidgetId, WidgetStates};

// Region management
pub use crate::region::{self, Region};

#[cfg(feature = "popup")]
pub use crate::modal::Modal;

#[cfg(all(feature = "interaction", feature = "popup"))]
pub use crate::widgets::interact::msgbox::MessageBox;

// Built-in widgets
pub use crate::widgets::{
    background::Background,
    bar::Bar,
    label::Label,
    listbox::ListBox,
    plaintext::PlainText,
    staticimage::StaticImage,
    staticline::{OriHorizontal, OriVertical, StaticLine},
};

// Style and theming
pub use crate::rgb565;
pub use crate::style::Style;

// Re-export
pub use paste;

// Re-export commonly used embedded-graphics items
pub use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{AnchorPoint, Point, Size},
    pixelcolor::{BinaryColor, PixelColor, Rgb565},
    prelude::*,
    primitives::{PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, RoundedRectangle},
    text::{Baseline, Text},
};
