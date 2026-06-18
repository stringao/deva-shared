#[cfg(test)]
mod tests {
    use crate::init_logging;

    #[test]
    fn test_init_logging_no_panic() {
        // Should not panic
        init_logging(None);
    }

    #[test]
    fn test_init_logging_with_dir() {
        let dir = tempfile::TempDir::new().unwrap();
        let path = dir.path().to_path_buf();
        init_logging(Some(path));
    }
}