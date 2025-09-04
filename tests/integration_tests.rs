use aui_next_generator::*;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

/// Helper to create a test config
fn create_test_config(name: &str, use_turbo: bool, use_react_query: bool) -> ProjectConfig {
    ProjectConfig::new(name.to_string(), false, use_turbo, use_react_query) // Skip deps for tests
}

// Helper function to generate project in a specific path (for testing)
fn generate_project_in_path(config: &ProjectConfig, path: &Path) -> anyhow::Result<()> {
    use aui_next_generator::config::DIRECTORIES;

    // Create directories
    for dir in DIRECTORIES {
        let dir_path = path.join(dir);
        fs::create_dir_all(&dir_path)?;
    }

    // Create files
    aui_next_generator::create_package_json(path, config)?;
    aui_next_generator::create_tsconfig(path)?;
    aui_next_generator::create_postcss_config(path)?;
    aui_next_generator::create_next_config(path)?;
    aui_next_generator::create_eslint_config(path)?;
    aui_next_generator::create_gitignore(path)?;
    aui_next_generator::create_npmrc(path)?;
    aui_next_generator::create_app_layout(path, &config.name, config)?;
    aui_next_generator::create_app_page(path, &config.name)?;
    aui_next_generator::create_globals_css(path)?;
    aui_next_generator::create_button_component(path)?;

    if config.use_react_query {
        aui_next_generator::create_query_provider(path)?;
        aui_next_generator::create_api_client(path)?;
        aui_next_generator::create_example_hooks(path)?;
    }

    aui_next_generator::create_readme(path, &config.name)?;

    Ok(())
}

#[test]
fn test_complete_project_generation() {
    let temp = TempDir::new().unwrap();
    let config = create_test_config("integration-test", false, false);
    let project_path = temp.path().join(&config.name);
    fs::create_dir(&project_path).unwrap();

    // Generate project in temp dir
    let result = generate_project_in_path(&config, &project_path);
    assert!(result.is_ok());

    // Assert that all expected files exist
    assert!(project_path.join("package.json").exists());
    assert!(project_path.join("tsconfig.json").exists());
    assert!(project_path.join("next.config.ts").exists());
    assert!(project_path.join(".eslintrc.json").exists());
    assert!(project_path.join(".gitignore").exists());
    assert!(project_path.join("postcss.config.mjs").exists());
    assert!(project_path.join(".npmrc").exists());
    assert!(project_path.join("README.md").exists());

    // Assert directory structure
    assert!(project_path.join("src").is_dir());
    assert!(project_path.join("src/app").is_dir());
    assert!(project_path.join("src/components").is_dir());
    assert!(project_path.join("src/styles").is_dir());
    assert!(project_path.join("public").is_dir());

    // Assert specific files
    assert!(project_path.join("src/app/layout.tsx").exists());
    assert!(project_path.join("src/app/page.tsx").exists());
    assert!(project_path.join("src/styles/globals.css").exists());
    assert!(project_path.join("src/components/Button.tsx").exists());
}

#[test]
fn test_project_generation_with_turbo() {
    let temp = TempDir::new().unwrap();
    let config = create_test_config("turbo-test", true, false);
    let project_path = temp.path().join(&config.name);
    fs::create_dir(&project_path).unwrap();

    let result = generate_project_in_path(&config, &project_path);
    assert!(result.is_ok());

    // Check that package.json contains turbo flag
    let content = fs::read_to_string(project_path.join("package.json")).unwrap();
    assert!(content.contains("next dev --turbo"));
}

#[test]
fn test_project_generation_without_turbo() {
    let temp = TempDir::new().unwrap();
    let config = create_test_config("no-turbo-test", false, false);
    let project_path = temp.path().join(&config.name);
    fs::create_dir(&project_path).unwrap();

    let result = generate_project_in_path(&config, &project_path);
    assert!(result.is_ok());

    // Check that package.json doesn't contain turbo flag
    let content = fs::read_to_string(project_path.join("package.json")).unwrap();
    assert!(content.contains("\"dev\": \"next dev\""));
    assert!(!content.contains("--turbo"));
}

#[test]
fn test_generated_files_content() {
    let temp = TempDir::new().unwrap();
    let config = create_test_config("content-test", false, false);
    let project_path = temp.path().join(&config.name);
    fs::create_dir(&project_path).unwrap();

    let result = generate_project_in_path(&config, &project_path);
    assert!(result.is_ok());

    // Test package.json content
    let package_content = fs::read_to_string(project_path.join("package.json")).unwrap();
    assert!(package_content.contains("\"name\": \"content-test\""));
    assert!(package_content.contains("\"next\": \"^15.0.0\""));
    assert!(package_content.contains("\"react\": \"^19.0.0\""));
    assert!(package_content.contains("\"typescript\": \"^5.0.0\""));

    // Test tsconfig.json content
    let tsconfig_content = fs::read_to_string(project_path.join("tsconfig.json")).unwrap();
    assert!(tsconfig_content.contains("\"@/*\": [\"./src/*\"]"));
    assert!(tsconfig_content.contains("\"strict\": true"));

    // Test Next.js config
    let next_config_content = fs::read_to_string(project_path.join("next.config.ts")).unwrap();
    assert!(next_config_content.contains("NextConfig"));
    assert!(next_config_content.contains("reactStrictMode: true"));

    // Test app layout
    let layout_content = fs::read_to_string(project_path.join("src/app/layout.tsx")).unwrap();
    assert!(layout_content.contains("title: 'content-test'"));
    assert!(layout_content.contains("import { Inter }"));

    // Test home page
    let page_content = fs::read_to_string(project_path.join("src/app/page.tsx")).unwrap();
    assert!(page_content.contains("Welcome to"));
    assert!(page_content.contains("content-test"));
    assert!(page_content.contains("Next.js 15"));
}

#[test]
fn test_directory_structure_creation() {
    let temp = TempDir::new().unwrap();
    let config = create_test_config("structure-test", false, false);
    let project_path = temp.path().join(&config.name);
    fs::create_dir(&project_path).unwrap();

    let result = generate_project_in_path(&config, &project_path);
    assert!(result.is_ok());

    // Test all expected directories exist
    let expected_dirs = [
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

    for dir in expected_dirs.iter() {
        assert!(
            project_path.join(dir).is_dir(),
            "Directory {} should exist",
            dir
        );
    }
}

#[test]
fn test_existing_directory_error() {
    let temp = TempDir::new().unwrap();
    let project_name = "existing-test";
    let config = create_test_config(project_name, false, false);

    // Create the directory first
    let existing_dir = temp.path().join(project_name);
    fs::create_dir(&existing_dir).unwrap();

    // Change to temp directory
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(temp.path()).unwrap();

    // Try to generate project - should fail
    let result = generate_project(&config);
    assert!(result.is_err());

    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();

    let error_msg = result.err().unwrap().to_string();
    assert!(error_msg.contains("already exists"));
}

#[test]
fn test_project_generation_with_react_query() {
    let temp = TempDir::new().unwrap();
    let config = create_test_config("react-query-test", false, true);
    let project_path = temp.path().join(&config.name);
    fs::create_dir(&project_path).unwrap();

    let result = generate_project_in_path(&config, &project_path);
    assert!(result.is_ok());

    // Check that React Query dependencies are in package.json
    let package_content = fs::read_to_string(project_path.join("package.json")).unwrap();
    assert!(package_content.contains("@tanstack/react-query"));
    assert!(package_content.contains("@tanstack/react-query-devtools"));

    // Check that React Query files were created
    assert!(project_path.join("src/libs/query-provider.tsx").exists());
    assert!(project_path.join("src/libs/api.ts").exists());
    assert!(project_path.join("src/hooks/use-api.ts").exists());

    // Check that layout includes QueryProvider
    let layout_content = fs::read_to_string(project_path.join("src/app/layout.tsx")).unwrap();
    assert!(layout_content.contains("import { QueryProvider }"));
    assert!(layout_content.contains("<QueryProvider>"));
    assert!(layout_content.contains("</QueryProvider>"));

    // Check query provider content
    let provider_content =
        fs::read_to_string(project_path.join("src/libs/query-provider.tsx")).unwrap();
    assert!(provider_content.contains("QueryClient"));
    assert!(provider_content.contains("ReactQueryDevtools"));
    assert!(provider_content.contains("staleTime: 60 * 1000"));

    // Check API client content
    let api_content = fs::read_to_string(project_path.join("src/libs/api.ts")).unwrap();
    assert!(api_content.contains("ApiError"));
    assert!(api_content.contains("apiRequest"));
    assert!(api_content.contains("jsonplaceholder.typicode.com"));

    // Check example hooks content
    let hooks_content = fs::read_to_string(project_path.join("src/hooks/use-api.ts")).unwrap();
    assert!(hooks_content.contains("useQuery"));
    assert!(hooks_content.contains("useMutation"));
    assert!(hooks_content.contains("usePosts"));
    assert!(hooks_content.contains("useCreatePost"));
}
