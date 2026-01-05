//! Include directive types and parsing

use regex::Regex;

/// Represents a module include found in Lua code
#[derive(Debug, Clone, PartialEq)]
pub struct ModuleInclude {
    pub module_path: String,
    pub line: usize,
    pub full_match: String,
    pub include_type: IncludeType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IncludeType {
    Include,
    Require,
    Dofile,
}

/// Extract includes from source using provided patterns
pub fn extract_includes(
    source: &str,
    include_pattern: &Regex,
    require_pattern: &Regex,
    dofile_pattern: &Regex,
    comment_pattern: &Regex,
) -> Vec<ModuleInclude> {
    let mut includes = Vec::new();
    let source_no_comments = comment_pattern.replace_all(source, "");

    for (line_num, line) in source_no_comments.lines().enumerate() {
        extract_include_matches(line, line_num, include_pattern, &mut includes);
        extract_require_matches(line, line_num, require_pattern, &mut includes);
        extract_dofile_matches(line, line_num, dofile_pattern, &mut includes);
    }

    includes
}

fn extract_include_matches(
    line: &str,
    line_num: usize,
    pattern: &Regex,
    includes: &mut Vec<ModuleInclude>,
) {
    for cap in pattern.captures_iter(line) {
        includes.push(ModuleInclude {
            module_path: cap[1].to_string(),
            line: line_num + 1,
            full_match: cap[0].to_string(),
            include_type: IncludeType::Include,
        });
    }
}

fn extract_require_matches(
    line: &str,
    line_num: usize,
    pattern: &Regex,
    includes: &mut Vec<ModuleInclude>,
) {
    for cap in pattern.captures_iter(line) {
        let path = &cap[1];
        if path.starts_with("./") || path.starts_with("../") {
            includes.push(ModuleInclude {
                module_path: path.to_string(),
                line: line_num + 1,
                full_match: cap[0].to_string(),
                include_type: IncludeType::Require,
            });
        }
    }
}

fn extract_dofile_matches(
    line: &str,
    line_num: usize,
    pattern: &Regex,
    includes: &mut Vec<ModuleInclude>,
) {
    for cap in pattern.captures_iter(line) {
        includes.push(ModuleInclude {
            module_path: cap[1].to_string(),
            line: line_num + 1,
            full_match: cap[0].to_string(),
            include_type: IncludeType::Dofile,
        });
    }
}
