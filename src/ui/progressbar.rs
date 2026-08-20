use indicatif::ProgressBar;
use std::time::Duration;

/// Progress bar component for the CLI application.
pub fn progress_bar(message: &str) {
    let pb = ProgressBar::new_spinner();
    pb.set_message(format!("{}...", message));
    pb.enable_steady_tick(Duration::from_millis(120));
}
