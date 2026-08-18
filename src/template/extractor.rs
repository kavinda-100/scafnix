use std::{fs, path::Path};

use include_dir::Dir;

pub fn extract_dir(dir: &Dir<'_>, destination: &Path) -> anyhow::Result<()> {
    // Create the destination directory if it doesn't exist
    fs::create_dir_all(destination)?;

    // Iterate over the files in the directory and write them to the destination
    for file in dir.files() {
        // Create the output path by joining the destination with the file's path
        let output_path = destination.join(file.path());

        // Create the parent directory if it doesn't exist
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Write the file contents to the output path
        fs::write(output_path, file.contents())?;
    }

    // Recursively extract subdirectories
    for child_dir in dir.dirs() {
        extract_dir(child_dir, destination)?;
    }

    Ok(())
}
