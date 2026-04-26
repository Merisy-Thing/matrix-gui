#![allow(clippy::missing_safety_doc)]

use embedded_graphics::{prelude::PixelColor, primitives::Rectangle};

pub fn fill_with_color<COL: PixelColor>(rect: &Rectangle, color: COL) {
    unsafe extern "Rust" {
        fn _ll_fill_with_color(x: u32, y: u32, w: u32, h: u32, color: u32);
    }

    let color_as_u32 = color_to_u32(&color);

    let top_left = rect.top_left;
    let size = rect.size;
    unsafe {
        _ll_fill_with_color(
            top_left.x as u32,
            top_left.y as u32,
            size.width,
            size.height,
            color_as_u32,
        )
    };
}
pub fn fill_with_buffer<C: PixelColor>(rect: &Rectangle, colors: &[C]) {
    unsafe extern "Rust" {
        fn _ll_fill_with_buffer(x: u32, y: u32, w: u32, h: u32, colors: *const u8);
    }

    let top_left = rect.top_left;
    let size = rect.size;

    unsafe {
        _ll_fill_with_buffer(
            top_left.x as u32,
            top_left.y as u32,
            size.width,
            size.height,
            colors.as_ptr() as *const u8,
        )
    };
}

fn color_to_u32<COL: PixelColor>(color: &COL) -> u32 {
    let color_ptr = color as *const COL as *const u8;
    let color_size = core::mem::size_of::<COL>();

    match color_size {
        1 => unsafe { *color_ptr as u32 },
        2 => unsafe { *color_ptr.cast::<u16>() as u32 },
        3 => {
            let bytes = unsafe { core::ptr::read_unaligned(color_ptr.cast::<[u8; 3]>()) };
            u32::from_be_bytes([0, bytes[0], bytes[1], bytes[2]])
        }
        4 => unsafe { *color_ptr.cast::<u32>() },
        _ => 0xFFFFFFFF,
    }
}

pub unsafe trait ImplFillRect {
    unsafe fn fill_with_color(x: u32, y: u32, w: u32, h: u32, color: u32);
    unsafe fn fill_with_buffer(x: u32, y: u32, w: u32, h: u32, colors: *const u8);
}

#[macro_export]
macro_rules! set_impl_fill_rect {
    ($t: ty) => {
        #[unsafe(no_mangle)]
        unsafe fn _ll_fill_with_color(x: u32, y: u32, w: u32, h: u32, color: u32) {
            unsafe { <$t as $crate::fill_rect::ImplFillRect>::fill_with_color(x, y, w, h, color) }
        }
        #[unsafe(no_mangle)]
        unsafe fn _ll_fill_with_buffer(x: u32, y: u32, w: u32, h: u32, colors: *const u8) {
            unsafe { <$t as $crate::fill_rect::ImplFillRect>::fill_with_buffer(x, y, w, h, colors) }
        }
    };
}
