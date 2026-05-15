use std::time::Duration;

use embedded_graphics::geometry::Size;
use embedded_graphics::image::ImageRaw;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::Point;
use embedded_graphics_simulator::sdl2::{Keycode, MouseButton};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

use matrix_gui::prelude::*;
use matrix_gui::style::*;
use matrix_gui::widgets::listbox::ListBox;
use multi_mono_font::{CharSize, GlyphData, MultiMonoFont, mapping::StrGlyphMapping};

// --------------------
// 字体: 宋体, 12
// 偏置: x:0 y:0 旋转:0
// 点阵格式: 阴码-1亮0灭
// 取模方式: 逐行
// 取模走向: MSB-高位在前
// 显示: 单色 阈值：64
// --------------------
const GB2313_TIER1_16X16_FONT: MultiMonoFont = MultiMonoFont {
    glyph_data: GlyphData::ImgRaw(ImageRaw::new(
        include_bytes!("../assets/GB2313_Tier1_16x16_11.bin"),
        3600,
    )),
    glyph_mapping: &StrGlyphMapping::new(include_str!("../assets/GB2313_Tier1.txt"), 0),
    character_size: CharSize::new(16, 16),
    character_spacing: 0,
    baseline: 16,
};
const TXT_FONT: UiFont = &[&multi_mono_font::ascii::FONT_9X18, &GB2313_TIER1_16X16_FONT];

//  enum RegionId
// const REGIONID_COUNT
// Region (RegionID, x, y, width, height)
matrix_gui::free_form_region!(
    RegionId,
    (LIST_BOX1, 0, 0, 320, 128),
    (LIST_BOX2, 0, 122, 320, 120)
);

const TEXT_LIST1: &[&str] = &[
    "你好，世界！",
    "Hello, World!",
    "Line 3",
    "Line 4",
    "第5行",
    "第6行",
];
const TEXT_LIST2: &[&str] = &["Line 1", "Line 2", "第3行", "第4行"];

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<Rgb565>::new(Size::new(640, 480));
    simple_logger::init().ok();

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let mut window = Window::new("Hello World", &output_settings);
    // input handling variables
    let mut mouse_down = false;
    let mut last_down = false;
    let mut location = Point::new(0, 0);
    let smartstates = RenderState::new_array::<REGIONID_COUNT>();
    let widget_states = WidgetStates::new(&smartstates);

    let style = rgb565_light_style();
    let mut ui = Ui::new_fullscreen(&mut display, &widget_states, &style);
    ui.clear_background().unwrap();
    let mut selected = 0;

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

        ui.add(
            ListBox::new(&LIST_BOX1, TEXT_LIST1)
                .with_font(TXT_FONT)
                .with_align(HorizontalAlign::Left)
                .with_padding(8, 8)
                .with_border(1)
                .with_start(3)
                .with_selected_item(selected),
        );
        ui.add(
            ListBox::new(&LIST_BOX2, TEXT_LIST2)
                .with_font(TXT_FONT)
                .with_align(HorizontalAlign::Left)
                .with_padding(8, 8)
                .with_border(1)
                .with_selected_item(selected),
        );
        // =================================

        // simulator window update
        window.update(&display);
        std::thread::sleep(Duration::from_millis(10));

        // take input, and quit application if necessary
        for evt in window.events() {
            match evt {
                SimulatorEvent::KeyUp { .. } => {}
                SimulatorEvent::KeyDown { keycode, .. } => {
                    if keycode == Keycode::UP {
                        selected = if selected > 0 { selected - 1 } else { 0 };
                        widget_states.force_redraw_all();
                    } else if keycode == Keycode::DOWN {
                        selected = if selected < 6 { selected + 1 } else { selected };
                        widget_states.force_redraw_all();
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
                SimulatorEvent::Quit => break 'outer,
            }
        }
    }
    Ok(())
}
