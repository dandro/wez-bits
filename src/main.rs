use std::{os::unix::process::ExitStatusExt, process::ExitStatus};

use log::{error, info};

mod adapters;
mod application;
mod constants;
mod domain;
mod logger;
mod ports;

use application::Application;

use crate::{adapters::file_adapter::FileAdapter, logger::init_logger};

fn main() {
    let file_adapter = FileAdapter::new();

    if let Err(err) = init_logger(&file_adapter) {
        error!("Wez Bits: failed to initialise logger {:#}", &err);
        ExitStatus::from_raw(1);
    }

    info!("Wez Bits");

    if let Err(err) = Application::run(file_adapter) {
        error!("Wez Bits Failed: {:#}", &err);
        ExitStatus::from_raw(1);
    }
}
