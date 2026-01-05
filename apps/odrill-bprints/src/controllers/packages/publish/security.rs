//! Security validation for packages

use loco_rs::prelude::*;
use pkg::ModPackage;

const CRITICAL_PATTERNS: [&str; 3] = ["os.execute", "io.popen", "package.loadlib"];

pub fn validate(pkg: &ModPackage) -> Result<()> {
    let scanner = container::security::scan::Scanner::new();
    let mut all_warnings = Vec::new();

    for (path, content) in &pkg.files {
        if path.ends_with(".lua") {
            if let Ok(code) = std::str::from_utf8(content) {
                for w in scanner.scan(code) {
                    all_warnings.push(format!("{}: {}", path, w));
                }
            }
        }
    }

    let critical: Vec<_> = all_warnings
        .iter()
        .filter(|w| CRITICAL_PATTERNS.iter().any(|p| w.contains(p)))
        .collect();

    if !critical.is_empty() {
        tracing::error!("Package rejected: {:?}", critical);
        return Err(Error::BadRequest(format!("Rejected: {:?}", critical)));
    }

    if !all_warnings.is_empty() {
        tracing::warn!("Package '{}' has warnings: {:?}", pkg.name, all_warnings);
    }

    Ok(())
}
