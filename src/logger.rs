use anyhow::Result;

use crate::{constants::DOTDIR, ports::FileSystemPort};

pub fn init_logger<FS: FileSystemPort>(fs_adapter: &FS) -> Result<()> {
    let log_file = std::env::var("WZB_LOG_FILE")
        .unwrap_or_else(|_| format!("{}/wez-bits.log", DOTDIR).to_string());

    fs_adapter.create_directory(DOTDIR)?;
    let formatter =
        |out: fern::FormatCallback, message: &std::fmt::Arguments, record: &log::Record| {
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
        .apply()?;

    Ok(())
}
