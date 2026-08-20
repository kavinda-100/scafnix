use colored::*;

/// `Scafnix` banner and description.
pub fn print_banner() {
    let banner = r#"
     ____              __      _
    / ___|  ___ __ _ / _|_ __ (_)_  __
    \___ \ / __/ _` | |_| '_ \| \ \/ /
     ___) | (_| (_| |  _| | | | |>  <
    |____/ \___\__,_|_| |_| |_|_/_/\_\
    "#;

    println!("{}", banner.green().bold());
    println!(
        "{} {}",
        "🛡".green(),
        "CLI tool to generate your TypeScript monorepo"
            .italic()
            .white()
    );
    println!(
        "{}\n",
        "-----------------------------------------------------------".bright_black()
    );
}
