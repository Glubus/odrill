//! Dead code detection rule

use crate::diagnostics::{Diagnostic, DiagnosticCode};
use std::path::Path;

/// Check for dead code after return statements (line-based scan)
pub fn check_dead_code(path: &Path, source: &str) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let lines: Vec<&str> = source.lines().collect();

    let mut function_depth = 0;
    let mut found_return_at_depth: Option<(usize, usize)> = None;

    for (line_idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        let line_num = line_idx + 1;

        // Track function scope
        if trimmed.starts_with("function ")
            || trimmed.contains(" function(")
            || trimmed.starts_with("local function ")
        {
            function_depth += 1;
        }

        if trimmed == "end" || trimmed.starts_with("end,") || trimmed.starts_with("end)") {
            if function_depth > 0 {
                function_depth -= 1;
                if let Some((ret_depth, _)) = found_return_at_depth {
                    if ret_depth >= function_depth {
                        found_return_at_depth = None;
                    }
                }
            }
            if function_depth == 0 {
                found_return_at_depth = None;
            }
        }

        // Reset on branch change
        if trimmed.starts_with("elseif") || trimmed.starts_with("else") {
            found_return_at_depth = None;
        }

        // Check for code after return
        if let Some((ret_depth, ret_line)) = found_return_at_depth {
            if ret_depth == function_depth
                && !trimmed.is_empty()
                && !trimmed.starts_with("--")
                && trimmed != "end"
                && !trimmed.starts_with("end")
                && !trimmed.starts_with("else")
                && !trimmed.starts_with("elseif")
            {
                diagnostics.push(Diagnostic::warning(
                    DiagnosticCode::DeadCode,
                    format!("Unreachable code after return on line {}", ret_line),
                    path.to_path_buf(),
                    line_num,
                    1,
                ));
                found_return_at_depth = None;
            }
        }

        // Detect return statement
        if function_depth > 0 && (trimmed.starts_with("return ") || trimmed == "return") {
            found_return_at_depth = Some((function_depth, line_num));
        }
    }

    diagnostics
}
