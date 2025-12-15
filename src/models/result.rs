use std::path::PathBuf;

pub struct CleanResult {
    pub files_found: usize,
    pub files_deleted: usize,
    pub files_failed: Vec<(PathBuf, String)>,
    pub dry_run: bool,
}

impl CleanResult {
    pub fn new(files_found: usize, dry_run: bool) -> Self {
        Self {
            files_found,
            files_deleted: 0,
            files_failed: Vec::new(),
            dry_run,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_result() {
        let result = CleanResult::new(5, false);
        assert_eq!(result.files_found, 5);
        assert_eq!(result.files_deleted, 0);
        assert!(result.files_failed.is_empty());
        assert!(!result.dry_run);
    }

    #[test]
    fn test_result_dry_run() {
        let result = CleanResult::new(3, true);
        assert!(result.dry_run);
    }
}
