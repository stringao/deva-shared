#[cfg(test)]
mod tests {
    use crate::DevaConfig;
    use tempfile::TempDir;

    #[test]
    fn test_save_load_config() {
        let config = DevaConfig::default_config();
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("deva.toml");
        config.save(path.clone()).unwrap();
        let loaded = DevaConfig::load(path).unwrap();
        assert_eq!(loaded.github.owner, "stringao");
    }
}