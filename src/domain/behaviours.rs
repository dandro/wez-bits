use std::process::ExitStatus;
use anyhow::{anyhow, Result};

use super::models::{AppError, Direction, Task, TaskConfig, TaskSettings};
use crate::ports::{TaskExecutionPort, TerminalPort};

/// The core application service for task execution
pub struct TaskExecutionService<T: TaskExecutionPort, P: TerminalPort> {
    task_executor: T,
    terminal_controller: P,
}

impl<T: TaskExecutionPort, P: TerminalPort> TaskExecutionService<T, P> {
    pub fn new(task_executor: T, terminal_controller: P) -> Self {
        Self {
            task_executor,
            terminal_controller,
        }
    }

    /// Execute a task based on its settings
    pub fn execute_task(&self, task: Task) -> Result<ExitStatus> {
        if task.settings.interactive {
            self.execute_interactive_task(task)
        } else {
            self.execute_non_interactive_task(task)
        }
    }

    /// Execute a task in interactive mode
    fn execute_interactive_task(&self, task: Task) -> Result<ExitStatus> {
        let pane_id = self.terminal_controller.open_pane(Direction::Right, 30)?;

        let args = [&[task.command.program], task.command.args.as_slice()].concat();

        self.terminal_controller.pipe_text_to_pane(args, pane_id)
    }

    /// Execute a task in non-interactive mode
    fn execute_non_interactive_task(&self, task: Task) -> Result<ExitStatus> {
        let pane_id = self.terminal_controller.open_pane(Direction::Down, 30)?;
        self.terminal_controller.display_logs_in_pane(&pane_id)?;

        let result = self.task_executor.execute_command(task.command)?;

        if result.success() {
            // Sleep and close pane - this would be handled by the controller in real impl
            let _ = self.terminal_controller.close_pane(&pane_id);
        }

        Ok(result)
    }

    /// Find a task by name in the config
    pub fn find_task(
        &self,
        task_name: &str,
        config: &TaskConfig,
        interactive: bool,
    ) -> Result<Task> {
        match config.get(task_name) {
            Some(command) => Ok(Task::new(command.clone(), TaskSettings { interactive })),
            None => Err(anyhow!(AppError::FeatureNotConfigured(task_name.to_string()))),
        }
    }
}

// Using the ports defined in the ports module
