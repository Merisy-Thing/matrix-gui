//! Static image widget for displaying images.
//!
//! This module provides a widget that displays a static image within
//! a specified region. The image can be shown or hidden dynamically.

use crate::prelude::*;
use embedded_graphics::image::Image;

/// Static image widget for displaying images.
///
/// This widget displays a static image within a specified region.
/// The image can be shown or hidden dynamically using the
/// `visible()` method.
///
/// # Type Parameters
///
/// * `'a` - The lifetime of the image data reference
/// * `ID` - The widget ID type implementing [`WidgetId`]
/// * `T` - The image type implementing [`ImageDrawable`]
pub struct StaticImage<'a, ID, T> {
    /// The region defining the image's position and size.
    region: &'a Region<ID>,
    /// Reference to the raw image data.
    raw_img: &'a T,
    /// Flag indicating whether the image should be shown.
    show: bool,
}

impl<'a, ID: WidgetId, T: ImageDrawable> StaticImage<'a, ID, T> {
    pub const fn new(region: &'a Region<ID>, raw_img: &'a T) -> Self {
        Self {
            region,
            raw_img,
            show: true,
        }
    }

    pub const fn visible(mut self, show: bool) -> Self {
        self.show = show;
        self
    }
}

impl<'a, DRAW, ID: WidgetId, T, COL> Widget<DRAW, COL> for StaticImage<'a, ID, T>
where
    COL: PixelColor,
    DRAW: DrawTarget<Color = COL>,
    T: ImageDrawable<Color = COL>,
{
    fn draw(&mut self, ui: &mut Ui<DRAW, COL>) -> GuiResult<Response> {
        let widget_state = ui.get_widget_state(self.region.id())?;

        if !widget_state.compare_set(RenderStatus::Rendered) {
            let area = self.region.rectangle();
            ui.clear_area(&area)?;

            if self.show {
                let image = Image::new(self.raw_img, area.top_left);
                ui.draw(&image)?;
            }
        }
        Ok(Response::Idle)
    }
}
