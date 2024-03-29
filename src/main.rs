use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, PrimitiveStyle};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

const WIDTH: usize = 320;
const HEIGHT: usize = 240;

fn main() -> Result<(), core::convert::Infallible> {
    let size = Size::new(WIDTH as u32, HEIGHT as u32);
    let mut display = SimulatorDisplay::<Rgb888>::new(size);
    let line_style = PrimitiveStyle::with_stroke(Rgb888::WHITE, 1);
    Circle::new(Point::new(72, 8), 48)
        .into_styled(line_style)
        .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new()
        .scale(4)
        .pixel_spacing(1)
        // make FPS intentionally too high, let the runtime manage it
        .max_fps(120)
        .build();
    let mut window = Window::new("Firefly emulator", &output_settings);
    window.show_static(&display);

    Ok(())
}
