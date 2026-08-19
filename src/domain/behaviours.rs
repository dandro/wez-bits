use anyhow::{anyhow, Result};
use log::info;
use std::{collections::HashMap, process::ExitStatus, slice};

use super::models::{Direction, DomainError, Task, TaskConfig, TaskSettings};
use crate::{
    domain::models::{TaskClose, TaskExecutionError},
    ports::TerminalPort,
};

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

    pub fn apply_param_injections(
        &self,
        task: Task,
        injected_params: HashMap<String, String>,
    ) -> Result<Task, TaskExecutionError> {
        let next_args = task
            .command
            .args
            .iter()
            .map(|arg| {
                info!("Checking arg {arg} for injection");
                if arg.starts_with("%{") && arg.ends_with("}") {
                    info!("Finding param to inject");
                    self.get_param_for(arg, &injected_params)
                } else {
                    Ok(arg.clone())
                }
            })
            .collect::<Result<Vec<String>>>()
            .map_err(|e: anyhow::Error| TaskExecutionError::TaskParamInjection(e.to_string()))?;

        Ok(Task {
            command: super::models::Command {
                args: next_args,
                ..task.command
            },
            ..task
        })
    }

    fn get_param_for(
        &self,
        arg: &str,
        injected_params: &HashMap<String, String>,
    ) -> Result<String> {
        let end = arg.len() - 1;
        let key = &arg[2..(end)];
        info!("Getting injected param for key {key}");
        match injected_params.get(key).and_then(|v| {
            let cleaned_value = v.trim();
            if cleaned_value.is_empty() {
                None
            } else {
                Some(cleaned_value)
            }
        }) {
            Some(v) => {
                info!("Got injected param {v}");
                Ok(v.to_string())
            }
            None => Err(anyhow!(arg.to_owned())),
        }
    }

    pub fn execute_task(&self, task: Task) -> Result<ExitStatus> {
        let pane_id = self
            .terminal_controller
            .open_pane(task.settings.direction, 30)?;
        self.execute_interactive_task(&pane_id, task)
    }

    fn execute_interactive_task(&self, pane_id: &str, task: Task) -> Result<ExitStatus> {
        let args = [
            slice::from_ref(&task.command.program),
            task.command.args.as_slice(),
        ]
        .concat();
        self.terminal_controller
            .pipe_text_to_pane(args, pane_id, task.settings.close)
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
