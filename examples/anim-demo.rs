//! Animation example demonstrating the animation subsystem.
//!
//! This example shows how to use the animation module to create
//! smooth transitions and effects for GUI elements.

use core::time::Duration;
use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use matrix_gui::animation::{AnimState as AnimPlaybackState, Animations};
use matrix_gui::prelude::*;

fn main() -> Result<(), core::convert::Infallible> {
    simple_logger::init().ok();

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
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
        .with_repeat(3);
    // 2. Color animation (Green channel: 0 to 63 for Rgb565)
    let color_anim = Anim::new(0, 63, Duration::from_millis(1500))
        .with_easing(Easing::Linear)
        .with_reverse(true)
        .with_repeat(10); // Infinite repeat

    let move_id = anim_manager
        .add(move_anim)
        .expect("Failed to add move animation");

    let color_id = anim_manager
        .add(color_anim)
        .expect("Failed to add color animation");

    anim_manager.play(move_id);
    anim_manager.play(color_id);

    let mut last_time = std::time::Instant::now();

    'running: loop {
        let now = std::time::Instant::now();
        let delta = now.duration_since(last_time);
        last_time = now;

        anim_manager.tick(delta);

        display.clear(Rgb565::BLACK).unwrap();

        let widget_x = anim_status[move_id as usize].get().unwrap();
        let widget_g = anim_status[color_id as usize].get().unwrap() as u8;

        // Combine red (fixed) and animated green channel
        let current_color = Rgb565::new(31, widget_g, 0);

        let style = PrimitiveStyleBuilder::new()
            .fill_color(current_color)
            .build();

        Rectangle::new(Point::new(widget_x, 100), Size::new(50, 50))
            .into_styled(style)
            .draw(&mut display)
            .unwrap();

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

        std::thread::sleep(std::time::Duration::from_millis(16));
    }

    Ok(())
}
