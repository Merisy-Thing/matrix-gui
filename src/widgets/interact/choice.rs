use crate::prelude::*;

/// A choice control that displays as a button and shows a modal with options when pressed.
///
/// This widget displays as a button showing the currently selected option (or default text).
/// When pressed, it opens a modal dialog containing a vertical list of buttons representing
/// each option. The first button's region is provided, and subsequent buttons are positioned
/// vertically below it based on the button height and padding from the style.
///
/// # Type Parameters
///
/// * `'a` - The lifetime of the options array and region reference
/// * `ID` - The widget ID type implementing [`WidgetId`]
pub struct Choice<'a, 'b, ID: WidgetId> {
    region: &'a Region<ID>,
    options: &'a [(&'b str, ID)],
    options_region: &'a Region<ID>,
    label: &'a str,
}

impl<'a, 'b, ID: WidgetId> Choice<'a, 'b, ID> {
    pub const fn new(
        region: &'a Region<ID>,
        options: &'a [(&'b str, ID)],
        options_region: &'a Region<ID>,
        label: &'a str,
    ) -> Self
    where
        'a: 'b,
    {
        Self {
            region,
            options,
            options_region,
            label,
        }
    }
}

impl<'a, 'b, DRAW, COL, ID: WidgetId> Widget<DRAW, COL> for Choice<'a, 'b, ID>
where
    DRAW: DrawTarget<Color = COL>,
    COL: PixelColor + From<Rgb565>,
    'a: 'b,
{
    fn draw(&mut self, ui: &mut Ui<DRAW, COL>) -> GuiResult<Response> {
        if !ui.get_widget_internal_flag(self.region.id())? {
            let mut button = Button::new(self.region, self.label);
            let resp = button.draw(ui)?;
            if resp.is_clicked() {
                ui.set_widget_internal_flag(self.region.id(), true)?;
            }
            Ok(resp)
        } else {
            let mut modal = Modal::new(
                self.options_region.rectangle(),
                ChoiceContent {
                    region: self.options_region,
                    options: self.options,
                },
            );
            let response = modal.draw(ui)?;

            if let Response::User(idx) = response {
                ui.set_widget_internal_flag(self.region.id(), false)?;
                Ok(Response::User(idx))
            } else {
                Ok(response)
            }
        }
    }
}

struct ChoiceContent<'a, 'b, ID: WidgetId> {
    region: &'a Region<ID>,
    options: &'a [(&'b str, ID)],
}

impl<'a, 'b, DRAW, COL, ID: WidgetId> Widget<DRAW, COL> for ChoiceContent<'a, 'b, ID>
where
    DRAW: DrawTarget<Color = COL>,
    COL: PixelColor + From<Rgb565>,
    'a: 'b,
{
    fn draw(&mut self, ui: &mut Ui<DRAW, COL>) -> GuiResult<Response> {
        let padding = ui.style().default_padding.height as u16;

        let redraw = !ui.get_widget_internal_flag(self.region.id())?;
        let mut opt_region = self.region.clone();
        let opt_item_height =
            (opt_region.height() as u16) / (self.options.len().max(1) as u16) - padding;
        opt_region = opt_region
            .resized(
                opt_region.width() as u16 - padding * 2,
                opt_item_height,
                AnchorPoint::TopLeft,
            )
            .move_by(padding as i16, padding as i16);
        let delta_y = (opt_item_height + padding) as i16;

        let mut idx = 0_u8;
        for (option_text, option_id) in self.options {
            if redraw {
                ui.get_widget_state(self.region.id())?.force_redraw();
            }

            let btn_region = opt_region.replace_id(*option_id);
            let mut button = Button::new(&btn_region, option_text);
            let response = button.draw(ui)?;

            if response.is_clicked() {
                ui.get_widget_state(self.region.id())?.set_interflag(false);
                return Ok(Response::User(idx));
            }
            idx += 1;

            opt_region = opt_region.move_by(0, delta_y);
        }
        ui.get_widget_state(self.region.id())?.set_interflag(true);

        Ok(Response::Idle)
    }
}
