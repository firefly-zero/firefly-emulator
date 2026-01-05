use crate::error::Error;
use clap::Parser;
use std::net::IpAddr;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
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
    pub(crate) fn options(&self) -> Result<minifb::WindowOptions, Error> {
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
