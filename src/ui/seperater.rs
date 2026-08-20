use colored::*;

/// Prints a separator line in the terminal.
pub fn print_separator(large: Option<bool>) {
    println!(
        "{}",
        "-----------------------------------------------------------".bright_black()
    );

    if let Some(true) = large {
        println!(" ");
    }
}
