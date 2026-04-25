//! Background widget for clearing the entire screen.
//!
//! This module provides a widget that clears the entire background area
//! of the UI. It's typically used as the first widget in a UI layout
//! to ensure a clean background before drawing other widgets.

use crate::prelude::*;

/// Background widget that clears the entire screen.
///
/// This widget clears the entire background area of the UI using the
/// background color from the current style. It's typically used as
/// the first widget in a UI layout to ensure a clean background.
///
/// # Type Parameters
///
/// * `ID` - The widget ID type implementing [`WidgetId`]
pub struct Background<ID: WidgetId> {
    /// The widget identifier.
    widget_id: ID,
}

impl<ID: WidgetId> Background<ID> {
    pub const fn new(widget_id: ID) -> Background<ID> {
        Background { widget_id }
    }
}

impl<DRAW: DrawTarget<Color = COL>, ID: WidgetId, COL: PixelColor> Widget<DRAW, COL>
    for Background<ID>
{
    fn draw(&mut self, ui: &mut Ui<DRAW, COL>) -> GuiResult<Response> {
        let widget_state = ui.get_widget_state(self.widget_id)?;
        if widget_state.compare_set(RenderStatus::Rendered) {
            return Ok(Response::Idle);
        }

        ui.clear_background()?;

        Ok(Response::Idle)
    }
}
