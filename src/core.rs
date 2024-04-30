use crate::error::Error;
use directories::ProjectDirs;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::sdl2::Keycode;
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use firefly_device::{DeviceImpl, Pad};
use firefly_runtime::*;
use std::path::PathBuf;

type Config = RuntimeConfig<SimulatorDisplay<Rgb888>, Rgb888>;

pub fn run_emulator() -> Result<(), Error> {
    let size = Size::new(WIDTH as u32, HEIGHT as u32);
    let display = SimulatorDisplay::<Rgb888>::new(size);
    let vfs_path = get_vfs_path();

    let device = DeviceImpl::new(vfs_path);
    let mut config = RuntimeConfig {
        id: None,
        device,
        display,
    };

    let output_settings = OutputSettingsBuilder::new()
        .scale(4)
        .pixel_spacing(0)
        // make FPS intentionally too high, let the runtime manage it
        .max_fps(120)
        .build();
    let mut window = Window::new("Firefly emulator", &output_settings);

    loop {
        config = match run_app(&mut window, config)? {
            Some(config) => config,
            None => break,
        };
    }
    Ok(())
}

fn run_app(window: &mut Window, mut config: Config) -> Result<Option<Config>, Error> {
    let mut input = firefly_device::InputState::default();
    // Reset input in case it is preserved from the previous runtime.
    config.device.update_input(input.clone());

    let mut runtime = Runtime::new(config)?;
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
            match event {
                SimulatorEvent::KeyDown { keycode, .. } => {
                    handle_key_down(keycode, &mut input);
                }
                SimulatorEvent::KeyUp { keycode, .. } => {
                    handle_key_up(keycode, &mut input);
                }
                // ESC is pressed. Exit the emulator.
                SimulatorEvent::Quit => {
                    return Ok(None);
                }
                _ => {}
            }
        }
        runtime.device_mut().update_input(input.clone());
    }
}

/// A key on the keyboard is released, update the input.
fn handle_key_up(keycode: Keycode, input: &mut firefly_device::InputState) {
    match keycode {
        Keycode::Z | Keycode::Return => input.buttons[0] = false,
        Keycode::X => input.buttons[1] = false,
        Keycode::A => input.buttons[2] = false,
        Keycode::S => input.buttons[3] = false,
        Keycode::Tab => input.buttons[4] = false,
        Keycode::Left => {
            if let Some(pad) = input.pad.as_mut() {
                pad.x = 0
            }
        }
        Keycode::Right => {
            if let Some(pad) = input.pad.as_mut() {
                pad.x = 0
            }
        }
        Keycode::Up => {
            if let Some(pad) = input.pad.as_mut() {
                pad.y = 0
            }
        }
        Keycode::Down => {
            if let Some(pad) = input.pad.as_mut() {
                pad.y = 0
            }
        }
        _ => {}
    }
}

/// A key on the keyboard is pressed, update the input.
fn handle_key_down(keycode: Keycode, input: &mut firefly_device::InputState) {
    match keycode {
        Keycode::Z | Keycode::Return => input.buttons[0] = true,
        Keycode::X => input.buttons[1] = true,
        Keycode::A => input.buttons[2] = true,
        Keycode::S => input.buttons[3] = true,
        Keycode::Tab => input.buttons[4] = true,
        Keycode::Left => match input.pad.as_mut() {
            Some(pad) => pad.x = -1000,
            None => input.pad = Some(Pad { x: -1000, y: 0 }),
        },
        Keycode::Right => match input.pad.as_mut() {
            Some(pad) => pad.x = 1000,
            None => input.pad = Some(Pad { x: 1000, y: 0 }),
        },
        Keycode::Up => match input.pad.as_mut() {
            Some(pad) => pad.y = 1000,
            None => input.pad = Some(Pad { x: 0, y: 1000 }),
        },
        Keycode::Down => match input.pad.as_mut() {
            Some(pad) => pad.y = -1000,
            None => input.pad = Some(Pad { x: 0, y: -1000 }),
        },
        _ => {}
    }
}

/// Get path to the virtual file system.
fn get_vfs_path() -> PathBuf {
    match ProjectDirs::from("com", "firefly", "firefly") {
        Some(dirs) => dirs.data_dir().to_owned(),
        None => PathBuf::from(".firefly"),
    }
}
