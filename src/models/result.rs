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
