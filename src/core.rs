use crate::display::Display;
use crate::error::Error;
use clap::Parser;
use directories::ProjectDirs;
use embedded_graphics::pixelcolor::Rgb888;
use firefly_hal::{DeviceConfig, DeviceImpl, InputState, Pad};
use firefly_runtime::*;
use minifb::Key;
use std::net::IpAddr;
use std::path::PathBuf;

type Config<'a> = RuntimeConfig<'a, Display, Rgb888>;

#[derive(Debug, Parser)]
pub struct CliArgs {
    /// The scale for the window and each pixel.
    ///
    /// Must be a power of 2: 1, 2, 4, 8, 16, or 32.
    #[arg(long, default_value_t = 4)]
    pub scale: u8,

    /// Run the emulator in borderless mode and scale to fit the screen.
    ///
    /// If specified, "--scale" has no effect.
    #[arg(long, default_value_t = false)]
    pub fullscreen: bool,

    /// The full ID of the app to run.
    ///
    /// If not specified, will start launcher (if installed) or the latest installed app.
    #[arg(long, default_value = None)]
    pub id: Option<String>,

    /// The TCP IP address where to listen for serial events.
    #[arg(long, default_value = None)]
    pub tcp_ip: Option<IpAddr>,

    /// The UDP IP address where to listen for netplay events.
    #[arg(long, default_value = None)]
    pub udp_ip: Option<IpAddr>,

    /// The UDP IP addresses where to send netplay advertisements.
    #[arg(long, default_value = None)]
    pub peers: Option<Vec<IpAddr>>,

    /// Path to the virtual FS to use.
    ///
    /// By default, the global one (~/.local/share/firefly) is used.
    #[arg(long, default_value = None)]
    pub vfs: Option<PathBuf>,

    /// If provided, the path where to save the audio output (as a WAV file).
    #[arg(long, default_value = None)]
    pub wav: Option<PathBuf>,

    /// Disable reading input from keyboard.
    ///
    /// Useful if you have troubles with keyboard (like a stuck key)
    /// and just want to use gamepad as input.
    #[arg(long, default_value_t = false)]
    pub no_keyboard: bool,
}

impl CliArgs {
    fn options(&self) -> Result<minifb::WindowOptions, Error> {
        let scale = if self.scale == 4 && self.fullscreen {
            minifb::Scale::FitScreen
        } else {
            match self.scale {
                1 => minifb::Scale::X1,
                2 => minifb::Scale::X2,
                4 => minifb::Scale::X4,
                8 => minifb::Scale::X8,
                16 => minifb::Scale::X16,
                32 => minifb::Scale::X32,
                _ => return Err("invalid scale".into()),
            }
        };
        let opts = minifb::WindowOptions {
            borderless: self.fullscreen,
            scale,
            scale_mode: minifb::ScaleMode::Stretch,
            resize: true,
            ..Default::default()
        };
        Ok(opts)
    }
}

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
            config.peers = peers.clone();
        }
        if let Some(wav) = &args.wav {
            config.wav = Some(wav.clone());
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
        // Exit requested. Finalize runtime and get ownership of the device back.
        if exit {
            let config = runtime.finalize()?;
            return Ok(Some(config));
        }
        runtime.display().update(window)?;
        if !window.is_open() {
            return Ok(None);
        }
        if keyboard {
            if window.is_key_down(Key::Escape) {
                return Ok(None);
            }
            let input = read_keys(window);
            runtime.device_mut().update_input(input);
        }
    }
}

fn read_keys(win: &minifb::Window) -> InputState {
    let s = win.is_key_down(Key::Z) | win.is_key_down(Key::Enter) | win.is_key_down(Key::Space);
    let e = win.is_key_down(Key::X) | win.is_key_down(Key::B) | win.is_key_down(Key::Backspace);
    let w = win.is_key_down(Key::A);
    let n = win.is_key_down(Key::S) | win.is_key_down(Key::Y);
    let menu = win.is_key_down(Key::Tab);
    let buttons =
        u8::from(s) | u8::from(e) << 1 | u8::from(w) << 2 | u8::from(n) << 3 | u8::from(menu) << 4;

    let l = win.is_key_down(Key::Left) | win.is_key_down(Key::Key4);
    let r = win.is_key_down(Key::Right) | win.is_key_down(Key::Key6);
    let u = win.is_key_down(Key::Up) | win.is_key_down(Key::Key8);
    let d = win.is_key_down(Key::Down) | win.is_key_down(Key::Key2);

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
