use crate::display::Display;
use crate::error::Error;
use directories::ProjectDirs;
use embedded_graphics::pixelcolor::Rgb888;
use firefly_device::{DeviceImpl, Pad};
use firefly_runtime::*;
use minifb::Key;
use std::path::PathBuf;

type Config = RuntimeConfig<Display, Rgb888>;

pub fn run_emulator() -> Result<(), Error> {
    let vfs_path = get_vfs_path();
    let device = DeviceImpl::new(vfs_path);
    let opts = minifb::WindowOptions {
        scale: minifb::Scale::X4,
        ..Default::default()
    };
    let mut window = minifb::Window::new("Firefly emulator", WIDTH, HEIGHT, opts)?;
    let display = Display::new();
    let mut config = RuntimeConfig {
        id: None,
        device,
        display,
    };
    loop {
        config = match run_app(&mut window, config)? {
            Some(config) => config,
            None => break,
        };
    }
    Ok(())
}

fn run_app(window: &mut minifb::Window, mut config: Config) -> Result<Option<Config>, Error> {
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
        runtime.display().update(window)?;
        for key in window.get_keys() {
            if key == Key::Escape {
                return Ok(None);
            }
            handle_key_down(key, &mut input);
        }
        for key in window.get_keys_released() {
            handle_key_up(key, &mut input);
        }
        runtime.device_mut().update_input(input.clone());
    }
}

/// A key on the keyboard is released, update the input.
fn handle_key_up(key: Key, input: &mut firefly_device::InputState) {
    match key {
        Key::Z | Key::Enter => input.buttons[0] = false,
        Key::X => input.buttons[1] = false,
        Key::A => input.buttons[2] = false,
        Key::S => input.buttons[3] = false,
        Key::Tab | Key::Backspace => input.buttons[4] = false,
        Key::Left | Key::Right | Key::Key4 | Key::Key6 => {
            if let Some(pad) = input.pad.as_mut() {
                pad.x = 0;
                if pad.y == 0 {
                    input.pad = None;
                }
            }
        }
        Key::Up | Key::Down | Key::Key8 | Key::Key2 => {
            if let Some(pad) = input.pad.as_mut() {
                pad.y = 0;
                if pad.x == 0 {
                    input.pad = None;
                }
            }
        }
        Key::Key5 => {
            if matches!(input.pad, Some(Pad { x: 0, y: 0 })) {
                input.pad = None
            }
        }
        _ => {}
    }
}

/// A key on the keyboard is pressed, update the input.
fn handle_key_down(keycode: Key, input: &mut firefly_device::InputState) {
    match keycode {
        // `Z` or `Enter`: (A)
        Key::Z | Key::Enter => input.buttons[0] = true,
        // `X`: (B)
        Key::X => input.buttons[1] = true,
        // `A`: (X)
        Key::A => input.buttons[2] = true,
        // `S`: (Y)
        Key::S => input.buttons[3] = true,
        // `Tab`, `Backspace`: (menu)
        Key::Tab | Key::Backspace => input.buttons[4] = true,
        // `←`, `4`: touchpad left
        Key::Left | Key::Key4 => match input.pad.as_mut() {
            Some(pad) => pad.x = -1000,
            None => input.pad = Some(Pad { x: -1000, y: 0 }),
        },
        // `→`, `6`: touchpad right
        Key::Right | Key::Key6 => match input.pad.as_mut() {
            Some(pad) => pad.x = 1000,
            None => input.pad = Some(Pad { x: 1000, y: 0 }),
        },
        // `↑`, `8`: touchpad up
        Key::Up | Key::Key8 => match input.pad.as_mut() {
            Some(pad) => pad.y = 1000,
            None => input.pad = Some(Pad { x: 0, y: 1000 }),
        },
        // `↓`, `2`: touchpad down
        Key::Down | Key::Key2 => match input.pad.as_mut() {
            Some(pad) => pad.y = -1000,
            None => input.pad = Some(Pad { x: 0, y: -1000 }),
        },
        // `5`: touchpad middle
        Key::Key5 => input.pad = Some(Pad { x: 0, y: 0 }),
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
