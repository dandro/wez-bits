fn pipe_text_to_pane(&self, args: Vec<String>, pane_id: &str) -> Result<ExitStatus> {
    fn display_logs_in_pane(&self, pane_id: &str) -> Result<()> {
        info!("Displaying logs in pane with id {}", pane_id);
        let error_file = format!("{}/{}", self.dot_dir, self.error_filename);
        let output_file = format!("{}/{}", self.dot_dir, self.output_filename);
        let arg = format!("tail -f -n 20 {error_file} {output_file} | bat --paging=never -l log");

        let echo_cmd = Command::new("echo")
            .arg(arg)
            .stdout(Stdio::piped())
            .spawn()
            .with_context(|| {
                TerminalError::DisplayLogs(format!(
                    "Failed to create echo command for pane {}",
                    pane_id
                ))
            })?;

        Command::new("wezterm")
            .args(["cli", "send-text", "--pane-id", pane_id, "--no-paste"])
            .stdin(Stdio::from(echo_cmd.stdout.unwrap()))
            .stdout(Stdio::inherit())
            .spawn()
            .with_context(|| {
                TerminalError::DisplayLogs(format!("Failed to send text to pane {}", pane_id))
            })?;

        Ok(())
    }
}
