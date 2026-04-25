use std::time::Duration;

use embedded_graphics::geometry::Size;
use embedded_graphics::image::ImageRaw;
use embedded_graphics::pixelcolor::{BinaryColor, Rgb565};
use embedded_graphics::prelude::Point;
use embedded_graphics_simulator::sdl2::{Keycode, MouseButton};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

use matrix_gui::prelude::*;
use matrix_gui::style::*;
use matrix_gui::ui_font::DEFAULT_FONT_ASCII;
use multi_mono_font::MonoImage;

const IMAGE_RAW: ImageRaw<BinaryColor> =
    ImageRaw::<BinaryColor>::new(include_bytes!("../assets/rust_64x64.bin"), 64);
const MONO_IMAGE: MonoImage<Rgb565> = MonoImage::<Rgb565>::new(&IMAGE_RAW, rgb565!(0xFF0000));

//  enum RegionId
// const REGIONID_COUNT
// Region (RegionID, x, y, width, height)
matrix_gui::free_form_region!(
    RegionId,
    (TITLE, 100, 6, 143, 24),
    (LABEL1, 19, 34, 130, 20),
    (LABEL2, 176, 36, 130, 18),
    (BUTTON1, 20, 85, 61, 24),
    (BUTTON2, 107, 85, 79, 24),
    (BUTTON3, 197, 85, 106, 24),
    (BAR, 20, 114, 249, 14),
    (LINE_V1, 21, 131, 12, 95),
    (LINE_V2, 290, 131, 14, 91),
    (LINE_H1, 42, 134, 186, 14),
    (IMAGE, 200, 158, 83, 66),
    (SLIDER, 41, 203, 154, 25),
    (CHECKBOX_1, 50, 154, 139, 20),
    (CHECKBOX_2, 51, 179, 130, 21),
    (RADIOBUTTON1, 11, 60, 74, 20),
    (RADIOBUTTON2, 100, 61, 88, 20),
    (RADIOBUTTON3, 202, 57, 99, 20),
);

#[derive(Debug, Clone, Copy, PartialEq)]
enum RadioGroup {
    None,
    Btn1,
    Btn2,
    Btn3,
}
const RADIOBUTTON_IDS: &[RegionId] = &[RADIOBUTTON1.id(), RADIOBUTTON2.id(), RADIOBUTTON3.id()];

matrix_gui::i18n_string!(TIP_ON, "ON", "OPEN");
matrix_gui::i18n_string!(TIP_OFF, "OFF", "CLOSE");
matrix_gui::i18n_toggle_type!(TipOnOff, TIP_ON, TIP_OFF);

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<Rgb565>::new(Size::new(320, 240));
    simple_logger::init().ok();

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let mut window = Window::new("Hello World", &output_settings);
    // input handling variables
    let mut mouse_down = false;
    let mut last_down = false;
    let mut location = Point::new(0, 0);
    let smartstates = RenderState::new_array::<REGIONID_COUNT>();
    let widget_states = WidgetStates::new(&smartstates);

    let mut focus_state = FocusState::<16>::new();

    let style = rgb565_light_style();
    let mut ui = Ui::new_fullscreen(&mut display, &widget_states, &style);
    ui.clear_background().unwrap();
    let mut slider_val = 0;
    let mut checkbox1 = false;
    let mut checkbox2 = false;
    let mut radio = RadioGroup::None;

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

        if widget_states.should_redraw_multi(&[TITLE.id(), LABEL1.id()]) {
            log::info!("Static 1 redraw needed");
            ui.add(
                Label::new(TITLE, "Basic Example")
                    .with_font(DEFAULT_FONT_ASCII)
                    .with_align(HorizontalAlign::Center),
            );
            ui.add(Label::new(LABEL1, &TipOnOff(checkbox1)));
        }

        ui.lazy_draw(LABEL2.id(), |lazy_ui| {
            log::info!("Label2 redraw");
            let label2 = format!("Label2 {}", slider_val);
            lazy_ui.add(Label::new(&LABEL2, &label2).with_align(HorizontalAlign::Right))
        });

        if ui.add(Button::new(BUTTON1, "Btn1")).is_clicked() {
            log::info!("Btn1 clicked");
        }
        if ui.add(Button::new(BUTTON2, "Button2")).is_clicked() {
            log::info!("Btn2 clicked");
        }
        if ui.add(Button::new(BUTTON3, "Button3")).is_clicked() {
            log::info!("Btn3 clicked");
        }

        if widget_states.should_redraw_multi(&[LINE_V1.id(), LINE_V2.id(), LINE_H1.id()]) {
            log::info!("Lines redraw needed");
            ui.add(StaticLine::new(LINE_V1, &OriVertical));
            ui.add(StaticLine::new(LINE_V2, &OriVertical));
            ui.add(StaticLine::new(LINE_H1, &OriHorizontal));
        }

        if widget_states.should_redraw_multi(&[BAR.id()]) {
            log::info!("Bar redraw needed");
            ui.add(Bar::new(BAR, -100, 100, slider_val).with_border_color(Rgb565::BLACK));
        }

        if ui
            .add(
                Slider::new(SLIDER, &mut slider_val, -100..=100)
                    .label("Fancy Slider")
                    .step_size(5),
            )
            .is_value_changed()
        {
            log::info!("Slider value: {}", slider_val);
            widget_states.force_redraw_multi(&[BAR.id(), LABEL2.id()]);
        }

        ui.add(StaticImage::new(IMAGE, &MONO_IMAGE));

        if ui
            .add(Checkbox::new(CHECKBOX_1, "Checkbox1", &mut checkbox1))
            .is_value_changed()
        {
            log::info!("Checkbox1 changed: {}", checkbox1);
            widget_states.force_redraw(LABEL1.id());
        }
        if ui
            .add(Checkbox::new(CHECKBOX_2, "Checkbox2", &mut checkbox2))
            .is_value_changed()
        {
            log::info!("Checkbox2 changed: {}", checkbox2);
            Languages::switch_language();
            widget_states.force_redraw(LABEL1.id());
        }

        if ui
            .add(RadioButton::new(
                RADIOBUTTON1,
                "Radio1",
                RadioGroup::Btn1,
                &mut radio,
            ))
            .is_value_changed()
        {
            log::info!("Radio1 clicked {:?}", radio);
            widget_states.force_redraw_range(RADIOBUTTON1.id(), RADIOBUTTON3.id());
        }

        if ui
            .add(RadioButton::new(
                RADIOBUTTON2,
                "Radio2",
                RadioGroup::Btn2,
                &mut radio,
            ))
            .is_value_changed()
        {
            log::info!("Radio2 clicked {:?}", radio);
            widget_states.force_redraw_multi(RADIOBUTTON_IDS);
        }
        if ui
            .add(RadioButton::new(
                RADIOBUTTON3,
                "Radio3",
                RadioGroup::Btn3,
                &mut radio,
            ))
            .is_value_changed()
        {
            log::info!("Radio3 clicked {:?}", radio);
            widget_states.force_redraw_multi(RADIOBUTTON_IDS);
        }

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
                SimulatorEvent::Quit => break 'outer,
            }
        }
    }
    Ok(())
}
