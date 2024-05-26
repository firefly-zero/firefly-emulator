use clap::Parser;
use firefly_emulator::{run_emulator, CliArgs};

fn main() {
    let cli_args = CliArgs::parse();
    let res = run_emulator(&cli_args);
    if let Err(err) = res {
        println!("{err}");
        std::process::exit(1)
    }
}
