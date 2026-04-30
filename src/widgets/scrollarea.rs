use embedded_graphics::{draw_target::DrawTarget, prelude::PixelColor};

use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScrollDirection {
    Vertical,
    Horizontal,
    Both,
}

#[derive(Clone, Debug)]
pub struct ScrollState {
    pub x: i16,
    pub y: i16,
    rendered_x: i16,
    rendered_y: i16,
    drag_start_point: LwPoint<i16>,
    offset_at_start_x: i16,
    offset_at_start_y: i16,
    dragging: bool,
}

impl ScrollState {
    pub const fn new() -> Self {
        Self {
            x: -1,
            y: -1,
            rendered_x: 0,
            rendered_y: 0,
            drag_start_point: LwPoint::new(0, 0),
            offset_at_start_x: 0,
            offset_at_start_y: 0,
            dragging: false,
        }
    }

    pub fn force_redraw(&mut self) {
        self.rendered_x = self.x.wrapping_add(1);
    }
}

impl Default for ScrollState {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ScrollArea<'a, ID, IID, COL, const N: usize> {
    region: &'a Region<ID>,
    inner_regions: &'a [Region<IID>; N],
    scroll_state: &'a mut ScrollState,
    direction: ScrollDirection,
    max_scroll_x: Option<u16>,
    max_scroll_y: Option<u16>,
    bg_color: OptionColor<COL>,
    border_color: Option<COL>,
    border_width: u8,
}

impl<'a, ID: WidgetId, IID: WidgetId, COL: PixelColor, const N: usize>
    ScrollArea<'a, ID, IID, COL, N>
{
    pub fn new(
        region: &'a Region<ID>,
        inner_regions: &'a [Region<IID>; N],
        scroll_state: &'a mut ScrollState,
    ) -> Self {
        Self {
            region,
            inner_regions,
            scroll_state,
            direction: ScrollDirection::Vertical,
            max_scroll_x: None,
            max_scroll_y: None,
            bg_color: OptionColor::none(),
            border_color: None,
            border_width: 0,
        }
    }

    pub const fn with_direction(mut self, direction: ScrollDirection) -> Self {
        self.direction = direction;
        self
    }

    pub const fn with_background_color(mut self, color: COL) -> Self {
        self.bg_color.set_color(color);
        self
    }

    pub const fn with_border(mut self, color: COL, width: u8) -> Self {
        self.border_color = Some(color);
        self.border_width = width;
        self
    }

    pub const fn with_max_scroll_x(mut self, max: u16) -> Self {
        self.max_scroll_x = Some(max);
        self
    }

    pub const fn with_max_scroll_y(mut self, max: u16) -> Self {
        self.max_scroll_y = Some(max);
        self
    }

    #[cfg(feature = "interaction")]
    fn handle_scroll(&mut self, interaction: Interaction) {
        match interaction {
            Interaction::Pressed(point) => {
                self.scroll_state.drag_start_point = point.into();
                self.scroll_state.offset_at_start_x = self.scroll_state.x;
                self.scroll_state.offset_at_start_y = self.scroll_state.y;
                self.scroll_state.dragging = true;
            }
            Interaction::Drag(point) => {
                if self.scroll_state.dragging {
                    let drag_start: LwPoint<i16> = self.scroll_state.drag_start_point;
                    let dx = drag_start.x - point.x as i16;
                    let dy = drag_start.y - point.y as i16;

                    if self.direction != ScrollDirection::Vertical {
                        self.scroll_state.x = self.scroll_state.offset_at_start_x + dx;
                        if let Some(max) = self.max_scroll_x {
                            self.scroll_state.x = self.scroll_state.x.clamp(0, max as i16);
                        }
                    }
                    if self.direction != ScrollDirection::Horizontal {
                        self.scroll_state.y = self.scroll_state.offset_at_start_y + dy;
                        if let Some(max) = self.max_scroll_y {
                            self.scroll_state.y = self.scroll_state.y.clamp(0, max as i16);
                        }
                    }
                }
            }
            Interaction::Release(_) | Interaction::Clicked(_) => {
                self.scroll_state.dragging = false;
            }
            Interaction::None => {}
        }
    }

    pub fn show<DRAW, F>(mut self, ui: &mut Ui<DRAW, COL>, add_contents: F) -> GuiResult<Response>
    where
        DRAW: DrawTarget<Color = COL>,
        F: FnOnce(&mut Ui<DRAW, COL>, &[Region<IID>]) -> Response,
    {
        let widget_id = self.region.id();

        #[cfg(feature = "interaction")]
        {
            let interaction = ui.check_interact(self.region);
            self.handle_scroll(interaction);

            let prevstate = ui.get_widget_state(widget_id)?.status();
            let next_state = match interaction {
                Interaction::None => RenderStatus::Rendered,
                Interaction::Pressed(_) | Interaction::Drag(_) => RenderStatus::Pressed,
                Interaction::Release(_) | Interaction::Clicked(_) => RenderStatus::Released,
            };

            let offset_changed = self.scroll_state.x != self.scroll_state.rendered_x
                || self.scroll_state.y != self.scroll_state.rendered_y;

            if next_state == prevstate && !offset_changed {
                return Ok(interaction.into());
            }
            ui.get_widget_state(widget_id)?.set_status(next_state);
        }

        #[cfg(not(feature = "interaction"))]
        if self.scroll_state.x == self.scroll_state.rendered_x
            && self.scroll_state.y == self.scroll_state.rendered_y
        {
            return Ok(Response::Idle);
        }

        self.scroll_state.rendered_x = self.scroll_state.x;
        self.scroll_state.rendered_y = self.scroll_state.y;

        let mut viewport = self.region.rectangle();

        let bg = self.bg_color.background_color(ui.style());
        ui.clear_area_raw(&viewport, bg)?;

        if let Some(border_color) = self.border_color
            && self.border_width > 0
        {
            let border_style = PrimitiveStyleBuilder::new()
                .stroke_color(border_color)
                .stroke_width(self.border_width as u32)
                .build();
            ui.draw(&viewport.into_styled(border_style)).ok();
            viewport = self
                .region
                .area()
                .delta_resize(DeltaResize::Center(
                    -(self.border_width as i16 * 2),
                    -(self.border_width as i16 * 2),
                ))
                .rectangle();
        }

        let offset_x = self.scroll_state.x;
        let offset_y = self.scroll_state.y;

        let mut scrolled: [Region<IID>; N] = *self.inner_regions;

        for r in &mut scrolled {
            *r = r.move_by(-offset_x, -offset_y);
            ui.force_redraw(r.id());
        }
        ui.set_clipped_area(Some(viewport));
        let response = add_contents(ui, &scrolled);
        ui.set_clipped_area(None);

        Ok(response)
    }
}
