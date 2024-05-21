use crate::display::Display;
use crate::error::Error;
use directories::ProjectDirs;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use firefly_device::{DeviceImpl, Pad};
use firefly_runtime::*;
use std::path::PathBuf;

type Config = RuntimeConfig<Display, Rgb888>;

pub fn run_emulator() -> Result<(), Error> {
    let vfs_path = get_vfs_path();
    let device = DeviceImpl::new(vfs_path);
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let opts = minifb::WindowOptions {
        scale: minifb::Scale::X4,
        ..Default::default()
    };
    let mut window = minifb::Window::new("Firefly emulator", WIDTH, HEIGHT, opts)?;
    let mut display = Display::new();
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
        runtime.display().update(window);
        // for event in window.events() {
        //     match event {
        //         SimulatorEvent::KeyDown { keycode, .. } => {
        //             handle_key_down(keycode, &mut input);
        //         }
        //         SimulatorEvent::KeyUp { keycode, .. } => {
        //             handle_key_up(keycode, &mut input);
        //         }
        //         // ESC is pressed. Exit the emulator.
        //         SimulatorEvent::Quit => {
        //             return Ok(None);
        //         }
        //         _ => {}
        //     }
        // }
        runtime.device_mut().update_input(input.clone());
    }
}

// /// A key on the keyboard is released, update the input.
// fn handle_key_up(keycode: Keycode, input: &mut firefly_device::InputState) {
//     match keycode {
//         Keycode::Z | Keycode::Return => input.buttons[0] = false,
//         Keycode::X => input.buttons[1] = false,
//         Keycode::A => input.buttons[2] = false,
//         Keycode::S => input.buttons[3] = false,
//         Keycode::Tab | Keycode::Backspace => input.buttons[4] = false,
//         Keycode::Left | Keycode::Right | Keycode::Kp4 | Keycode::Kp6 => {
//             if let Some(pad) = input.pad.as_mut() {
//                 pad.x = 0;
//                 if pad.y == 0 {
//                     input.pad = None;
//                 }
//             }
//         }
//         Keycode::Up | Keycode::Down | Keycode::Kp8 | Keycode::Kp2 => {
//             if let Some(pad) = input.pad.as_mut() {
//                 pad.y = 0;
//                 if pad.x == 0 {
//                     input.pad = None;
//                 }
//             }
//         }
//         Keycode::Kp5 => {
//             if matches!(input.pad, Some(Pad { x: 0, y: 0 })) {
//                 input.pad = None
//             }
//         }
//         _ => {}
//     }
// }

// /// A key on the keyboard is pressed, update the input.
// fn handle_key_down(keycode: Keycode, input: &mut firefly_device::InputState) {
//     match keycode {
//         // `Z` or `Enter`: (A)
//         Keycode::Z | Keycode::Return => input.buttons[0] = true,
//         // `X`: (B)
//         Keycode::X => input.buttons[1] = true,
//         // `A`: (X)
//         Keycode::A => input.buttons[2] = true,
//         // `S`: (Y)
//         Keycode::S => input.buttons[3] = true,
//         // `Tab`, `Backspace`: (menu)
//         Keycode::Tab | Keycode::Backspace => input.buttons[4] = true,
//         // `←`, `4`: touchpad left
//         Keycode::Left | Keycode::Kp4 => match input.pad.as_mut() {
//             Some(pad) => pad.x = -1000,
//             None => input.pad = Some(Pad { x: -1000, y: 0 }),
//         },
//         // `→`, `6`: touchpad right
//         Keycode::Right | Keycode::Kp6 => match input.pad.as_mut() {
//             Some(pad) => pad.x = 1000,
//             None => input.pad = Some(Pad { x: 1000, y: 0 }),
//         },
//         // `↑`, `8`: touchpad up
//         Keycode::Up | Keycode::Kp8 => match input.pad.as_mut() {
//             Some(pad) => pad.y = 1000,
//             None => input.pad = Some(Pad { x: 0, y: 1000 }),
//         },
//         // `↓`, `2`: touchpad down
//         Keycode::Down | Keycode::Kp2 => match input.pad.as_mut() {
//             Some(pad) => pad.y = -1000,
//             None => input.pad = Some(Pad { x: 0, y: -1000 }),
//         },
//         // `5`: touchpad middle
//         Keycode::Kp5 => input.pad = Some(Pad { x: 0, y: 0 }),
//         _ => {}
//     }
// }

/// Get path to the virtual file system.
fn get_vfs_path() -> PathBuf {
    match ProjectDirs::from("com", "firefly", "firefly") {
        Some(dirs) => dirs.data_dir().to_owned(),
        None => PathBuf::from(".firefly"),
    }
}
