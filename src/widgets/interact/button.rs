//! Button widget for user interaction.
//!
//! This module provides a clickable button widget that responds to
//! touch/click events and optionally focus events. The button displays
//! a label and can have customizable border and corner radius.

use crate::prelude::*;

/// Button widget for user interaction.
///
/// This widget displays a clickable button with a text label. It responds
/// to touch/click events and optionally focus events (when the `focus`
/// feature is enabled). The button can have customizable border width
/// and corner radius.
///
/// # Type Parameters
///
/// * `'a` - The lifetime of the label string reference
/// * `ID` - The widget ID type implementing [`WidgetId`]
pub struct Button<'a, ID> {
    /// The region defining the button's position and size.
    region: &'a Region<ID>,
    /// The text label to display on the button.
    label: &'a str,
    /// Optional border (width, corner_radius).
    border: Option<(u8, u8)>,
}

impl<'a, ID: WidgetId> Button<'a, ID> {
    pub const fn new(region: &'a Region<ID>, label: &'a str) -> Button<'a, ID> {
        Button {
            region,
            label,
            border: None,
        }
    }

    pub const fn with_border(mut self, border: u8, radius: u8) -> Self {
        self.border = Some((border, radius));
        self
    }
}

impl<DRAW: DrawTarget<Color = COL>, ID: WidgetId, COL: PixelColor> Widget<DRAW, COL>
    for Button<'_, ID>
{
    fn draw(&mut self, ui: &mut Ui<DRAW, COL>) -> GuiResult<Response> {
        #[allow(unused_mut)]
        let mut interaction = ui.check_interact(self.region);
        let widget_id = self.region.id();
        #[cfg(feature = "focus")]
        let focused = ui.check_focused(widget_id);

        let prevstate = ui.get_widget_state(widget_id)?.status();
        let next_state;
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
                        }
                    }
                }

                #[cfg(not(feature = "focus"))]
                {
                    next_state = RenderStatus::Rendered;
                }
            }
            Interaction::Pressed(_) | Interaction::Drag(_) => {
                next_state = RenderStatus::Pressed;
            }
            Interaction::Release(pos) | Interaction::Clicked(pos) => {
                next_state = RenderStatus::Released;
                interaction = Interaction::Clicked(pos);
            }
        };

        if next_state == prevstate {
            return Ok(interaction.into());
        }
        ui.get_widget_state(widget_id)?.set_status(next_state);

        let area = self.region.rectangle();
        let font = ui.style().default_font;
        let mut text = matrix_utils::make_text(self.label, font, ui.style().text_color);

        let (border_width, corner_radius) = self
            .border
            .unwrap_or((ui.style().border_width, ui.style().corner_radius));
        let border_width = border_width as u32;
        let corner_radius = corner_radius as u32;
        let rounded_rect = matrix_utils::make_rounded_rect(&area, corner_radius);

        ui.clear_area(&area)?;

        let rect_style = match interaction {
            Interaction::None => PrimitiveStyleBuilder::new()
                .stroke_color(ui.style().border_color)
                .stroke_width(border_width)
                .fill_color(ui.style().background_color)
                .build(),
            _ => {
                text.character_style.text_color = ui.style().background_color;
                PrimitiveStyleBuilder::new()
                    .stroke_color(ui.style().background_color)
                    .stroke_width(border_width)
                    .fill_color(ui.style().border_color)
                    .build()
            }
        };

        ui.draw(&rounded_rect.into_styled(rect_style)).ok();

        matrix_utils::text_align_translate(&mut text, &area, HorizontalAlign::Center);
        ui.draw(&text).ok();

        Ok(interaction.into())
    }
}
