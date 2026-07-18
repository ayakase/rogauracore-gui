use std::process::Command;

use super::command::AuraCommand;

pub struct ExecutionResult {
    pub command: String,
    pub success: bool,
    pub detail: String,
}

impl ExecutionResult {
    pub fn summary(&self) -> String {
        if self.success {
            "Applied.".into()
        } else if self.detail.is_empty() {
            "Failed.".into()
        } else {
            format!("Failed: {}", self.detail)
        }
    }
}

pub fn run(command: AuraCommand) -> ExecutionResult {
    let args = command.args();
    let display = command.display();

    match Command::new("rogauracore").args(&args).output() {
        Ok(output) if output.status.success() => ExecutionResult {
            command: display,
            success: true,
            detail: String::from_utf8_lossy(&output.stdout).trim().to_owned(),
        },
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();
            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_owned();
            let detail = if !stderr.is_empty() { stderr } else { stdout };

            ExecutionResult {
                command: display,
                success: false,
                detail,
            }
        }
        Err(error) => ExecutionResult {
            command: display,
            success: false,
            detail: error.to_string(),
        },
    }
}
