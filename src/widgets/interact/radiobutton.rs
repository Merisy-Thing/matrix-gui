//! Radio button widget for selecting from options.
//!
//! This module provides a radio button widget that allows users to
//! select one option from a group. Radio buttons are typically used
//! in groups where only one option can be selected at a time.

use crate::prelude::*;
use embedded_graphics::geometry::AnchorX;
use embedded_graphics::primitives::Circle;

/// Radio button widget for selecting from options.
///
/// This widget displays a radio button that can be selected by clicking
/// or touching. When selected, it displays a filled circle. Radio buttons
/// are typically used in groups where only one option can be selected
/// at a time, controlled by a shared value.
///
/// # Type Parameters
///
/// * `'a` - The lifetime of the label string reference and radio value
/// * `ID` - The widget ID type implementing [`WidgetId`]
/// * `T` - The radio button ID and value type (must implement `PartialEq + Copy`)
pub struct RadioButton<'a, ID, T> {
    /// The region defining the radio button's position and size.
    region: &'a Region<ID>,
    /// The ID of this radio button option.
    radio_id: T,
    /// Mutable reference to the currently selected value.
    radio_value: &'a mut T,
    /// The text label to display next to the radio button.
    label: &'a str,
    /// Optional border width.
    border: OptionU8,
}

impl<'a, ID: WidgetId, T> RadioButton<'a, ID, T> {
    pub const fn new(
        region: &'a Region<ID>,
        label: &'a str,
        radio_id: T,
        radio_value: &'a mut T,
    ) -> RadioButton<'a, ID, T> {
        RadioButton {
            region,
            radio_id,
            radio_value,
            label,
            border: OptionU8::none(),
        }
    }

    pub const fn with_border(mut self, border: u8) -> Self {
        self.border.set_value(border);
        self
    }
}

impl<DRAW, COL, ID, T> Widget<DRAW, COL> for RadioButton<'_, ID, T>
where
    DRAW: DrawTarget<Color = COL>,
    ID: WidgetId,
    COL: PixelColor,
    T: PartialEq + Copy,
{
    fn draw(&mut self, ui: &mut Ui<DRAW, COL>) -> GuiResult<Response> {
        #[allow(unused_mut)]
        let mut interaction = ui.check_interact(self.region);
        let widget_id = self.region.id();
        #[cfg(feature = "focus")]
        let focused = ui.check_focused(widget_id);

        let next_state;
        let prev_state = ui.get_widget_state(widget_id)?.status();
        let old_value = *self.radio_value;
        match interaction {
            Interaction::None => {
                #[cfg(feature = "focus")]
                {
                    use crate::focus::Focused;

                    match focused {
                        Focused::No => {
                            next_state = RenderStatus::Rendered;
                        }
                        Focused::Yes => {
                            next_state = RenderStatus::Focused;
                            interaction = focused.into();
                        }
                        Focused::Trigger => {
                            interaction = focused.into();
                            next_state = RenderStatus::Triggered;
                            *self.radio_value = self.radio_id;
                        }
                    }
                }

                #[cfg(not(feature = "focus"))]
                {
                    next_state = RenderStatus::Normal;
                }
            }
            Interaction::Pressed(_) | Interaction::Drag(_) => {
                next_state = RenderStatus::Pressed;
            }
            Interaction::Release(pos) | Interaction::Clicked(pos) => {
                *self.radio_value = self.radio_id;
                next_state = RenderStatus::Released;
                interaction = Interaction::Clicked(pos);
            }
        }
        if next_state == prev_state {
            return Ok(interaction.into());
        }
        ui.get_widget_state(widget_id)?.set_status(next_state);

        let area = self.region.rectangle();
        let changed = *self.radio_value != old_value;
        let style = *ui.style();
        let font = style.default_font;
        let padding = style
            .default_padding
            .width
            .max(style.default_padding.height) as i32;

        let mut radio_rect = area.offset(-padding);
        radio_rect.size.width = radio_rect.size.height;

        let border_width = self.border.border_width(&style);
        let diameter = radio_rect.size.width;
        let center = radio_rect.center();

        let circle_style = PrimitiveStyleBuilder::new()
            .stroke_color(style.border_color)
            .stroke_width(border_width)
            .fill_color(style.background_color)
            .build();

        ui.clear_area(&area)?;
        ui.draw(&Circle::with_center(center, diameter).into_styled(circle_style))
            .ok();

        if *self.radio_value == self.radio_id {
            let dot_style = PrimitiveStyleBuilder::new()
                .stroke_color(style.text_color)
                .stroke_width(0)
                .fill_color(style.text_color)
                .build();

            ui.draw(&Circle::with_center(center, diameter / 2).into_styled(dot_style))
                .ok();
        }

        let mut text = matrix_utils::make_text(self.label, font, style.text_color);
        let text_area = area.resized_width(area.size.width - area.size.height, AnchorX::Right);

        matrix_utils::text_align_translate(&mut text, &text_area, HorizontalAlign::Left);
        ui.draw(&text).ok();

        #[cfg(feature = "focus")]
        if matches!(interaction, Interaction::Pressed(_)) {
            let rect_style = PrimitiveStyleBuilder::new()
                .stroke_color(style.text_color)
                .stroke_width(border_width)
                .build();
            ui.draw(&text_area.into_styled(rect_style)).ok();
        }

        Ok(Response::from_change(changed))
    }
}
