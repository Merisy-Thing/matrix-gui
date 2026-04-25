//! Slider widget for selecting numeric values.
//!
//! This module provides a slider widget that allows users to select
//! a numeric value within a specified range by dragging a knob along
//! a track. The slider can have a label and customizable step size.

use crate::prelude::*;
use core::cmp::max;
use core::ops::RangeInclusive;
use embedded_graphics::{
    primitives::{Circle, Line, PrimitiveStyleBuilder},
    text::Alignment,
};

/// Slider widget for selecting numeric values.
///
/// This widget displays a horizontal slider that allows users to select
/// a numeric value within a specified range by dragging a knob along
/// a track. The slider can have an optional label and customizable
/// step size for discrete values.
///
/// # Type Parameters
///
/// * `'a` - The lifetime of the label string reference and value
/// * `ID` - The widget ID type implementing [`WidgetId`]
pub struct Slider<'a, ID> {
    /// The region defining the slider's position and size.
    region: &'a Region<ID>,
    /// Mutable reference to the current value.
    value: &'a mut i16,
    /// The valid range of values.
    range: RangeInclusive<i16>,
    /// The step size for discrete values.
    step_size: u16,
    /// Optional label to display above the slider.
    label: Option<&'a str>,
}

impl<'a, ID: WidgetId> Slider<'a, ID> {
    pub const fn new(
        region: &'a Region<ID>,
        value: &'a mut i16,
        range: RangeInclusive<i16>,
    ) -> Self {
        Self {
            region,
            value,
            range,
            step_size: 1,
            label: None,
        }
    }

    pub const fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    pub fn step_size(mut self, step_size: u16) -> Self {
        let range_span = (*self.range.end() - *self.range.start()).abs();
        self.step_size = step_size.clamp(1, range_span as u16);
        self
    }
}

impl<DRAW: DrawTarget<Color = COL>, ID: WidgetId, COL: PixelColor> Widget<DRAW, COL>
    for Slider<'_, ID>
{
    fn draw(&mut self, ui: &mut Ui<DRAW, COL>) -> GuiResult<Response> {
        let area = self.region.rectangle();
        let interaction = ui.check_interact(self.region);
        let state = ui.get_widget_state(self.region.id())?;
        let padding = ui.style().default_padding;

        let slider_thickness = 2;
        let slider_knob_diameter = 10;

        let mut height = max(area.size.height, slider_knob_diameter + padding.height * 2);
        let mut width = area.size.width + 2 * padding.width;

        let font = ui.style().default_font;
        let mut text = if let Some(label) = self.label {
            let mut text = Text::new(
                label,
                area.top_left,
                UiTextStyle::new(font, ui.style().text_color),
            );
            text.text_style.alignment = Alignment::Center;
            text.text_style.baseline = Baseline::Top;
            height += padding.height + text.bounding_box().size.height;
            width = width.max(text.bounding_box().size.width + 2 * padding.width);
            Some(text)
        } else {
            None
        };

        let size = Size::new(width, height);

        let slider_line = Line::new(
            Point::new(
                area.top_left.x + padding.width as i32 + slider_knob_diameter as i32 / 2,
                area.top_left.y + (padding.height + slider_knob_diameter / 2) as i32,
            ),
            Point::new(
                area.top_left.x + (size.width - padding.width - slider_knob_diameter / 2) as i32,
                area.top_left.y + (padding.height + slider_knob_diameter / 2) as i32,
            ),
        );

        let style = ui.style();
        let line_style = PrimitiveStyleBuilder::new()
            .stroke_color(style.border_color)
            .stroke_width(slider_thickness)
            .fill_color(style.text_color)
            .build();
        let slider_knob_style = PrimitiveStyleBuilder::new()
            .stroke_color(style.border_color)
            .stroke_width(1.max(style.border_width as u32))
            .fill_color(style.background_color)
            .build();
        let old_slider_knob_style = PrimitiveStyleBuilder::new()
            .stroke_color(style.background_color)
            .stroke_width(0)
            .fill_color(style.background_color)
            .build();

        if let Some(text) = text.as_mut() {
            let center_offset = area.top_left
                + Point::new(
                    (area.size.width / 2) as i32,
                    area.size.height as i32
                        - text.bounding_box().size.height as i32
                        - padding.height as i32,
                );
            text.translate_mut(center_offset);
        }

        let old_val = *self.value;
        match interaction {
            Interaction::Pressed(point) | Interaction::Drag(point) => {
                let slider_val = lerp_fixed(
                    *self.range.start(),
                    *self.range.end(),
                    point.x as i16 - area.top_left.x as i16,
                    padding.width as i16 + slider_knob_diameter as i16 / 2,
                    width as i16 - padding.width as i16 - slider_knob_diameter as i16 / 2,
                );
                let range_span = (*self.range.end() - *self.range.start()).abs();
                let step_size = self.step_size.clamp(1, range_span as u16) as i16;
                let to_next = slider_val.rem_euclid(step_size);
                let to_prev = step_size - to_next;
                if to_next < to_prev {
                    *self.value = (slider_val - to_next).max(*self.range.start());
                } else {
                    *self.value = (slider_val + to_prev).min(*self.range.end());
                }
            }
            _ => {}
        }

        let slider_knob_pos = lerp_fixed(
            padding.width as i16 + slider_knob_diameter as i16 / 2,
            width as i16 - padding.width as i16 - slider_knob_diameter as i16 / 2,
            *self.value,
            *self.range.start(),
            *self.range.end(),
        );

        let slider_knob = Circle::with_center(
            Point::new(
                area.top_left.x + slider_knob_pos as i32,
                area.top_left.y + padding.height as i32 + (slider_knob_diameter / 2) as i32,
            ),
            slider_knob_diameter,
        );

        let old_slider_knob_pos = lerp_fixed(
            padding.width as i16 + slider_knob_diameter as i16 / 2,
            width as i16 - padding.width as i16 - slider_knob_diameter as i16 / 2,
            old_val,
            *self.range.start(),
            *self.range.end(),
        );

        let old_slider_knob = Circle::with_center(
            Point::new(
                area.top_left.x + old_slider_knob_pos as i32,
                area.top_left.y + padding.height as i32 + (slider_knob_diameter / 2) as i32,
            ),
            slider_knob_diameter + 4,
        );

        let interact_val = match interaction {
            Interaction::Pressed(_) | Interaction::Drag(_) => RenderStatus::Dragging,
            _ => RenderStatus::Inactive,
        };

        let changed = *self.value != old_val;
        if changed || !state.compare_set(interact_val) {
            if old_slider_knob_pos != slider_knob_pos {
                ui.draw(&old_slider_knob.into_styled(old_slider_knob_style))
                    .ok();
            }
            ui.draw(&slider_line.into_styled(line_style)).ok();
            ui.draw(&slider_knob.into_styled(slider_knob_style)).ok();
            if let Some(text) = text.as_mut() {
                ui.draw(text)?;
            }
        }

        Ok(Response::from_change(changed))
    }
}

fn lerp_fixed(start: i16, end: i16, t: i16, min_t: i16, max_t: i16) -> i16 {
    let (start, end, t, min_t, max_t) = (
        start as i32,
        end as i32,
        t as i32,
        min_t as i32,
        max_t as i32,
    );

    let clamped_t = if t < min_t {
        min_t
    } else if t > max_t {
        max_t
    } else {
        t
    };

    let range = max_t - min_t;
    if range == 0 {
        return start as i16;
    }

    let interpolated = start + ((end - start) * (clamped_t - min_t) + (range / 2)) / range;

    interpolated as i16
}
