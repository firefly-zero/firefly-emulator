use crate::error::Error;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

pub(crate) fn run() -> Result<(), Error> {
    let size = Size::new(
        firefly_runtime::WIDTH as u32,
        firefly_runtime::HEIGHT as u32,
    );
    let display = SimulatorDisplay::<Rgb888>::new(size);
    let device = firefly_runtime::Device {
        display,
        timer: firefly_hosted::Timer::new(),
        input: firefly_hosted::Input::new(),
        storage: firefly_hosted::Storage::new(".."),
        reader: std::marker::PhantomData,
    };
    let mut runtime = firefly_runtime::Runtime::new(device, "go-animation")?;

    let output_settings = OutputSettingsBuilder::new()
        .scale(4)
        .pixel_spacing(0)
        // make FPS intentionally too high, let the runtime manage it
        .max_fps(120)
        .build();
    let mut window = Window::new("Firefly emulator", &output_settings);
    runtime.start()?;
    loop {
        runtime.update()?;
        window.update(runtime.display());
        for event in window.events() {
            if event == SimulatorEvent::Quit {
                return Ok(());
            }
        }
    }
}
