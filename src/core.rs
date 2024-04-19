use crate::error::Error;
use directories::ProjectDirs;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use firefly_device::DeviceImpl;
use firefly_runtime::*;
use std::path::PathBuf;

type Config = RuntimeConfig<SimulatorDisplay<Rgb888>, Rgb888>;

pub fn run_emulator() -> Result<(), Error> {
    let size = Size::new(WIDTH as u32, HEIGHT as u32);
    let display = SimulatorDisplay::<Rgb888>::new(size);
    let vfs_path = get_vfs_path();

    let meta_raw = std::fs::read(vfs_path.join("sys").join("new-app")).unwrap();
    let meta = firefly_meta::ShortMeta::decode(&meta_raw).unwrap();

    let device = DeviceImpl::new(vfs_path);
    let id = FullID::new(
        meta.author_id.try_into().unwrap(),
        meta.app_id.try_into().unwrap(),
    );
    let mut config = RuntimeConfig {
        id: Some(id),
        device,
        display,
    };
    loop {
        config = match run_app(config)? {
            Some(config) => config,
            None => break,
        };
    }
    Ok(())
}

fn run_app(config: Config) -> Result<Option<Config>, Error> {
    let mut runtime = Runtime::new(config)?;

    let output_settings = OutputSettingsBuilder::new()
        .scale(4)
        .pixel_spacing(0)
        // make FPS intentionally too high, let the runtime manage it
        .max_fps(120)
        .build();
    let mut window = Window::new("Firefly emulator", &output_settings);
    runtime.start()?;
    loop {
        let exit = runtime.update()?;
        // Exit requested. Finalize runtime and get ownership of the device back.
        if exit {
            let config = runtime.into_config();
            return Ok(Some(config));
        }
        window.update(runtime.display());
        for event in window.events() {
            // ESC is pressed. Exit the emulator.
            if event == SimulatorEvent::Quit {
                return Ok(None);
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
