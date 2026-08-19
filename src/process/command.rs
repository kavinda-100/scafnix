use std::{
    path::Path,
    process::{Command, Stdio},
};

pub fn run_command(program: &str, args: &[&str], working_directory: &Path) -> anyhow::Result<()> {
    let status = Command::new(program)
        .args(args)
        .current_dir(working_directory)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map_err(|error| anyhow::anyhow!("Failed to run `{program}`: {error}"))?;

    if !status.success() {
        anyhow::bail!(
            "Command `{}` failed with exit code {:?}",
            program,
            status.code()
        );
    }

    Ok(())
}
