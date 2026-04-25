use crate::prelude::*;
use crate::ui::GuiResult;

/// A modal dialog widget that dims the background and centers its content.
pub struct Modal<'a, W> {
    area: Rectangle,
    content: W,
    _phantom: core::marker::PhantomData<&'a ()>,
}

impl<'a, W> Modal<'a, W> {
    /// Creates a new modal dialog.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for the modal (usually a widget ID).
    /// * `show` - Whether the modal is currently open.
    /// * `content` - The widget to display inside the modal.
    pub const fn new(area: Rectangle, content: W) -> Self {
        Self {
            area,
            content,
            _phantom: core::marker::PhantomData,
        }
    }
}

impl<'a, DRAW, COL, W> Widget<DRAW, COL> for Modal<'a, W>
where
    DRAW: DrawTarget<Color = COL>,
    COL: PixelColor,
    W: Widget<DRAW, COL>,
{
    fn draw(&mut self, ui: &mut Ui<DRAW, COL>) -> GuiResult<Response> {
        if !ui.is_modal_active() {
            ui.clear_area(&self.area.into())?;
        }

        // Begin modal context
        ui.set_modal_active(false);

        // Draw content
        let response = self.content.draw(ui)?;

        // End modal context
        if !response.is_idle() {
            ui.force_redraw_all();
        } else {
            ui.set_modal_active(true);
        }

        Ok(response)
    }
}
