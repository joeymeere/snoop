use anyhow::{Error, Result};
use std::{env, path::PathBuf};

pub fn find_project_root() -> Result<PathBuf> {
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        return Ok(PathBuf::from(manifest_dir));
    }

    let mut current_dir = env::current_dir()?;
    loop {
        if current_dir.join("Cargo.toml").exists() {
            return Ok(current_dir);
        }
        if !current_dir.pop() {
            return Err(Error::msg("Could not find project root"));
        }
    }
}
