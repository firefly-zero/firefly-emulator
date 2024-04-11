use firefly_emulator::run_emulator;

fn main() {
    let res = run_emulator();
    if let Err(err) = res {
        println!("{err}");
        std::process::exit(1)
    }
}
