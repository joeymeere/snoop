use anyhow::{Error, Result};
use std::path::Path;

pub fn check_valid_path(path: &Path) -> Result<(), Error> {
    if path.extension().and_then(|ext| ext.to_str()) != Some("so") {
        return Err(Error::msg("Invalid file path. Expected .so file"));
    } else {
        Ok(())
    }
}
