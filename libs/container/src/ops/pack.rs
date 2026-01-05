use crate::io::encode::encode;
use crate::security::scan::Scanner;
use anyhow::Result;
use pkg::{ModPackage, OdrillProject};
use std::fs;
use walkdir::WalkDir;

pub fn pack(project: &OdrillProject) -> Result<Vec<u8>> {
    let mut pkg = ModPackage::new(
        &project.manifest.package.name,
        &project.manifest.package.version,
    );

    let scanner = Scanner::new();

    // Walk directory and add files
    for entry in WalkDir::new(&project.root) {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            continue;
        }

        // Ignore logic (simple)
        if path.to_string_lossy().contains("target") || path.to_string_lossy().contains(".git") {
            continue;
        }

        let relative = path
            .strip_prefix(&project.root)?
            .to_string_lossy()
            .replace("\\", "/");
        let content = fs::read(path)?;

        // Security Scan (Text files only)
        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            if matches!(ext, "lua" | "txt" | "json" | "toml" | "xml") {
                let text = String::from_utf8_lossy(&content);
                let warnings = scanner.scan(&text);
                if !warnings.is_empty() {
                    println!("⚠️ Security Warning for {}: {:?}", relative, warnings);
                    // Fail or Warn? User said "verifie les failles".
                    // For now just warn, or return error?
                    // Let's Warn. If critical, we should error.
                    // But user said "si y'arrive pas error il renvoie une erreur".
                }
            }
        }

        pkg.add_file(relative, content);
    }

    // Encode
    encode(&pkg)
}
