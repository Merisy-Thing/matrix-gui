use std::time::Duration;

use embedded_graphics::geometry::Size;
use embedded_graphics::image::{Image, ImageRaw};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::pixelcolor::raw::LittleEndian;
use embedded_graphics::prelude::Point;
use embedded_graphics_simulator::sdl2::{Keycode, MouseButton};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

use local_static::LocalStatic;
use matrix_gui::fill_rect::fill_with_color;
use matrix_gui::framebuf::WidgetFramebuf;
use matrix_gui::prelude::*;
use matrix_gui::style::*;
use tinybmp::Bmp;

matrix_gui::free_form_region!(
    RegionId,
    (IMG1, 0, 0, 256, 256),
    (IMG2, 280, 0, 256, 256),
    (IMG3, 0, 260, 256, 256)
);

pub struct BufferedImage<'a, ID, T, COL: PixelColor> {
    region: &'a Region<ID>,
    raw_img: &'a T,
    buf: &'a mut [COL],
}

impl<'a, ID: WidgetId, T: ImageDrawable, COL: PixelColor> BufferedImage<'a, ID, T, COL> {
    pub fn new(region: &'a Region<ID>, raw_img: &'a T, buf: &'a mut [COL]) -> Self {
        Self {
            region,
            raw_img,
            buf,
        }
    }
}

impl<'a, DRAW, ID: WidgetId, T: ImageDrawable, COL: PixelColor> Widget<DRAW, COL>
    for BufferedImage<'a, ID, T, COL>
where
    COL: PixelColor,
    DRAW: DrawTarget<Color = COL>,
    T: ImageDrawable<Color = COL>,
{
    fn draw(&mut self, ui: &mut Ui<DRAW, COL>) -> GuiResult<Response> {
        let widget_state = ui.get_widget_state(self.region.id())?;
        if widget_state.compare_set(RenderStatus::Rendered) {
            return Ok(Response::Idle);
        }

        let area = self.region.rectangle();
        ui.clear_area(&area)?;

        if let Some(mut img_fb) = WidgetFramebuf::try_new(self.buf, area.size, area.top_left) {
            Image::new(self.raw_img, area.top_left).draw(&mut img_fb);
            ui.draw(&img_fb)?;

            log::info!("Filled rect: {}", area.top_left);
            Ok(Response::Idle)
        } else {
            log::error!("Failed to create framebuf");
            Err(GuiError::DrawError)
        }
    }
}

static DISPLAY: LocalStatic<SimulatorDisplay<Rgb565>> = LocalStatic::new();

struct FillRect;
matrix_gui::set_impl_fill_rect!(FillRect);

unsafe impl matrix_gui::fill_rect::ImplFillRect for FillRect {
    unsafe fn fill_with_color(x: u32, y: u32, w: u32, h: u32, color: u32) {
        let display = DISPLAY.get_mut();

        let mut buffer: Vec<u16> = Vec::<u16>::new();
        buffer.resize((w * h) as usize, color as u16);
        let buffer_ref = unsafe {
            core::slice::from_raw_parts(
                buffer.as_ptr() as *const u8,
                buffer.len() * core::mem::size_of::<u16>(),
            )
        };

        let img_raw = ImageRaw::<Rgb565, LittleEndian>::new(buffer_ref, w);
        let img = Image::new(&img_raw, Point::new(x as i32, y as i32));
        img.draw(display);
        log::info!(
            "fill_with_color: x={}, y={}, w={}, h={}, color=0x{:04X}",
            x,
            y,
            w,
            h,
            color
        );
    }

    unsafe fn fill_with_buffer(x: u32, y: u32, w: u32, h: u32, colors: *const u8) {
        let display = DISPLAY.get_mut();

        let mut buffer = Vec::<u8>::new();
        buffer.resize((w * h * 2) as usize, 0);
        let colors_ref = unsafe { core::slice::from_raw_parts(colors, buffer.len()) };
        buffer.copy_from_slice(colors_ref);

        let img_raw = ImageRaw::<Rgb565, LittleEndian>::new(&buffer, w);
        let img = Image::new(&img_raw, Point::new(x as i32, y as i32));
        img.draw(display);
        log::info!("fill_with_buffer: x={}, y={}, w={}, h={}", x, y, w, h);
    }
}

fn main() -> Result<(), core::convert::Infallible> {
    let display = SimulatorDisplay::<Rgb565>::new(Size::new(640, 480));
    DISPLAY.set(display);

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

    let display = DISPLAY.get_mut();

    let mut ui = Ui::new_fullscreen(display, &widget_states, &style);
    ui.clear_background().unwrap();

    let ferris = include_bytes!("../assets/Ferris.bmp");
    let bmp = Bmp::from_slice(ferris).unwrap();

    'outer: loop {
        let mut ui = Ui::new_fullscreen(display, &widget_states, &style);
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

        let mut framebuf = [Rgb565::BLACK; 256 * 257];
        ui.add(BufferedImage::new(IMG1, &bmp, &mut framebuf));
        ui.add(BufferedImage::new(IMG2, &bmp, &mut framebuf));

        ui.lazy_draw(IMG3.id(), |_| {
            widget_states.set_status(IMG3.id(), RenderStatus::Rendered);

            let rect = IMG1.delta_resize(DeltaResize::CenterRight(-200, -100));

            fill_with_color(&rect.rectangle(), rgb565!(0xFF0000));
            let rect = rect.delta_resize(DeltaResize::CenterLeft(50, -70));
            fill_with_color(&rect.rectangle(), rgb565!(0x00FF00));
            Response::Idle
        });

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
                        widget_states.force_redraw_all();
                    } else if keycode == Keycode::DOWN {
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
