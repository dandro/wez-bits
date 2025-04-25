use app::run_app;
use log::{error, info};
use pretty_env_logger::init_timed;

mod app;
mod constants;
mod domain;
pub mod project_picker;
mod wezterm;

fn main() {
    init_timed();
    info!("Helix Projectile");

    match run_app() {
        Err(err) => error!("Helix Projectile Failed: {}", &err.to_string()),
        Ok(_) => (),
    }
}
