use indicatif::ProgressBar;
use std::time::Duration;

/// Progress bar component for the CLI application.
#[allow(dead_code)]
pub fn progress_bar(message: &str) -> ProgressBar {
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_message(format!("{message}..."));
    progress_bar.enable_steady_tick(Duration::from_millis(120));

    progress_bar
}
