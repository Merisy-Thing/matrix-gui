//! Animation example demonstrating the animation subsystem.
//!
//! This example shows how to use the animation module to create
//! smooth transitions and effects for GUI elements.

use core::time::Duration;
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use matrix_gui::animation::{AnimState as AnimPlaybackState, Animations};
use matrix_gui::prelude::*;
use matrix_gui::style::*;

//  enum RegionId
// const REGIONID_COUNT
// Region (RegionID, x, y, width, height)
matrix_gui::free_form_region!(RegionId, (BACKGROUND), (ANIM_RECT, 10, 100, 310, 30));

pub struct BufferedImage<'a, ID> {
    region: &'a Region<ID>,
    move_id: AnimId,
    color_id: AnimId,
}

impl<'a, ID: WidgetId> BufferedImage<'a, ID> {
    pub fn new(region: &'a Region<ID>, move_id: AnimId, color_id: AnimId) -> Self {
        Self {
            region,
            move_id,
            color_id,
        }
    }
}

impl<'a, DRAW, ID: WidgetId> Widget<DRAW, Rgb565> for BufferedImage<'a, ID>
where
    DRAW: DrawTarget<Color = Rgb565>,
{
    fn draw(&mut self, ui: &mut Ui<DRAW, Rgb565>) -> GuiResult<Response> {
        let widget_state = ui.get_widget_state(self.region.id())?;
        let move_state = ui.take_anim_status(self.move_id)?;
        let color_state = ui.take_anim_status(self.color_id)?;

        if move_state.is_none() || color_state.is_none() {
            if widget_state.compare_set(RenderStatus::Rendered) {
                return Ok(Response::Idle);
            }
        }

        let mut area = self.region.rectangle();
        ui.clear_area(&area)?;
        let move_state = move_state.unwrap_or_default();
        let color_state = color_state.unwrap_or_default();

        let current_color = Rgb565::new(31, color_state as u8, 0);

        let style = PrimitiveStyleBuilder::new()
            .fill_color(current_color)
            .build();
        area.top_left.x += move_state;
        area.size.width = 50;

        let styled_rect = area.into_styled(style);

        ui.draw(&styled_rect)?;

        Ok(Response::Idle)
    }
}

fn main() -> Result<(), core::convert::Infallible> {
    simple_logger::init().ok();

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(320, 240));
    let mut window = Window::new("Animation Demo", &output_settings);

    let (mut anim_instance, anim_status) = Animations::<2>::new().split();
    let mut anim_manager = AnimManager::new(&mut anim_instance, &anim_status);

    log::info!(
        "AnimManager memory size: {} bytes",
        std::mem::size_of_val(&anim_manager)
    );

    // 1. Position animation (X coordinate)
    let move_anim = Anim::new(50, 220, Duration::from_millis(2000))
        .with_easing(Easing::EaseInOut)
        .with_reverse(true)
        .with_repeat(0);
    // 2. Color animation (Green channel: 0 to 63 for Rgb565)
    let color_anim = Anim::new(0, 63, Duration::from_millis(2000))
        .with_easing(Easing::Linear)
        .with_reverse(true)
        .with_repeat(0); // Infinite repeat

    let move_id = anim_manager
        .add(move_anim)
        .expect("Failed to add move animation");

    let color_id = anim_manager
        .add(color_anim)
        .expect("Failed to add color animation");

    anim_manager.play(move_id);
    anim_manager.play(color_id);

    let mut last_time = std::time::Instant::now();
    let smartstates = RenderState::new_array::<REGIONID_COUNT>();
    let widget_states = WidgetStates::new(&smartstates, &anim_status);

    let style = rgb565_light_style();

    'running: loop {
        let now = std::time::Instant::now();
        let delta = now.duration_since(last_time);
        last_time = now;

        anim_manager.tick(delta);

        let mut ui = Ui::new_fullscreen(&mut display, &widget_states, &style);

        ui.add(Background::new(RegionId::Background));
        ui.add(BufferedImage::new(&ANIM_RECT, move_id, color_id));

        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                SimulatorEvent::KeyDown { .. } => {
                    // Toggle both animations
                    if anim_manager.get_state(move_id) == Some(AnimPlaybackState::Paused) {
                        anim_manager.resume(move_id);
                        anim_manager.resume(color_id);
                    } else {
                        anim_manager.pause(move_id);
                        anim_manager.pause(color_id);
                    }
                }
                _ => {}
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(41));
    }

    Ok(())
}
