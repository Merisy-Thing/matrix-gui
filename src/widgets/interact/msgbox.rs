use crate::prelude::*;
use embedded_graphics::primitives::PrimitiveStyleBuilder;

/// A simple message box with a title, message, and an OK button.
pub struct MessageBox<'a, ID: WidgetId> {
    region: &'a Region<ID>,
    ok_btn: Option<&'a Region<ID>>,
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

    pub const fn with_ok_btn(mut self, ok_btn: &'a Region<ID>) -> Self {
        self.ok_btn = Some(ok_btn);
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

struct MessageBoxContent<'a, ID: WidgetId> {
    region: &'a Region<ID>,
    ok_btn: Option<&'a Region<ID>>,
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
        if let Some(ok_region) = self.ok_btn {
            ok_btn_offset = self.region.y() + self.region.height() as i32 - ok_region.y();
            let mut ok_btn = Button::new(ok_region, "OK");
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

        let title_region =
            self.region
                .resized(self.region.width() as u16, 32, AnchorPoint::TopLeft);

        let padding_w = ui.style().default_padding.width as i16;
        let msg_region = self
            .region
            .delta_resize(DeltaResize::TopLeft(0, -(ok_btn_offset as i16)))
            .delta_resize(DeltaResize::BottomLeft(-(padding_w * 2), -32))
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
