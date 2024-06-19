use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use firefly_runtime::*;
use std::convert::Infallible;

const BUFFER_SIZE: usize = WIDTH * HEIGHT;

pub(crate) struct Display {
    buffer: [u32; BUFFER_SIZE],
}

impl Display {
    pub fn new() -> Self {
        Self {
            buffer: [0u32; BUFFER_SIZE],
        }
    }

    pub fn update(&self, window: &mut minifb::Window) -> Result<(), minifb::Error> {
        window.update_with_buffer(&self.buffer, WIDTH, HEIGHT)
    }
}

impl OriginDimensions for Display {
    fn size(&self) -> Size {
        Size::new(WIDTH as u32, HEIGHT as u32)
    }
}

impl DrawTarget for Display {
    type Color = Rgb888;
    type Error = Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = embedded_graphics::prelude::Pixel<Self::Color>>,
    {
        for Pixel(point, color) in pixels {
            if point.x >= WIDTH as i32 || point.y >= HEIGHT as i32 {
                continue;
            }
            if point.x < 0 || point.y < 0 {
                continue;
            }
            let index = (point.y as usize) * WIDTH + (point.x as usize);
            self.buffer[index] = color.into_storage();
        }
        Ok(())
    }
}
