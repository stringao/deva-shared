#[cfg(test)]
mod tests {
    use crate::error::DevaError;

    #[test]
    fn test_error_display_not_found() {
        let err = DevaError::NotFound("file.txt".to_string());
        assert_eq!(err.to_string(), "Not found: file.txt");
    }

    #[test]
    fn test_error_from_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "test");
        let err = DevaError::from(io_err);
        assert!(matches!(err, DevaError::Io(_)));
    }
}