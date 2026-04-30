use std::time::Duration;

use embedded_graphics::geometry::Size;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::Point;
use embedded_graphics_simulator::sdl2::{Keycode, MouseButton};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

use matrix_gui::prelude::*;
use matrix_gui::style::*;

// enum RegionId { .. }
// const REGIONID_COUNT: usize
// (RegionID, x, y, width, height)
// Regions: CHOICE_BTN, RESULT_LABEL
matrix_gui::free_form_region!(
    RegionId,
    (CHOICE_BTN, 28, 8, 140, 36),
    (Apple),
    (Banana),
    (Cherry),
    (Date),
    (CHOICE_OPTIONS_REGION, 84, 64, 158, 158),
    (RESULT_LABEL, 28, 200, 272, 28),
);

matrix_gui::region_id_with_start!(RawID, REGIONID_COUNT, [Background, Empty]);

// enum Btn4x4 { .. }
// const BTN_4X4_COUNT: usize
// const BTN_4X4_AREA: Rectangle
// const BTN_4X4_GLRM: [Region<Btn4x4>; BTN_4X4_COUNT]
#[rustfmt::skip]
matrix_gui::grid_layout_row_major_with_start! (
    BTN_4X4,
    (REGIONID_COUNT + RAWID_COUNT),
    (28, 48, 272, 142),
    (4, 4, 2),
    [
        CELL_0, CELL_1, CELL_2, CELL_3,
        CELL_4, CELL_5, CELL_6, CELL_7,
        CELL_8, CELL_9, CELL_10, CELL_11,
        CELL_12, CELL_13, CELL_14, CELL_15,
    ]
);

const RENDER_STATE_CNT: usize = BTN_4X4_COUNT + REGIONID_COUNT + RAWID_COUNT;

const CHOICE_OPTIONS: &[(&str, RegionId)] = &[
    ("1: Apple", RegionId::Apple),
    ("2: Banana", RegionId::Banana),
    ("3: Cherry", RegionId::Cherry),
    ("4: Date", RegionId::Date),
];

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<Rgb565>::new(Size::new(320, 240));
    simple_logger::init().ok();

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let mut window = Window::new("Choice Demo", &output_settings);
    // input handling variables
    let mut mouse_down = false;
    let mut last_down = false;
    let mut location = Point::new(0, 0);
    let smartstates = RenderState::new_array::<RENDER_STATE_CNT>();
    let widget_states = WidgetStates::new(&smartstates);

    let style = rgb565_light_style();
    let mut ui = Ui::new_fullscreen(&mut display, &widget_states, &style);
    ui.clear_background().unwrap();
    let mut selected_idx = -1_i32;
    let mut focus_state = FocusState::<RENDER_STATE_CNT>::new();

    'outer: loop {
        let mut ui = Ui::new_fullscreen(&mut display, &widget_states, &style);
        ui.draw_widget_bounds_debug(rgb565!(0x7F7F00));

        ui.set_focus_state(&mut focus_state);

        // handle input
        match (last_down, mouse_down, location) {
            (false, true, loc) => {
                ui.interact(Interaction::Pressed(loc));
            }
            (true, true, loc) => {
                ui.interact(Interaction::Drag(loc));
            }
            (true, false, loc) => {
                ui.interact(Interaction::Release(loc));
            }
            (false, false, _) => {}
        }

        last_down = mouse_down;
        // =================================
        ui.add(Background::new(RawId::Background));

        let selected_option_str = CHOICE_OPTIONS
            .get(selected_idx as usize)
            .unwrap_or(&("Select an option", RegionId::Apple))
            .0;

        // Display choice control
        let response = ui.add(Choice::new(
            CHOICE_BTN,
            CHOICE_OPTIONS,
            CHOICE_OPTIONS_REGION,
            selected_option_str,
        ));

        // Handle choice selection
        if let Response::User(idx) = response {
            log::info!(
                "Selected option: {} - {}",
                idx,
                CHOICE_OPTIONS[idx as usize].0
            );
            selected_idx = idx as i32;
            focus_state.clear_focus();
            continue;
        } else if response.is_clicked() {
            focus_state.clear_focus();
            continue;
        }

        // Display result label
        let result_text = format!("Selected: {}", selected_option_str);
        ui.add(Label::new(RESULT_LABEL, &result_text).with_align(HorizontalAlign::Center));

        // Display other buttons for reference
        for (idx, btn) in (&BTN_4X4_GLRM).iter().enumerate() {
            ui.add(Button::new(btn, &format!("{}", idx + 1)));
        }

        log::info!("Debug counter: {}", ui.get_debug_counter());
        // =================================

        // simulator window update
        window.update(&display);
        std::thread::sleep(Duration::from_millis(10));

        // take input, and quit application if necessary
        for evt in window.events() {
            match evt {
                SimulatorEvent::KeyUp { .. } => {}
                SimulatorEvent::KeyDown { keycode, .. } => {
                    if keycode == Keycode::Left || keycode == Keycode::UP {
                        focus_state.focus_prev();
                    } else if keycode == Keycode::Right || keycode == Keycode::DOWN {
                        focus_state.focus_next();
                    } else if keycode == Keycode::Return || keycode == Keycode::Space {
                        focus_state.trigger_focus();
                    }
                }
                SimulatorEvent::MouseButtonUp { mouse_btn, point } => {
                    if let MouseButton::Left = mouse_btn {
                        mouse_down = false;
                    }
                    location = point;
                }
                SimulatorEvent::MouseButtonDown { mouse_btn, point } => {
                    if let MouseButton::Left = mouse_btn {
                        mouse_down = true;
                    }
                    location = point;
                }
                SimulatorEvent::MouseWheel { .. } => {}
                SimulatorEvent::MouseMove { point } => {
                    location = point;
                }
                SimulatorEvent::Quit => {
                    break 'outer;
                }
            }
        }
    }
    Ok(())
}
