use anyhow::{anyhow, Result};
use std::{process::ExitStatus, slice};

use super::models::{Direction, DomainError, Task, TaskConfig, TaskSettings};
use crate::{domain::models::TaskClose, ports::TerminalPort};

/// The core application service for task execution
pub struct TaskExecutionService<P: TerminalPort> {
    terminal_controller: P,
}

impl<P: TerminalPort> TaskExecutionService<P> {
    pub fn new(terminal_controller: P) -> Self {
        Self {
            terminal_controller,
        }
    }

    pub fn execute_task(&self, task: Task) -> Result<ExitStatus> {
        let pane_id = self
            .terminal_controller
            .open_pane(task.settings.direction, 30)?;
        let result = self.execute_interactive_task(&pane_id, &task);

        match task.settings.close {
            TaskClose::Always | TaskClose::OnSuccess if result.is_ok() => {
                self.terminal_controller.close_pane(&pane_id)?
            }
            _ => (),
        };

        result
    }

    fn execute_interactive_task(&self, pane_id: &str, task: &Task) -> Result<ExitStatus> {
        let args = [
            slice::from_ref(&task.command.program),
            task.command.args.as_slice(),
        ]
        .concat();
        self.terminal_controller.pipe_text_to_pane(args, pane_id)
    }

    pub fn find_task(
        &self,
        task_name: &str,
        config: &TaskConfig,
        close: TaskClose,
        direction: Direction,
    ) -> Result<Task> {
        match config.get(task_name) {
            Some(command) => Ok(Task::new(
                command.to_owned(),
                TaskSettings { close, direction },
            )),
            None => Err(anyhow!(DomainError::FeatureNotConfigured(
                task_name.to_string()
            ))),
        }
    }
}
