#[macro_use]
extern crate log;

extern crate core;
extern crate dependencies;

use dependencies::{env_logger};

fn main() {
    env_logger::init().unwrap_or_else(|err| panic!("Unable to Initate Env Logger: {}", err));

    core::start();
    warn!("Game exited Successfully");
}
