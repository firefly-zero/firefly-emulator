mod core;
mod error;

fn main() {
    let res = crate::core::run();
    if let Err(err) = res {
        println!("{err}");
        std::process::exit(1)
    }
}
