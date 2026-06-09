//! Message box widget for displaying modal dialogs.
//!
//! This module provides a message box widget that displays a modal dialog
//! with a title, message text, and an optional OK button. It uses the
//! [`Modal`] system to dim the background and block interaction with
//! other widgets while the message box is open.

use crate::prelude::*;
use embedded_graphics::primitives::PrimitiveStyleBuilder;

/// A modal message box with a title, message, and an optional OK button.
///
/// This widget wraps its content in a [`Modal`], dimming the background and
/// blocking interaction with other widgets. When the OK button (if configured)
/// is clicked, the message box returns [`Response::Clicked`].
///
/// # Type Parameters
///
/// * `'a` - The lifetime of the string and region references
/// * `ID` - The widget ID type implementing [`WidgetId`]
pub struct MessageBox<'a, ID: WidgetId> {
    region: &'a Region<ID>,
    ok_btn: Option<(&'a Region<ID>, &'a str)>,
    title: &'a str,
    message: &'a str,
}

impl<'a, ID: WidgetId> MessageBox<'a, ID> {
    pub const fn new(region: &'a Region<ID>, title: &'a str, message: &'a str) -> Self {
        Self {
            region,
            ok_btn: None,
            title,
            message,
        }
    }

    pub const fn with_ok_btn(mut self, ok_btn: &'a Region<ID>, ok_btn_text: &'a str) -> Self {
        self.ok_btn = Some((ok_btn, ok_btn_text));
        self
    }
}

impl<'a, DRAW, COL, ID: WidgetId> Widget<DRAW, COL> for MessageBox<'a, ID>
where
    DRAW: DrawTarget<Color = COL>,
    COL: PixelColor + From<Rgb565>,
{
    fn draw(&mut self, ui: &mut Ui<DRAW, COL>) -> GuiResult<Response> {
        // Use Modal to handle interaction blocking and global state
        let mut modal = Modal::new(
            self.region.rectangle(),
            MessageBoxContent {
                region: self.region,
                ok_btn: self.ok_btn,
                title: self.title,
                message: self.message,
            },
        );

        modal.draw(ui)
    }
}

/// Internal content widget rendered inside a [`Modal`] by [`MessageBox`].
struct MessageBoxContent<'a, ID: WidgetId> {
    region: &'a Region<ID>,
    ok_btn: Option<(&'a Region<ID>, &'a str)>,
    title: &'a str,
    message: &'a str,
}

impl<'a, DRAW, COL, ID: WidgetId> Widget<DRAW, COL> for MessageBoxContent<'a, ID>
where
    DRAW: DrawTarget<Color = COL>,
    COL: PixelColor + From<Rgb565>,
{
    fn draw(&mut self, ui: &mut Ui<DRAW, COL>) -> GuiResult<Response> {
        let mut ok_btn_offset = 0;
        if let Some((ok_region, ok_btn_text)) = self.ok_btn {
            ok_btn_offset = self.region.y() + self.region.height() as i32 - ok_region.y();
            let mut ok_btn = Button::new(ok_region, ok_btn_text);
            let resp = ok_btn.draw(ui)?;
            if resp.is_clicked() {
                return Ok(Response::Clicked);
            }
        }

        let widget_state = ui.get_widget_state(self.region.id())?;
        if widget_state.compare_set(RenderStatus::Rendered) {
            return Ok(Response::Idle);
        }
        let style = ui.style();

        let title_font = style
            .default_font
            .first()
            .unwrap_or(&crate::ui_font::DEFAULT_FONT_ASCII[0]);
        let title_height =
            title_font.character_size.height as u16 + style.default_padding.height as u16 * 2;
        let title_region = self.region.resized(
            self.region.width() as u16,
            title_height,
            AnchorPoint::TopLeft,
        );

        let padding_w = ui.style().default_padding.width as i16;
        let msg_region = self
            .region
            .delta_resize(DeltaResize::TopLeft(0, -(ok_btn_offset as i16)))
            .delta_resize(DeltaResize::BottomLeft(
                -(padding_w * 2),
                -(title_height as i16),
            ))
            .move_by(padding_w, 0);

        let border_style = PrimitiveStyleBuilder::new()
            .stroke_color(style.border_color)
            .stroke_width(1)
            .build();
        // Draw title
        ui.force_redraw(self.region.id());
        let mut title_label: Label<ID, COL> =
            Label::new(&title_region, self.title).with_align(HorizontalAlign::Center);
        title_label.draw(ui)?;
        ui.draw(&title_region.rectangle().into_styled(border_style))?;

        // Draw message
        ui.force_redraw(self.region.id());
        let mut msg_label: Label<ID, COL> = Label::new(&msg_region, self.message);
        msg_label.draw(ui)?;

        ui.draw(&self.region.rectangle().into_styled(border_style))?;

        Ok(Response::Idle)
    }
}
