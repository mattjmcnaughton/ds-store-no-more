use anyhow::Result;
use glob::Pattern;

pub struct PatternMatcher {
    patterns: Vec<Pattern>,
}

impl PatternMatcher {
    pub fn new(patterns: &[String]) -> Result<Self> {
        let compiled: Result<Vec<_>, _> = patterns.iter().map(|p| Pattern::new(p)).collect();
        Ok(Self {
            patterns: compiled?,
        })
    }

    pub fn matches(&self, filename: &str) -> bool {
        self.patterns.iter().any(|p| p.matches(filename))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_match() {
        let matcher = PatternMatcher::new(&[".DS_Store".to_string()]).unwrap();
        assert!(matcher.matches(".DS_Store"));
        assert!(!matcher.matches("other.txt"));
    }

    #[test]
    fn test_glob_pattern() {
        let matcher = PatternMatcher::new(&["*.bak".to_string()]).unwrap();
        assert!(matcher.matches("file.bak"));
        assert!(matcher.matches("another.bak"));
        assert!(!matcher.matches("file.txt"));
    }

    #[test]
    fn test_multiple_patterns() {
        let matcher =
            PatternMatcher::new(&[".DS_Store".to_string(), "Thumbs.db".to_string()]).unwrap();
        assert!(matcher.matches(".DS_Store"));
        assert!(matcher.matches("Thumbs.db"));
        assert!(!matcher.matches("other.txt"));
    }

    #[test]
    fn test_no_match() {
        let matcher = PatternMatcher::new(&[".DS_Store".to_string()]).unwrap();
        assert!(!matcher.matches("readme.md"));
        assert!(!matcher.matches("DS_Store")); // Missing dot
    }
}
