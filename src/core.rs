use crate::error::Error;
use directories::ProjectDirs;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use firefly_device::DeviceImpl;
use std::path::PathBuf;

pub fn run_emulator() -> Result<(), Error> {
    let size = Size::new(
        firefly_runtime::WIDTH as u32,
        firefly_runtime::HEIGHT as u32,
    );
    let display = SimulatorDisplay::<Rgb888>::new(size);
    let vfs_path = get_vfs_path();

    let meta_raw = std::fs::read(vfs_path.join("sys").join("new-app")).unwrap();
    let meta = firefly_meta::ShortMeta::decode(&meta_raw).unwrap();

    let device = DeviceImpl::new(vfs_path);
    let mut runtime = firefly_runtime::Runtime::new(device, display, meta.author_id, meta.app_id)?;

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

pub(crate) fn get_vfs_path() -> PathBuf {
    match ProjectDirs::from("com", "firefly", "firefly") {
        Some(dirs) => dirs.data_dir().to_owned(),
        None => PathBuf::from(".firefly"),
    }
}
