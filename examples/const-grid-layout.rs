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
    GridLayout_Test,
    [
        TestID0, TestID1, TestID2, TestID3, TestID4, TestID5, TestID6, TestID7, TestID8, TestID9,
        TestID10, TestID11, TestID12, TestID13
    ]
);

matrix_gui::region_id_with_start!(
    GridLayout_Btn,
    GRIDLAYOUT_TEST_COUNT,
    [
        BtnID0, BtnID1, BtnID2, BtnID3, BtnID4, BtnID5, BtnID6, BtnID7, BtnID8, BtnID9, BtnID10,
        BtnID11, BtnID12, BtnID13, BtnID14,
    ]
);

const STATE_COUNT: usize = GRIDLAYOUT_TEST_COUNT + GRIDLAYOUT_BTN_COUNT;

const REGIONS_COL_MAJOR_RECT: Rectangle = Rectangle::new(Point::new(5, 0), Size::new(400, 180));
const REGIONS_COL_MAJOR: [Region<GridLayoutTest>; GRIDLAYOUT_TEST_COUNT] =
    region::const_grid_layout_column_major(
        &GridLayoutTest::all(),
        &REGIONS_COL_MAJOR_RECT,
        6,
        4,
        3,
    );

const REGIONS_ROW_MAJOR: [Region<GridLayoutBtn>; GRIDLAYOUT_BTN_COUNT] =
    region::const_grid_layout_row_major(
        &GridLayoutBtn::all(),
        &Rectangle::new(Point::new(16, 185), Size::new(480, 180)),
        6,
        4,
        3,
    );

matrix_gui::grid_layout_column_major!(
    GridLayout_COL_MAJOR,
    (32, 345, 280, 60),
    (2, 3, 6),
    [ColID0, ColID1, ColID2, ColID3, ColID4,]
);

matrix_gui::grid_layout_column_major_with_start!(
    GridLayout_COL_MAJOR_WITH_START,
    GRIDLAYOUT_COL_MAJOR_COUNT,
    (32 + 300, 345, 280, 60),
    (2, 3, 6),
    [
        ColWithStartID0,
        ColWithStartID1,
        ColWithStartID2,
        ColWithStartID3,
        ColWithStartID4,
    ]
);
// const KEY_PAD_COUNT: usize
// const KEY_PAD_AREA: Rectangle
// const KEY_PAD_GLRM: [Region<_>; KEY_PAD_COUNT]
#[rustfmt::skip]
matrix_gui::grid_layout_column_major! (
    KEY_PAD,
    (440, 5, 180, 178),
    (4, 3, 2),
    [
        KP_7, KP_4, KP_1, KP_0,
        KP_8, KP_5, KP_2, KP_DOT,
        KP_9, KP_6, KP_3, 
    ]
);

matrix_gui::grid_layout_row_major!(
    GridLayout_ROW_MAJOR,
    (64, 420, 240, 90),
    (3, 2, 12),
    [RowID0, RowID1, RowID2, RowID3, RowID4,]
);

matrix_gui::grid_layout_row_major_with_start!(
    GridLayout_ROW_MAJOR_WITH_START,
    GRIDLAYOUT_ROW_MAJOR_COUNT,
    (64 + 280, 420, 240, 90),
    (3, 2, 12),
    [
        RowWithStartID0,
        RowWithStartID1,
        RowWithStartID2,
        RowWithStartID3,
        RowWithStartID4,
    ]
);

fn main() -> Result<(), core::convert::Infallible> {
    let screen_size = Size::new(640, 540);
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

    log::info!("GRIDLAYOUT_TEST_COUNT = {}", GRIDLAYOUT_TEST_COUNT);
    log::info!("GRIDLAYOUT_BTN_COUNT  = {}", GRIDLAYOUT_BTN_COUNT);

    let style = rgb565_light_style();
    let mut ui = Ui::new_fullscreen(&mut display, &widget_states, &style);
    ui.clear_background().unwrap();

    'outer: loop {
        let mut ui = Ui::new_fullscreen(&mut display, &widget_states, &style);
        ui.draw_widget_bounds_debug(rgb565!(0x7F7F00));

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

        widget_states.force_redraw_all();
        for region in REGIONS_COL_MAJOR {
            let label = format!("Column {}", region.id().id());
            ui.add(Label::new(&region, &label));
        }

        widget_states.force_redraw_all();
        for region in REGIONS_ROW_MAJOR {
            let label = format!("Row {}", region.id().id() - GRIDLAYOUT_TEST_COUNT);
            ui.add(Button::new(&region, &label));
        }

        widget_states.force_redraw_all();
        ui.add(Button::new(COLID0, "Column 0"));
        ui.add(Button::new(COLID1, "Column 1"));
        ui.add(Button::new(COLID2, "Column 2"));
        ui.add(Button::new(COLID3, "Column 3"));
        ui.add(Button::new(COLID4, "Column 4"));

        ui.add(Button::new(COLWITHSTARTID0, "Col0 WS"));
        ui.add(Button::new(COLWITHSTARTID1, "Col1 WS"));
        ui.add(Button::new(COLWITHSTARTID2, "Col2 WS"));
        ui.add(Button::new(COLWITHSTARTID3, "Col3 WS"));
        ui.add(Button::new(COLWITHSTARTID4, "Col4 WS"));

        widget_states.force_redraw_all();
        ui.add(Button::new(ROWID0, "Row 0"));
        ui.add(Button::new(ROWID1, "Row 1"));
        ui.add(Button::new(ROWID2, "Row 2"));
        ui.add(Button::new(ROWID3, "Row 3"));
        ui.add(Button::new(ROWID4, "Row 4"));

        ui.add(Button::new(ROWWITHSTARTID0, "Row 0 WS"));
        ui.add(Button::new(ROWWITHSTARTID1, "Row 1 WS"));
        ui.add(Button::new(ROWWITHSTARTID2, "Row 2 WS"));
        ui.add(Button::new(ROWWITHSTARTID3, "Row 3 WS"));
        ui.add(Button::new(ROWWITHSTARTID4, "Row 4 WS"));

        widget_states.force_redraw_all();
        ui.add(Button::new(KP_7, "7"));
        ui.add(Button::new(KP_8, "8"));
        ui.add(Button::new(KP_9, "9"));
        ui.add(Button::new(KP_4, "4"));
        ui.add(Button::new(KP_5, "5"));
        ui.add(Button::new(KP_6, "6"));

        ui.add(Button::new(KP_1, "1"));
        ui.add(Button::new(KP_2, "2"));
        ui.add(Button::new(KP_3, "3"));

        ui.add(Button::new(KP_0, "0"));
        ui.add(Button::new(KP_DOT, "."));

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
