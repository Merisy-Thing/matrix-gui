use std::time::Duration;

use embedded_graphics::geometry::Size;
use embedded_graphics::image::ImageRaw;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

use matrix_gui::prelude::*;
use matrix_gui::style::*;
use matrix_gui::widgets::plaintext::PlainText;
use multi_mono_font::{CharSize, MultiMonoFont, mapping::StrGlyphMapping};

// --------------------
// 字体: 宋体, 12
// 偏置: x:0 y:0 旋转:0
// 点阵格式: 阴码-1亮0灭
// 取模方式: 逐行
// 取模走向: MSB-高位在前
// 显示: 单色 阈值：64
// --------------------
const GB2313_TIER1_16X16_FONT: MultiMonoFont = MultiMonoFont {
    image: ImageRaw::new(include_bytes!("../assets/GB2313_Tier1_16x16_11.bin"), 3600),
    glyph_mapping: &StrGlyphMapping::new(include_str!("../assets/GB2313_Tier1.txt"), 0),
    character_size: CharSize::new(16, 16),
    character_spacing: 0,
    baseline: 16,
};
const TXT_FONT: UiFont = &[&multi_mono_font::ascii::FONT_9X18, &GB2313_TIER1_16X16_FONT];

//  enum RegionId
// const REGIONID_COUNT
// Region (RegionID, x, y, width, height)
matrix_gui::free_form_region!(RegionId, (PLAIN_TEXT, 0, 0, 640, 480));

const PLAIN_STRING: &str = "Hello, World!\n\
    A paragraph is a number of lines that end with a manual newline. Paragraph spacing is the \
    number of pixels between two paragraphs.\n\
    Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when \
    an unknown printer took a galley of type and scrambled it to make a type specimen book.\n\
    你好，世界！\n\
    段落是由手动换行符结尾的多行文本。段落间距是两个段落之间的像素数量。\n\
    Lorem Ipsum 自1500年代以来一直是行业的标准虚拟文本，当时一位不知名的印刷工\n\
    拿了一个铅字盘并打乱它来制作一本字体样本书。";

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<Rgb565>::new(Size::new(640, 480));
    simple_logger::init().ok();

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let mut window = Window::new("Hello World", &output_settings);
    let smartstates = RenderState::new_array::<REGIONID_COUNT>();
    let widget_states = WidgetStates::new(&smartstates);

    let style = rgb565_light_style();
    let mut ui = Ui::new_fullscreen(&mut display, &widget_states, &style);
    ui.clear_background().unwrap();

    'outer: loop {
        let mut ui = Ui::new_fullscreen(&mut display, &widget_states, &style);
        //ui.draw_widget_bounds_debug(rgb565!(0x7F7F00));

        // =================================
        ui.add(
            PlainText::new(&PLAIN_TEXT, PLAIN_STRING)
                .with_font(TXT_FONT)
                .with_align(HorizontalAlign::Left)
                .with_paragraph_spacing(8)
                .with_padding(8, 12)
                .with_border(1),
        );
        // =================================

        // simulator window update
        window.update(&display);
        std::thread::sleep(Duration::from_millis(10));

        // take input, and quit application if necessary
        for evt in window.events() {
            match evt {
                SimulatorEvent::KeyUp { .. } => {}
                SimulatorEvent::KeyDown { .. } => {}
                SimulatorEvent::MouseButtonUp { .. } => {}
                SimulatorEvent::MouseButtonDown { .. } => {}
                SimulatorEvent::MouseWheel { .. } => {}
                SimulatorEvent::MouseMove { .. } => {}
                SimulatorEvent::Quit => break 'outer,
            }
        }
    }
    Ok(())
}
