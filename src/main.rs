use clap::Parser;
use firefly_emulator::{run_emulator, CliArgs};

fn main() {
    let cli_args = CliArgs::parse();
    let res = run_emulator(&cli_args);
    if let Err(err) = res {
        let no_launcher = matches!(
            err,
            firefly_emulator::Error::Runtime(firefly_runtime::Error::NoLauncher)
        );
        if no_launcher {
            println!("Nothing is installed so there is nothing to run.");
            println!("You can start by installing the launcher and some games:");
            println!("  firefly_cli import sys.launcher");
            println!("  firefly_cli import lux.snek");
        } else {
            println!("{err}");
        }
        std::process::exit(1)
    }
}
