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
        "for now CLI only support".italic().white(),
        "postgresql".italic().cyan().bold(),
        "databse. you can manualy change it to your databse of choice."
            .italic()
            .white()
    )
}
