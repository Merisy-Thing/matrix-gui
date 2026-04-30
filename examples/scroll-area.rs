use std::time::Duration;

use embedded_graphics::geometry::Size;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics_simulator::sdl2::{Keycode, MouseButton};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

use matrix_gui::prelude::*;
use matrix_gui::style::*;
use matrix_gui::ui_font::DEFAULT_FONT_ASCII;

matrix_gui::free_form_region!(
    RegionId,
    (SCROLL_VIEWPORT, 30, 30, 260, 180),
    (TITLE, 80, 4, 160, 24),
);

matrix_gui::grid_layout_column_major_with_start!(
    ScrollItems,
    REGIONID_COUNT,
    (32, 32, 150, (16 * (20 + 2))),
    (16, 1, 2),
    [
        Item0, Item1, Item2, Item3, Item4, Item5, Item6, Item7, Item8, Item9, Item10, Item11,
        Item12, Item13, Item14, Item15
    ]
);

const INNER_REGIONS: &[Region<ScrollItems>; 16] = &[
    *ITEM0, *ITEM1, *ITEM2, *ITEM3, *ITEM4, *ITEM5, *ITEM6, *ITEM7, *ITEM8, *ITEM9, *ITEM10,
    *ITEM11, *ITEM12, *ITEM13, *ITEM14, *ITEM15,
];

const ITEM_LABELS: [&str; 16] = [
    "Item 1 - First",
    "Item 2 - Second",
    "Item 3 - Third",
    "Item 4 - Fourth",
    "Item 5 - Fifth",
    "Item 6 - Sixth",
    "Item 7 - Seventh",
    "Item 8 - Eighth",
    "Item 9 - Ninth",
    "Item 10 - Tenth",
    "Item 11 - Eleventh",
    "Item 12 - Twelfth",
    "Item 13 - Thirteenth",
    "Item 14 - Fourteenth",
    "Item 15 - Fifteenth",
    "Item 16 - Sixteenth",
];

const UI_WIDGET_COUNT: usize = SCROLLITEMS_COUNT + REGIONID_COUNT;
const MAX_SCROLL_Y: i16 =
    SCROLLITEMS_AREA.size.height as i16 - SCROLL_VIEWPORT.area().size.height as i16;
fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<Rgb565>::new(Size::new(320, 240));
    simple_logger::init().ok();

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let mut window = Window::new("ScrollArea Demo", &output_settings);

    let smartstates = RenderState::new_array::<UI_WIDGET_COUNT>();
    let widget_states = WidgetStates::new(&smartstates);

    let style = rgb565_light_style();
    let mut ui = Ui::new_fullscreen(&mut display, &widget_states, &style);
    ui.clear_background().unwrap();

    let mut scroll_state = ScrollState::new();

    let mut mouse_down = false;
    let mut last_down = false;
    let mut location = Point::new(0, 0);
    let mut last_draw_count = 0;

    'outer: loop {
        let mut ui = Ui::new_fullscreen(&mut display, &widget_states, &style);
        #[cfg(feature = "interaction")]
        match (last_down, mouse_down, location) {
            (false, true, loc) => ui.interact(Interaction::Pressed(loc)),
            (true, true, loc) => ui.interact(Interaction::Drag(loc)),
            (true, false, loc) => ui.interact(Interaction::Release(loc)),
            (false, false, _) => {}
        }
        last_down = mouse_down;

        ui.add(
            Label::new(TITLE, "ScrollArea Demo")
                .with_font(DEFAULT_FONT_ASCII)
                .with_align(HorizontalAlign::Center),
        );

        let scroll_area = ScrollArea::new(SCROLL_VIEWPORT, INNER_REGIONS, &mut scroll_state)
            .with_background_color(style.background_color)
            .with_border(style.border_color, 1)
            .with_max_scroll_y(MAX_SCROLL_Y as u16)
            .with_direction(ScrollDirection::Both);

        scroll_area
            .show(&mut ui, |ui, scrolled_regions| {
                for (i, region) in scrolled_regions.iter().enumerate() {
                    let txt = ITEM_LABELS.get(i).unwrap_or(&"out index");
                    ui.add(Label::new(region, txt).with_align(HorizontalAlign::Left));
                }
                Response::Idle
            })
            .unwrap();

        let draw_count = ui.get_debug_counter();
        if draw_count != last_draw_count {
            log::info!("Draw count: {}", draw_count);
            last_draw_count = draw_count;
        }

        window.update(&display);
        std::thread::sleep(Duration::from_millis(16));

        for evt in window.events() {
            match evt {
                SimulatorEvent::MouseButtonDown { mouse_btn, point } => {
                    if mouse_btn == MouseButton::Left {
                        mouse_down = true;
                        location = point;
                    }
                }
                SimulatorEvent::MouseButtonUp { mouse_btn, .. } => {
                    if mouse_btn == MouseButton::Left {
                        mouse_down = false;
                    }
                }
                SimulatorEvent::MouseMove { point } => {
                    location = point;
                }
                SimulatorEvent::MouseWheel { scroll_delta, .. } => {
                    scroll_state.y -= (scroll_delta.y * 15) as i16;
                    scroll_state.y = scroll_state.y.clamp(0, MAX_SCROLL_Y);
                }
                SimulatorEvent::KeyDown { keycode, .. } => match keycode {
                    Keycode::Up => {
                        scroll_state.y = (scroll_state.y - 15).max(0);
                    }
                    Keycode::Down => {
                        scroll_state.y = (scroll_state.y + 15).min(MAX_SCROLL_Y);
                    }
                    _ => {}
                },
                SimulatorEvent::Quit => break 'outer,
                _ => {}
            }
        }
    }
    Ok(())
}
