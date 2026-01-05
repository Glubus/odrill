//! File utilities for the run command

use anyhow::Result;
use std::fs;
use std::path::Path;

/// Recursively copy a directory and all its contents
pub fn copy_dir_all(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let dst_path = dst.join(entry.file_name());

        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dst_path)?;
        } else {
            fs::copy(entry.path(), &dst_path)?;
        }
    }

    Ok(())
}

/// Copy the dist/ directory to the specified destination
pub fn copy_dist_to(dest: &Path) -> Result<()> {
    let dist = Path::new("dist");

    if !dist.exists() {
        anyhow::bail!("dist/ not found. Run 'odrill build' first.");
    }

    if dest.exists() {
        fs::remove_dir_all(dest)?;
    }

    copy_dir_all(dist, dest)?;
    Ok(())
}
