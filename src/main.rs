use std::{env, fs, path::PathBuf};

const SERVER_TEMPLATE: &str = include_str!("../templates/server.ts");

fn main() -> anyhow::Result<()> {
    let project_name = env::args().nth(1).unwrap_or_else(|| "demo".to_string());

    let project_path = PathBuf::from(&project_name);

    fs::create_dir_all(&project_path)?;

    let server_path = project_path.join("server.ts");

    fs::write(&server_path, SERVER_TEMPLATE)?;

    println!("Created {}", server_path.display());

    Ok(())
}
