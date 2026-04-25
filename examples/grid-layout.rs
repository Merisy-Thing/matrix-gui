use std::time::Duration;

use embedded_graphics::geometry::Size;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::Point;
use embedded_graphics_simulator::sdl2::MouseButton;
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

use matrix_gui::prelude::*;
use matrix_gui::style::*;

matrix_gui::region_id!(
    GridLayoutTest,
    [
        TestID0, TestID1, TestID2, TestID3, TestID4, TestID5, TestID6, TestID7, TestID8, TestID9,
        TestID10, TestID11, TestID12, TestID13
    ]
);

matrix_gui::region_id_with_start!(
    GridLayoutBtn,
    GRIDLAYOUTTEST_COUNT,
    [
        BtnID0, BtnID1, BtnID2, BtnID3, BtnID4, BtnID5, BtnID6, BtnID7, BtnID8, BtnID9, BtnID10,
        BtnID11, BtnID12, BtnID13, BtnID14,
    ]
);

const STATE_COUNT: usize = GRIDLAYOUTTEST_COUNT + GRIDLAYOUTBTN_COUNT;

fn main() -> Result<(), core::convert::Infallible> {
    let screen_size = Size::new(640, 480);
    let mut display = SimulatorDisplay::<Rgb565>::new(screen_size);
    simple_logger::init().ok();

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let mut window = Window::new("Hello World", &output_settings);
    // input handling variables
    let mut mouse_down = false;
    let mut last_down = false;
    let mut location = Point::new(0, 0);
    let smartstates = RenderState::new_array::<STATE_COUNT>();
    let widget_states = WidgetStates::new(&smartstates);
    let mut regions_col_major = [Region::default(); GRIDLAYOUTTEST_COUNT];

    log::info!("GRIDLAYOUTTEST_COUNT = {}", GRIDLAYOUTTEST_COUNT);
    log::info!("GRIDLAYOUTBTN_COUNT  = {}", GRIDLAYOUTBTN_COUNT);

    let grid_layout_size = Size::new(480, 200);
    region::grid_layout_column_major_mut(
        GridLayoutTest::TestId0,
        &Rectangle::new(Point::new(32, 0), grid_layout_size),
        6,
        4,
        10,
        &mut regions_col_major,
    );

    let mut regions_row_major = [Region::default(); GRIDLAYOUTBTN_COUNT];
    region::grid_layout_row_major_mut(
        GridLayoutBtn::BtnId0,
        &Rectangle::new(Point::new(32, 240), grid_layout_size),
        6,
        4,
        10,
        &mut regions_row_major,
    );

    let style = rgb565_light_style();
    let mut ui = Ui::new_fullscreen(&mut display, &widget_states, &style);
    ui.clear_background().unwrap();

    'outer: loop {
        let mut ui = Ui::new_fullscreen(&mut display, &widget_states, &style);
        //ui.draw_widget_bounds_debug(rgb565!(0x7F7F00));

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

        for region in regions_col_major {
            let label = format!("Button {}", region.id().id());
            if region.id() != GridLayoutTest::TestId13 {
                ui.add(Button::new(&region, &label));
            } else {
                ui.lazy_draw(GridLayoutTest::TestId13, |ui| {
                    let resized_region = region.delta_resize(DeltaResize::TopRight(-12, 120));
                    ui.add(Button::new(&resized_region, &label))
                });
            }
        }

        for region in regions_row_major {
            let label = format!("Button {}", region.id().id());
            ui.add(Button::new(&region, &label));
        }

        // =================================

        // simulator window update
        window.update(&display);
        std::thread::sleep(Duration::from_millis(10));

        // take input, and quit application if necessary
        for evt in window.events() {
            match evt {
                SimulatorEvent::KeyUp { .. } => {}
                SimulatorEvent::KeyDown { .. } => {}
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
                SimulatorEvent::Quit => break 'outer,
            }
        }
    }
    Ok(())
}
