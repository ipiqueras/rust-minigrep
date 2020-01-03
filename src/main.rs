use std::env;
use std::process;
#[macro_use] extern crate log;

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    let config = minigrep::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    info!("searching: {}", config.query);
    info!("in: {}", config.filename);
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
