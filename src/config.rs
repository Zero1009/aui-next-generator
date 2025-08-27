#[derive(Debug, Clone, PartialEq)]
pub struct ProjectConfig {
    pub name: String,
    pub install_deps: bool,
    pub use_turbo: bool,
}

impl ProjectConfig {
    pub fn new(name: String, install_deps: bool, use_turbo: bool) -> Self {
        Self {
            name,
            install_deps,
            use_turbo,
        }
    }
}

pub static DIRECTORIES: &[&str] = &[
    "src/app",
    "src/components",
    "src/constants",
    "src/hooks",
    "src/libs",
    "src/assets",
    "src/types",
    "src/fonts",
    "src/styles",
    "public",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_config_new() {
        let config = ProjectConfig::new("test-project".to_string(), true, false);
        
        assert_eq!(config.name, "test-project");
        assert_eq!(config.install_deps, true);
        assert_eq!(config.use_turbo, false);
    }

    #[test]
    fn test_project_config_clone() {
        let config1 = ProjectConfig::new("original".to_string(), true, true);
        let config2 = config1.clone();
        
        assert_eq!(config1, config2);
    }

    #[test]
    fn test_directories_contains_expected_paths() {
        assert!(DIRECTORIES.contains(&"src/app"));
        assert!(DIRECTORIES.contains(&"src/components"));
        assert!(DIRECTORIES.contains(&"public"));
        assert_eq!(DIRECTORIES.len(), 10);
    }

    #[test]
    fn test_all_directories_start_with_src_or_public() {
        for dir in DIRECTORIES {
            assert!(
                dir.starts_with("src/") || dir == &"public",
                "Directory '{}' should start with 'src/' or be 'public'",
                dir
            );
        }
    }
}