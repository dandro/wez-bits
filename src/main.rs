use app::run_app;
use log::{error, info};
use pretty_env_logger::init_timed;

mod app;
mod config;
mod constants;
mod domain;
mod wezterm;

fn main() {
    init_timed();
    info!("Helix Projectile");

    if let Err(err) = run_app() {
        error!("Helix Projectile Failed: {}", &err.to_string())
    }
}
