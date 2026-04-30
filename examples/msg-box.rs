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
// Regions: MSG_BOX, MSG_BOX_OK
matrix_gui::free_form_region!(
    RegionId,
    (MSG_BOX, 27, 23, 266, 143),
    (MSG_BOX_OK, 119, 110, 82, 40),
);

matrix_gui::region_id_with_start!(RawID, REGIONID_COUNT, [Background, Empty]);

// enum Btn4X6 { .. }
// const BTN_4X6_COUNT: usize
// const BTN_4X6_AREA: Rectangle
// const RECT_1_BTN_4X6_GLRMGLRM: [Region<Rect1>; RECT_1_COUNT]
#[rustfmt::skip]
matrix_gui::grid_layout_row_major_with_start! (
    BTN_4X6,
    (REGIONID_COUNT + RAWID_COUNT),
    (19, 16, 286, 209),
    (4, 6, 2),
    [
        CELL_0, CELL_1, CELL_2, CELL_3, CELL_4, CELL_5,
        CELL_6, CELL_7, CELL_8, CELL_9, CELL_10, CELL_11,
        CELL_12, CELL_13, CELL_14, CELL_15, CELL_16, CELL_17,
        CELL_18, CELL_19, CELL_20, CELL_21, CELL_22, CELL_23,
    ]
);

const RENDER_STATE_CNT: usize = BTN_4X6_COUNT + REGIONID_COUNT + RAWID_COUNT;

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<Rgb565>::new(Size::new(320, 240));
    simple_logger::init().ok();

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let mut window = Window::new("Hello World", &output_settings);
    // input handling variables
    let mut mouse_down = false;
    let mut last_down = false;
    let mut location = Point::new(0, 0);
    let smartstates = RenderState::new_array::<RENDER_STATE_CNT>();
    let widget_states = WidgetStates::new(&smartstates);

    let style = rgb565_light_style();
    let mut ui = Ui::new_fullscreen(&mut display, &widget_states, &style);
    ui.clear_background().unwrap();
    let mut show_msg_box = false;
    let mut focus_state = FocusState::<26>::new();

    'outer: loop {
        let mut ui = Ui::new_fullscreen(&mut display, &widget_states, &style);
        //ui.draw_widget_bounds_debug(rgb565!(0x7F7F00));

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

        if !show_msg_box {
            if ui.add(Button::new(&BTN_4X6_GLRM[0], "Show")).is_clicked() {
                log::info!("BTN0 clicked");
                show_msg_box = true;
                focus_state.clear_focus();
                continue;
            }
            for (idx, btn) in (&BTN_4X6_GLRM[1..]).iter().enumerate() {
                ui.add(Button::new(btn, &format!("{}", idx + 1)));
            }
        } else {
            // NOTE: popup widget place at last position
            let response = ui.add(
                MessageBox::new(MSG_BOX, "This is title", "Hello World\nHello Matrix GUI!")
                    .with_ok_btn(MSG_BOX_OK, "OoooK"),
            );
            if response.is_clicked() {
                log::info!("OK msg clicked");
                show_msg_box = false;
                focus_state.clear_focus();
                continue;
            }
        }
        // =================================

        // simulator window update
        window.update(&display);
        std::thread::sleep(Duration::from_millis(100));

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
                    if show_msg_box {
                        show_msg_box = false;
                        widget_states.force_redraw_all();
                        log::info!("msg box hidden");
                    } else {
                        break 'outer;
                    }
                }
            }
        }
    }
    Ok(())
}
