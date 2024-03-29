use embedded_graphics::mono_font::ascii::FONT_6X9;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Line, PrimitiveStyle, Rectangle};
use embedded_graphics::text::Text;
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(128, 64));

    let line_style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
    let text_style = MonoTextStyle::new(&FONT_6X9, BinaryColor::On);

    Circle::new(Point::new(72, 8), 48)
        .into_styled(line_style)
        .draw(&mut display)?;

    Line::new(Point::new(48, 16), Point::new(8, 16))
        .into_styled(line_style)
        .draw(&mut display)?;

    Line::new(Point::new(48, 16), Point::new(64, 32))
        .into_styled(line_style)
        .draw(&mut display)?;

    Rectangle::new(Point::new(79, 15), Size::new(34, 34))
        .into_styled(line_style)
        .draw(&mut display)?;

    Text::new("Hello World!", Point::new(5, 5), text_style).draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    Window::new("Hello World", &output_settings).show_static(&display);

    Ok(())
}
