use crate::display::Display;
use crate::error::Error;
use crate::*;
use directories::ProjectDirs;
use embedded_graphics::pixelcolor::Rgb888;
use firefly_hal::{DeviceConfig, DeviceImpl, InputState, Pad};
use firefly_runtime::*;
use minifb::Key;
use std::path::PathBuf;

type Config<'a> = RuntimeConfig<'a, Display, Rgb888>;

pub fn run_emulator(args: &CliArgs) -> Result<(), Error> {
    let device = {
        let vfs_path = match &args.vfs {
            Some(vfs) => vfs.clone(),
            None => get_vfs_path(),
        };
        let mut config = DeviceConfig {
            root: vfs_path,
            ..Default::default()
        };
        if let Some(ip) = args.tcp_ip {
            config.tcp_ip = ip;
        }
        if let Some(ip) = args.udp_ip {
            config.udp_ip = ip;
        }
        if let Some(peers) = &args.peers {
            config.peers.clone_from(peers);
        }
        if let Some(wav) = &args.wav {
            config.wav = Some(wav.clone());
        }
        if args.mute {
            config.mute = true;
        }
        DeviceImpl::new(config)
    };

    let opts = args.options()?;
    let mut window = minifb::Window::new("Firefly emulator", WIDTH, HEIGHT, opts)?;
    let id = match &args.id {
        Some(full_id) => Some(FullID::try_from(full_id.as_str())?),
        None => None,
    };
    let mut config = RuntimeConfig {
        id,
        device,
        display: Display::new(),
        net_handler: NetHandler::None,
    };
    config.apply_settings();
    config.save_device_info(DeviceInfo {
        model: 2,
        serial: 12_3147_4813,
        main_version: get_firmware_version(),
        io_version: (9, 99, 99),
        main_partition: 0,
        io_partition: 0,
    });
    loop {
        config = match run_app(&mut window, config, !args.no_keyboard)? {
            Some(config) => config,
            None => break,
        };
    }
    Ok(())
}

fn run_app<'a>(
    window: &mut minifb::Window,
    mut config: Config<'a>,
    keyboard: bool,
) -> Result<Option<Config<'a>>, Error> {
    let title = if let Some(id) = &config.id {
        format!("Firefly Emulator: {}.{}", id.author(), id.app())
    } else {
        "Firefly Emulator".to_string()
    };
    window.set_title(&title);

    // Reset input in case it is preserved from the previous runtime.
    config.device.update_input(InputState::default());

    let mut runtime = Runtime::new(config)?;
    runtime.start()?;
    loop {
        let exit = runtime.update()?;
        runtime.display_mut().update(window)?;
        // Exit requested. Finalize runtime and get ownership of the device back.
        if exit {
            let config = runtime.finalize()?;
            return Ok(Some(config));
        }
        if !window.is_open() {
            let config = runtime.finalize()?;
            config.finalize();
            return Ok(None);
        }
        if keyboard {
            if window.is_key_down(Key::Escape) {
                let config = runtime.finalize()?;
                config.finalize();
                return Ok(None);
            }
            let input = read_keys(window);
            runtime.device_mut().update_input(input);
        }
    }
}

fn read_keys(win: &minifb::Window) -> InputState {
    let mut s = false;
    let mut e = false;
    let mut w = false;
    let mut n = false;
    let mut menu = false;

    let mut l = false;
    let mut r = false;
    let mut u = false;
    let mut d = false;
    let mut shift = false;

    for key in win.get_keys() {
        use Key::*;
        match key {
            Z | Space => s = true,
            X | E | F | Enter => e = true,
            A | W | B | Backspace => w = true,
            S | N | Y => n = true,
            Tab | M => menu = true,
            Left | Key4 => l = true,
            Right | Key6 => r = true,
            Up | Key8 => u = true,
            Down | Key2 => d = true,
            LeftShift | RightShift => shift = true,
            _ => {}
        }
    }

    let buttons =
        u8::from(s) | u8::from(e) << 1 | u8::from(w) << 2 | u8::from(n) << 3 | u8::from(menu) << 4;
    if !l && !r && !u && !d {
        return InputState { pad: None, buttons };
    }

    let mut pad = Pad { x: 0, y: 0 };
    pad.x = match (l, r) {
        (true, true) => 0,
        (true, false) => -1000,
        (false, true) => 1000,
        (false, false) => 0,
    };
    pad.y = match (u, d) {
        (true, true) => 0,
        (true, false) => 1000,
        (false, true) => -1000,
        (false, false) => 0,
    };
    // Make sure diagonal direction is within a circle.
    // 1000/√2 ≈ 707.
    (pad.x, pad.y) = match (pad.x, pad.y) {
        (1000, 1000) => (707, 707),
        (1000, -1000) => (707, -707),
        (-1000, 1000) => (-707, 707),
        (-1000, -1000) => (-707, -707),
        (x, y) => (x, y),
    };
    if shift {
        pad.x /= 2;
        pad.y /= 2;
    }
    InputState {
        pad: Some(pad),
        buttons,
    }
}

/// Get path to the virtual file system.
fn get_vfs_path() -> PathBuf {
    let current_dir = std::env::current_dir().ok();
    if let Some(current_dir) = &current_dir {
        let path = current_dir.join(".firefly");
        if path.is_dir() {
            return path;
        }
    }
    match ProjectDirs::from("com", "firefly", "firefly") {
        Some(dirs) => dirs.data_dir().to_owned(),
        None => match current_dir {
            // Make the path absolute if possible
            Some(current_dir) => current_dir.join(".firefly"),
            None => PathBuf::from(".firefly"),
        },
    }
}

fn get_firmware_version() -> (u8, u8, u8) {
    let major: u8 = env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap();
    let minor: u8 = env!("CARGO_PKG_VERSION_MINOR").parse().unwrap();
    let patch: u8 = env!("CARGO_PKG_VERSION_PATCH").parse().unwrap();
    (major, minor, patch)
}
