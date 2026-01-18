use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use firefly_runtime::{FrameBuffer, RenderFB};
use std::convert::Infallible;

pub const SCREEN_W: usize = firefly_runtime::WIDTH;
pub const SCREEN_H: usize = firefly_runtime::HEIGHT;
pub const SCREEN_M: usize = 20;
pub const PANEL_W: usize = 155;
pub const DEVICE_W: usize = PANEL_W * 2 + SCREEN_W;
pub const DEVICE_H: usize = SCREEN_M * 2 + SCREEN_H;

#[expect(clippy::cast_possible_truncation)]
const WIDTH: u8 = firefly_runtime::WIDTH as u8;
#[expect(clippy::cast_possible_truncation)]
const HEIGHT: u8 = firefly_runtime::HEIGHT as u8;

pub(crate) struct Display {
    buffer: Box<[u32]>,
    dirty: bool,
    device: bool,
    pub width: usize,
    pub height: usize,
}

impl Display {
    pub fn new(device: bool) -> Self {
        let (width, height) = if device {
            (DEVICE_W, DEVICE_H)
        } else {
            (SCREEN_W, SCREEN_H)
        };
        let buf_size = width * height;
        let buffer = vec![0u32; buf_size].into_boxed_slice();
        Self {
            buffer,
            device,
            dirty: true,
            width,
            height,
        }
    }

    pub fn update(&mut self, window: &mut minifb::Window) -> Result<(), minifb::Error> {
        if self.dirty {
            self.dirty = false;
            window.update_with_buffer(&self.buffer[..], self.width, self.height)
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

    #[expect(clippy::cast_sign_loss)]
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
            let index = if self.device {
                let line_offset = (point.y as usize + SCREEN_M) * DEVICE_W;
                let col_offset = point.x as usize + PANEL_W;
                line_offset + col_offset
            } else {
                (point.y as usize) * SCREEN_W + (point.x as usize)
            };
            self.buffer[index] = color.into_storage();
        }
        Ok(())
    }
}
