use firefly_emulator::run_emulator;

fn main() {
    let res = run_emulator("demo", "go-touchpad");
    if let Err(err) = res {
        println!("{err}");
        std::process::exit(1)
    }
}
