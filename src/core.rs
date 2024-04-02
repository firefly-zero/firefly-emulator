use crate::error::Error;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use firefly_device::{Device, DeviceImpl};

pub(crate) fn run() -> Result<(), Error> {
    let size = Size::new(
        firefly_runtime::WIDTH as u32,
        firefly_runtime::HEIGHT as u32,
    );
    let display = SimulatorDisplay::<Rgb888>::new(size);
    let device = DeviceImpl::new("..");
    let mut runtime = firefly_runtime::Runtime::new(device, display, "demo", "go-animation")?;

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
