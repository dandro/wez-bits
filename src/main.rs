use log::{error, info};
use pretty_env_logger::init_timed;

mod adapters;
mod application;
mod constants;
mod domain;
mod ports;

use application::Application;

fn main() {
    init_timed();
    info!("Wez Bits");

    if let Err(err) = Application::run() {
        error!("Wez Bits Failed: {}", &err.to_string())
    }
}