//! Security validation for uploaded packages

use pkg::ModPackage;

/// Critical patterns that will cause package rejection
const CRITICAL_PATTERNS: [&str; 3] = ["os.execute", "io.popen", "package.loadlib"];

/// Scan a ModPackage for security issues
/// Returns Err with list of critical issues if any found
/// Returns Ok with list of warnings (non-blocking) otherwise
pub fn validate_package(pkg: &ModPackage) -> Result<Vec<String>, Vec<String>> {
    let scanner = container::security::scan::Scanner::new();
    let mut all_warnings = Vec::new();

    for (path, content) in &pkg.files {
        if path.ends_with(".lua") {
            if let Ok(code) = std::str::from_utf8(content) {
                let warnings = scanner.scan(code);
                for w in warnings {
                    all_warnings.push(format!("{}: {}", path, w));
                }
            }
        }
    }

    let critical_issues: Vec<_> = all_warnings
        .iter()
        .filter(|w| CRITICAL_PATTERNS.iter().any(|p| w.contains(p)))
        .cloned()
        .collect();

    if critical_issues.is_empty() {
        Ok(all_warnings)
    } else {
        Err(critical_issues)
    }
}
