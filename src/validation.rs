use anyhow::Result;
use colored::Colorize;
use std::process::Command;

pub fn check_node_version() -> Result<()> {
    let output = Command::new("node").arg("--version").output()?;

    if !output.status.success() {
        anyhow::bail!("Node.js is not installed or not in PATH");
    }

    let version_output = String::from_utf8_lossy(&output.stdout);
    let version_str = version_output
        .trim()
        .strip_prefix('v')
        .unwrap_or(&version_output.trim());

    // Parse major and minor version
    let parts: Vec<&str> = version_str.split('.').collect();
    if parts.len() < 2 {
        anyhow::bail!("Could not parse Node.js version: {}", version_str);
    }

    let major: u32 = parts[0]
        .parse()
        .map_err(|_| anyhow::anyhow!("Invalid major version"))?;
    let minor: u32 = parts[1]
        .parse()
        .map_err(|_| anyhow::anyhow!("Invalid minor version"))?;

    // Next.js 15 requires Node.js 18.18.0+
    let required_major = 18;
    let required_minor = 18;

    if major > required_major || (major == required_major && minor >= required_minor) {
        println!("✅ Node.js {} (compatible)", version_str);
        Ok(())
    } else {
        println!("{}", "❌ Node.js version is too old".red());
        println!("   Current: v{}", version_str);
        println!(
            "   Required: v{}.{}.0 or higher",
            required_major, required_minor
        );
        println!();
        println!("Please update Node.js:");
        println!("   https://nodejs.org/");
        println!("   # or use a version manager like nvm");
        anyhow::bail!("Node.js version {} is not supported", version_str);
    }
}

pub fn check_and_install_pnpm() -> Result<()> {
    use dialoguer::Confirm;

    match which::which("pnpm") {
        Ok(_) => {
            println!("✅ pnpm found");
            Ok(())
        }
        Err(_) => {
            println!("{}", "⚠️  pnpm not found".yellow());

            let install_pnpm = Confirm::new()
                .with_prompt("Install pnpm globally?")
                .default(true)
                .interact()?;

            if install_pnpm {
                install_pnpm_global()?;
            } else {
                println!(
                    "{}",
                    "❌ pnpm is required. Please install it manually:".red()
                );
                println!("   npm install -g pnpm");
                println!("   # or");
                println!("   curl -fsSL https://get.pnpm.io/install.sh | sh -");
                anyhow::bail!("pnpm installation required");
            }
            Ok(())
        }
    }
}

fn install_pnpm_global() -> Result<()> {
    println!("{}", "📦 Installing pnpm globally...".blue());

    let output = Command::new("npm")
        .args(&["install", "-g", "pnpm"])
        .output()?;

    if output.status.success() {
        println!("{}", "✅ pnpm installed successfully!".green());
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        println!("{}", "❌ Failed to install pnpm".red());
        println!("Error: {}", error);
        anyhow::bail!("pnpm installation failed");
    }

    Ok(())
}

pub fn install_dependencies_with_pnpm(project_path: &std::path::Path) -> Result<()> {
    let output = Command::new("pnpm")
        .arg("install")
        .current_dir(project_path)
        .output()?;

    if output.status.success() {
        println!("{}", "✅ Dependencies installed successfully!".green());
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        println!("{}", "❌ Failed to install dependencies".red());
        println!("Error: {}", error);
        println!("You can install manually with: pnpm install");
    }

    Ok(())
}

// Helper function for testing - extracts version parsing logic
pub fn parse_node_version(version_str: &str) -> Result<(u32, u32)> {
    let clean_version = version_str.strip_prefix('v').unwrap_or(version_str);
    let parts: Vec<&str> = clean_version.split('.').collect();

    if parts.len() < 2 {
        anyhow::bail!("Could not parse Node.js version: {}", clean_version);
    }

    let major: u32 = parts[0]
        .parse()
        .map_err(|_| anyhow::anyhow!("Invalid major version"))?;
    let minor: u32 = parts[1]
        .parse()
        .map_err(|_| anyhow::anyhow!("Invalid minor version"))?;

    Ok((major, minor))
}

// Helper function for testing - checks if version is compatible
pub fn is_node_version_compatible(major: u32, minor: u32) -> bool {
    let required_major = 18;
    let required_minor = 18;
    major > required_major || (major == required_major && minor >= required_minor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_node_version_with_v_prefix() {
        let result = parse_node_version("v20.10.0");
        assert!(result.is_ok());
        let (major, minor) = result.unwrap();
        assert_eq!(major, 20);
        assert_eq!(minor, 10);
    }

    #[test]
    fn test_parse_node_version_without_v_prefix() {
        let result = parse_node_version("18.18.2");
        assert!(result.is_ok());
        let (major, minor) = result.unwrap();
        assert_eq!(major, 18);
        assert_eq!(minor, 18);
    }

    #[test]
    fn test_parse_node_version_invalid_format() {
        let result = parse_node_version("invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_node_version_only_major() {
        let result = parse_node_version("20");
        assert!(result.is_err());
    }

    #[test]
    fn test_is_node_version_compatible_valid_versions() {
        // Exact minimum version
        assert!(is_node_version_compatible(18, 18));
        // Higher minor version
        assert!(is_node_version_compatible(18, 19));
        // Higher major version
        assert!(is_node_version_compatible(20, 0));
        // Much higher version
        assert!(is_node_version_compatible(22, 5));
    }

    #[test]
    fn test_is_node_version_compatible_invalid_versions() {
        // Lower major version
        assert!(!is_node_version_compatible(17, 99));
        // Same major but lower minor
        assert!(!is_node_version_compatible(18, 17));
        // Much lower version
        assert!(!is_node_version_compatible(16, 0));
    }

    #[test]
    fn test_node_version_edge_cases() {
        // Test boundary conditions
        assert!(!is_node_version_compatible(18, 17)); // Just below
        assert!(is_node_version_compatible(18, 18)); // Exact match
        assert!(is_node_version_compatible(18, 19)); // Just above
        assert!(is_node_version_compatible(19, 0)); // Next major
    }

    #[test]
    fn test_parse_node_version_with_patch() {
        let result = parse_node_version("v18.18.2");
        assert!(result.is_ok());
        let (major, minor) = result.unwrap();
        assert_eq!(major, 18);
        assert_eq!(minor, 18);
        // We don't parse patch version, but it shouldn't break
    }

    #[test]
    fn test_parse_node_version_with_prerelease() {
        let result = parse_node_version("v20.0.0-pre");
        assert!(result.is_ok());
        let (major, minor) = result.unwrap();
        assert_eq!(major, 20);
        assert_eq!(minor, 0);
    }
}
