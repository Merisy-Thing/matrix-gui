use core::convert::Infallible;
use core::ops::Sub;
use embedded_graphics::Pixel;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;

pub struct WidgetFramebuf<'a, C: PixelColor> {
    buf: &'a mut [C],
    size: Size,
    position: Point,
    len: usize,
}

impl<'a, C: PixelColor> WidgetFramebuf<'a, C> {
    pub fn new(buf: &'a mut [C], size: Size, position: Point) -> Self {
        let len = size.width as usize * size.height as usize;
        assert!(len <= buf.len(), "buf too small for framebuffer");
        Self {
            buf,
            size,
            position,
            len,
        }
    }

    pub fn try_new(buf: &'a mut [C], size: Size, position: Point) -> Option<Self> {
        let len = size.width as usize * size.height as usize;
        if len <= buf.len() {
            Some(Self {
                buf,
                size,
                position,
                len,
            })
        } else {
            None
        }
    }

    pub fn get_pos(&self) -> Point {
        self.position
    }

    pub fn get_size(&self) -> Size {
        self.size
    }
}

impl<C: PixelColor> Dimensions for WidgetFramebuf<'_, C> {
    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(self.position, self.size)
    }
}

impl<C: PixelColor> DrawTarget for WidgetFramebuf<'_, C> {
    type Color = C;
    type Error = Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for pixel in pixels {
            let pt = pixel.0.sub(self.position);
            let pos = pt.y * self.size.width as i32 + pt.x;
            if pos < 0 || pos >= self.len as i32 {
                continue;
            }
            self.buf[pos as usize] = pixel.1;
        }

        Ok(())
    }

    fn fill_contiguous<I>(&mut self, area: &Rectangle, colors: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Self::Color>,
    {
        let drawable_area = area.intersection(&self.bounding_box());
        if drawable_area.is_zero_sized() {
            return Ok(());
        }

        let top_skip = drawable_area.top_left.y - area.top_left.y;
        let left_skip = drawable_area.top_left.x - area.top_left.x;
        let right_skip = area.size.width as i32 - (left_skip + drawable_area.size.width as i32);

        let mut color_iter = colors.into_iter();

        for _ in 0..top_skip {
            for _ in 0..area.size.width as usize {
                color_iter.next();
            }
        }
        for y in drawable_area.top_left.y as usize
            ..drawable_area.top_left.y as usize + drawable_area.size.height as usize
        {
            for _ in 0..left_skip {
                color_iter.next();
            }
            for x in drawable_area.top_left.x as usize
                ..drawable_area.top_left.x as usize + drawable_area.size.width as usize
            {
                let pos = (y as i32 - self.position.y) as usize * self.size.width as usize
                    + (x as i32 - self.position.x) as usize;
                match color_iter.next() {
                    Some(color) => self.buf[pos] = color,
                    None => return Ok(()),
                }
            }
            for _ in 0..right_skip {
                color_iter.next();
            }
        }

        Ok(())
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
        let drawable_area = area.intersection(&self.bounding_box());

        for y in drawable_area.top_left.y as usize
            ..drawable_area.top_left.y as usize + drawable_area.size.height as usize
        {
            for x in drawable_area.top_left.x as usize
                ..drawable_area.top_left.x as usize + drawable_area.size.width as usize
            {
                let pos = (y as i32 - self.position.y) as usize * self.size.width as usize
                    + (x as i32 - self.position.x) as usize;
                self.buf[pos] = color;
            }
        }
        Ok(())
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        self.buf[0..(self.size.width * self.size.height) as usize].fill(color);
        Ok(())
    }
}

impl<C: PixelColor> Drawable for WidgetFramebuf<'_, C> {
    type Color = C;
    type Output = ();

    fn draw<D>(&self, _target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let area = Rectangle::new(self.position, self.size);
        #[cfg(not(feature = "fill-rect"))]
        {
            _target.fill_contiguous(&area, self.buf.iter().cloned())
        }

        #[cfg(feature = "fill-rect")]
        {
            crate::fill_rect::fill_with_buffer(&area, self.buf);
            Ok(())
        }
    }
}
