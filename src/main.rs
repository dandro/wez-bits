use std::{os::unix::process::ExitStatusExt, process::ExitStatus};

use log::{error, info};

mod adapters;
mod application;
mod constants;
mod domain;
mod ports;

use application::Application;

fn init_logger() {
    let log_file = std::env::var("WZB_LOG_FILE")
        .unwrap_or_else(|_| "/tmp/wez-bits.log".to_string());

    let formatter = |out: fern::FormatCallback, message: &std::fmt::Arguments, record: &log::Record| {
        out.finish(format_args!(
            "{} {} {} - {}",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
            record.level(),
            record.target(),
            message
        ))
    };

    fern::Dispatch::new()
        .chain(
            fern::Dispatch::new()
                .format(formatter)
                .level(log::LevelFilter::Error)
                .chain(std::io::stderr()),
        )
        .chain(
            fern::Dispatch::new()
                .format(formatter)
                .level(log::LevelFilter::Trace)
                .chain(fern::log_file(log_file).expect("Failed to open log file")),
        )
        .apply()
        .expect("Failed to initialize logger");
}

fn main() {
    init_logger();
    info!("Wez Bits");

    if let Err(err) = Application::run() {
        error!("Wez Bits Failed: {:#}", &err);
        ExitStatus::from_raw(1);
    }
}
