use regex::Regex;

pub struct Scanner {
    suspicious_patterns: Vec<(Regex, &'static str)>,
}

impl Scanner {
    pub fn new() -> Self {
        Self {
            suspicious_patterns: vec![
                (Regex::new(r"os\.execute").unwrap(), "os.execute call"),
                (Regex::new(r"io\.popen").unwrap(), "io.popen call"),
                (
                    Regex::new(r"package\.loadlib").unwrap(),
                    "package.loadlib call",
                ),
                (Regex::new(r"http[s]?://").unwrap(), "HTTP(S) URL usage"),
            ],
        }
    }

    pub fn scan(&self, content: &str) -> Vec<String> {
        let mut warnings = Vec::new();
        for (pattern, msg) in &self.suspicious_patterns {
            if pattern.is_match(content) {
                warnings.push(msg.to_string());
            }
        }
        warnings
    }
}
