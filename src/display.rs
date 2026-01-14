use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use firefly_runtime::{FrameBuffer, RenderFB};
use std::convert::Infallible;

#[expect(clippy::cast_possible_truncation)]
const WIDTH: u8 = firefly_runtime::WIDTH as u8;
#[expect(clippy::cast_possible_truncation)]
const HEIGHT: u8 = firefly_runtime::HEIGHT as u8;
const BUFFER_SIZE: usize = WIDTH as usize * HEIGHT as usize;

pub(crate) struct Display {
    dirty: bool,
    buffer: Box<[u32; BUFFER_SIZE]>,
}

impl Display {
    pub fn new() -> Self {
        let buffer = vec![0u32; BUFFER_SIZE].into_boxed_slice();
        Self {
            buffer: buffer.try_into().unwrap(),
            dirty: true,
        }
    }

    pub fn update(&mut self, window: &mut minifb::Window) -> Result<(), minifb::Error> {
        if self.dirty {
            self.dirty = false;
            window.update_with_buffer(&self.buffer[..], WIDTH as _, HEIGHT as _)
        } else {
            window.update();
            Ok(())
        }
    }
}

impl RenderFB for Display {
    type Error = Infallible;

    fn render_fb(&mut self, frame: &mut FrameBuffer) -> Result<(), Self::Error> {
        frame.draw(self)
    }
}

impl OriginDimensions for Display {
    fn size(&self) -> Size {
        Size::new(u32::from(WIDTH), u32::from(HEIGHT))
    }
}

impl DrawTarget for Display {
    type Color = Rgb888;
    type Error = Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = embedded_graphics::prelude::Pixel<Self::Color>>,
    {
        self.dirty = true;
        for Pixel(point, color) in pixels {
            if point.x >= i32::from(WIDTH) || point.y >= i32::from(HEIGHT) {
                continue;
            }
            if point.x < 0 || point.y < 0 {
                continue;
            }
            #[expect(clippy::cast_sign_loss)]
            let index = (point.y as usize) * WIDTH as usize + (point.x as usize);
            self.buffer[index] = color.into_storage();
        }
        Ok(())
    }
}
