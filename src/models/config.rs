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
