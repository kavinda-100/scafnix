use colored::*;

/// Notes for the user before project creation.
pub fn print_notes() {
    println!(
        "{} {} {}",
        "Make sure you have".italic().white(),
        "Node.js, npm, pnpm or bun".italic().cyan().bold(),
        "installed before proceeding.".italic().white()
    );
    println!(
        "{} {} {}",
        "For now, CLI only supports".italic().white(),
        "postgresql".italic().cyan().bold(),
        "database. You can manually change it to your database of choice."
            .italic()
            .white()
    )
}
