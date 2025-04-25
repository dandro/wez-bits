use std::{os::unix::process::ExitStatusExt, process::ExitStatus};

use app::model::Model;
use tuirealm::{AttrValue, Attribute, PollStrategy, Update};

mod app;
mod components;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    Heading,
    ProjectPicker,
}

#[derive(Debug, PartialEq)]
pub enum Msg {
    AppClose,
    SelectProject(String),
    None,
}

pub fn init() -> anyhow::Result<ExitStatus> {
    let mut model = Model::default();

    // Enter alternate screen
    let _ = model.terminal.enter_alternate_screen();
    let _ = model.terminal.enable_raw_mode();

    while !model.quit {
        match model.app.tick(PollStrategy::Once) {
            Ok(messages) if !messages.is_empty() => {
                model.redraw = true;
                for msg in messages.into_iter() {
                    let mut msg = Some(msg);
                    while msg.is_some() {
                        msg = model.update(msg);
                    }
                }
            }
            Err(err) => {
                assert!(model
                    .app
                    .attr(
                        &Id::Heading,
                        Attribute::Text,
                        AttrValue::String(format!("[ERROR] {}", err))
                    )
                    .is_ok())
            }
            _ => {}
        }

        if model.redraw {
            model.view();
            model.redraw = false;
        }
    }

    // Terminate terminal
    let _ = model.terminal.leave_alternate_screen();
    let _ = model.terminal.disable_raw_mode();
    let _ = model.terminal.clear_screen();

    Ok(ExitStatus::from_raw(0))
}
