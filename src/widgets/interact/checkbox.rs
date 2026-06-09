//! Checkbox widget for boolean state.
//!
//! This module provides a checkbox widget that allows users to toggle
//! a boolean value. The checkbox displays a check mark when checked
//! and can have a label and customizable border.

use embedded_graphics::{geometry::AnchorX, primitives::Line};

use crate::prelude::*;

/// Checkbox widget for boolean state.
///
/// This widget displays a checkbox that can be toggled by clicking or
/// touching. When checked, it displays a check mark. The checkbox can
/// have a label and customizable border width and corner radius.
///
/// # Type Parameters
///
/// * `'a` - The lifetime of the label string reference and checked state
/// * `ID` - The widget ID type implementing [`WidgetId`]
pub struct Checkbox<'a, ID> {
    /// The region defining the checkbox's position and size.
    region: &'a Region<ID>,
    /// Mutable reference to the checked state.
    checked: &'a mut bool,
    /// The text label to display next to the checkbox.
    label: &'a str,
    /// Optional border (width, corner_radius).
    border: Option<(u8, u8)>,
}

impl<'a, ID: WidgetId> Checkbox<'a, ID> {
    pub const fn new(
        region: &'a Region<ID>,
        label: &'a str,
        checked: &'a mut bool,
    ) -> Checkbox<'a, ID> {
        Checkbox {
            region,
            checked,
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
    for Checkbox<'_, ID>
{
    fn draw(&mut self, ui: &mut Ui<DRAW, COL>) -> GuiResult<Response> {
        #[allow(unused_mut)]
        let mut interaction = ui.check_interact(self.region);
        let widget_id = self.region.id();
        #[cfg(feature = "focus")]
        let focused = ui.check_focused(widget_id);

        let next_state;
        let prev_state = ui.get_widget_state(widget_id)?.status();
        let old_checked = *self.checked;

        match interaction {
            Interaction::None => {
                #[cfg(feature = "focus")]
                {
                    use crate::focus::Focused;

                    match focused {
                        Focused::No => {
                            next_state = RenderStatus::Normal;
                        }
                        Focused::Yes => {
                            next_state = RenderStatus::Focused;
                            interaction = focused.into();
                        }
                        Focused::Trigger => {
                            interaction = focused.into();
                            next_state = RenderStatus::Triggered;
                            *self.checked = !*self.checked;
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
            Interaction::Release(_pos) | Interaction::Clicked(_pos) => {
                *self.checked = !*self.checked;
                next_state = RenderStatus::Released;
                #[cfg(feature = "focus")]
                {
                    interaction = Interaction::Clicked(_pos);
                }
            }
        }
        if next_state == prev_state {
            return Ok(Response::Idle);
        }
        ui.get_widget_state(widget_id)?.set_status(next_state);

        let area = self.region.rectangle();
        let changed = *self.checked != old_checked;
        let style = *ui.style();
        let font = style.default_font;
        let padding = style
            .default_padding
            .width
            .max(style.default_padding.height) as i32;

        let mut checkbox_rect = area.offset(-padding);
        checkbox_rect.size.width = checkbox_rect.size.height;

        let (border_width, corner_radius) = self
            .border
            .unwrap_or((style.border_width, style.corner_radius));
        let border_width = border_width as u32;
        let corner_radius = corner_radius as u32;
        let rounded_rect = matrix_utils::make_rounded_rect(&checkbox_rect, corner_radius);

        #[allow(unused_mut)]
        let mut rect_style = PrimitiveStyleBuilder::new()
            .stroke_color(style.border_color)
            .stroke_width(border_width)
            .fill_color(style.background_color)
            .build();
        ui.clear_area(&area)?;
        ui.draw(&rounded_rect.into_styled(rect_style)).ok();

        if *self.checked {
            let check_mark_style = PrimitiveStyleBuilder::new()
                .stroke_color(style.text_color)
                .stroke_width(2)
                .fill_color(style.text_color)
                .build();

            let check_rect = checkbox_rect.offset(-padding);
            let check_size = check_rect.size.width as i32 - 2;

            let ck_top_left = check_rect.top_left;
            let p1 = Point::new(ck_top_left.x, ck_top_left.y + check_size / 2);
            let p2 = Point::new(ck_top_left.x + check_size / 2, ck_top_left.y + check_size);
            let p3 = Point::new(ck_top_left.x + check_size, ck_top_left.y);

            ui.draw(&Line::new(p1, p2).into_styled(check_mark_style))?;
            ui.draw(&Line::new(p2, p3).into_styled(check_mark_style))?;
        }

        let mut text = matrix_utils::make_text(self.label, font, style.text_color);
        let text_area = area.resized_width(area.size.width - area.size.height, AnchorX::Right);

        matrix_utils::text_align_translate(&mut text, &text_area, HorizontalAlign::Left);
        ui.draw(&text).ok();

        #[cfg(feature = "focus")]
        if matches!(interaction, Interaction::Pressed(_)) {
            rect_style.fill_color = None;
            rect_style.stroke_color = Some(style.text_color);
            ui.draw(&text_area.into_styled(rect_style)).ok();
        }

        Ok(Response::from_change(changed))
    }
}
