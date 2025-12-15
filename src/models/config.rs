use std::path::PathBuf;

pub struct CleanConfig {
    pub root_dir: PathBuf,
    pub patterns: Vec<String>,
    pub dry_run: bool,
}

impl CleanConfig {
    pub fn new(root_dir: PathBuf, additional_patterns: Vec<String>, dry_run: bool) -> Self {
        let mut all_patterns = vec![".DS_Store".to_string()];
        all_patterns.extend(additional_patterns);
        Self {
            root_dir,
            patterns: all_patterns,
            dry_run,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_config_includes_ds_store() {
        let config = CleanConfig::new(PathBuf::from("/test"), vec![], false);
        assert!(config.patterns.contains(&".DS_Store".to_string()));
    }

    #[test]
    fn test_new_config_with_additional_patterns() {
        let config = CleanConfig::new(
            PathBuf::from("/test"),
            vec!["*.bak".to_string(), "Thumbs.db".to_string()],
            false,
        );
        assert_eq!(config.patterns.len(), 3);
        assert!(config.patterns.contains(&".DS_Store".to_string()));
        assert!(config.patterns.contains(&"*.bak".to_string()));
        assert!(config.patterns.contains(&"Thumbs.db".to_string()));
    }

    #[test]
    fn test_config_dry_run_flag() {
        let config = CleanConfig::new(PathBuf::from("/test"), vec![], true);
        assert!(config.dry_run);
    }
}
