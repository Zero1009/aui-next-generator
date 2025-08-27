use anyhow::Result;
use clap::{Parser, arg};
use colored::Colorize;
use dialoguer::{Confirm, Input};
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Parser)]
#[command(name = "aui-next-gen")]
#[command(about = "Generate Next.js projects with Tailwind & ESLint using pnpm")]
struct Cli {
    /// Project name (optional - will prompt if not provided)
    name: Option<String>,

    /// Skip dependency installation
    #[arg(long)]
    skip_install: bool,
}

static DIRECTORIES: &[&str] = &[
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

fn main() -> Result<()> {
    let args = Cli::parse();

    println!("{}", "🚀 AUI Next.js Generator".bold().blue());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".blue());

    let project_name = match args.name {
        Some(name) => {
            println!("{}", name.green());
            name
        }
        None => {
            let name: String = Input::new()
                .with_prompt("📝 Enter project name")
                .interact()?;

            if name.trim().is_empty() {
                anyhow::bail!("Project name cannot be empty");
            }
            name
        }
    };

    let install_deps = if args.skip_install {
        false
    } else {
        check_and_install_pnpm()?;

        Confirm::new()
            .with_prompt("📦 Install project dependencies")
            .default(true)
            .interact()?
    };

    generate_project(&project_name, install_deps)?;

    println!("\n{}", "🎉 Project created successfully!".green().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".green());
    println!("Next steps:");
    println!("   cd {}", project_name.blue());
    if !install_deps {
        println!("   pnpm install");
    }
    println!("   pnpm dev");
    println!("\n🌐 Then open http://localhost:3000");

    Ok(())
}

fn check_and_install_pnpm() -> Result<()> {
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

fn generate_project(name: &str, install_deps: bool) -> Result<()> {
    let project_path = Path::new(name);

    if project_path.exists() {
        anyhow::bail!("Directory '{}' already exists!", name);
    }
    println!("\n🏗️  Creating project: {}", name.yellow());

    fs::create_dir(project_path)?;

    create_directories(project_path)?;

    create_files(project_path, name)?;

    if install_deps {
        install_dependencies_with_pnpm(project_path)?;
    }

    Ok(())
}

fn create_directories(project_path: &Path) -> Result<()> {
    println!("{}", "📁 Creating directory structure...".blue());

    for dir in DIRECTORIES {
        let dir_path = project_path.join(dir);
        fs::create_dir_all(&dir_path)?;
        println!("   Created: {}", dir.green());
    }

    Ok(())
}

fn create_files(project_path: &Path, project_name: &str) -> Result<()> {
    println!("{}", "📝 Creating project files...".blue());

    create_package_json(project_path, project_name)?;
    create_tsconfig(project_path)?;
    create_postcss_config(project_path)?;
    create_next_config(project_path)?;
    create_eslint_config(project_path)?;
    create_gitignore(project_path)?;
    create_npmrc(project_path)?;
    create_app_layout(project_path, project_name)?;
    create_app_page(project_path, project_name)?;
    create_globals_css(project_path)?;
    create_button_component(project_path)?;
    create_readme(project_path, project_name)?;

    Ok(())
}

fn install_dependencies_with_pnpm(project_path: &Path) -> Result<()> {
    println!("{}", "📦 Installing dependencies with pnpm...".blue());

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

fn create_package_json(project_path: &Path, name: &str) -> Result<()> {
    let content = format!(r#"{{
  "name": "{}",
  "version": "0.1.0",
  "private": true,
  "scripts": {{
    "dev": "next dev",
    "build": "next build",
    "start": "next start",
    "lint": "next lint",
    "lint:fix": "next lint --fix"
  }},
  "dependencies": {{
    "next": "^14.0.0",
    "react": "^18.0.0",
    "react-dom": "^18.0.0"
  }},
  "devDependencies": {{
    "@types/node": "^20.0.0",
    "@types/react": "^18.0.0",
    "@types/react-dom": "^18.0.0",
    "eslint": "^8.0.0",
    "eslint-config-next": "^14.0.0",
    "tailwindcss": "^4.0.0-alpha.31",
    "typescript": "^5.0.0",
    "clsx": "^2.0.0",
    "tailwind-merge": "^2.0.0"
  }}
}}"#, name);

    fs::write(project_path.join("package.json"), content)?;
    println!("   Created: {}", "package.json".green());
    Ok(())
}

fn create_tsconfig(project_path: &Path) -> Result<()> {
    let content = r#"{
  "compilerOptions": {
    "target": "es5",
    "lib": ["dom", "dom.iterable", "es6"],
    "allowJs": true,
    "skipLibCheck": true,
    "strict": true,
    "noEmit": true,
    "esModuleInterop": true,
    "module": "esnext",
    "moduleResolution": "bundler",
    "resolveJsonModule": true,
    "isolatedModules": true,
    "jsx": "preserve",
    "incremental": true,
    "plugins": [
      {
        "name": "next"
      }
    ],
    "paths": {
      "@/*": ["./src/*"],
      "@/components/*": ["./src/components/*"],
      "@/libs/*": ["./src/libs/*"],
      "@/hooks/*": ["./src/hooks/*"],
      "@/types/*": ["./src/types/*"],
      "@/constants/*": ["./src/constants/*"],
      "@/assets/*": ["./src/assets/*"]
    }
  },
  "include": ["next-env.d.ts", "**/*.ts", "**/*.tsx", ".next/types/**/*.ts"],
  "exclude": ["node_modules"]
}"#;

    fs::write(project_path.join("tsconfig.json"), content)?;
    println!("   Created: {}", "tsconfig.json".green());
    Ok(())
}


fn create_postcss_config(project_path: &Path) -> Result<()> {
    let content = r#"const config = {
    plugins: {
        "@tailwindcss/postcss": {},
    },
};
export default config;
"#;

    fs::write(project_path.join("postcss.config.mjs"), content)?;
    println!("   Created: {}", "postcss.config.mjs".green());
    Ok(())
}

fn create_next_config(project_path: &Path) -> Result<()> {
    let content = r#"/** @type {import('next').NextConfig} */
const nextConfig = {
  experimental: {
    appDir: true,
  },
}

module.exports = nextConfig"#;

    fs::write(project_path.join("next.config.js"), content)?;
    println!("   Created: {}", "next.config.js".green());
    Ok(())
}

fn create_eslint_config(project_path: &Path) -> Result<()> {
    let content = r#"{
  "extends": ["next/core-web-vitals"],
  "rules": {
    "prefer-const": "error",
    "no-unused-vars": "warn",
    "no-console": "warn"
  }
}"#;

    fs::write(project_path.join(".eslintrc.json"), content)?;
    println!("   Created: {}", ".eslintrc.json".green());
    Ok(())
}

fn create_gitignore(project_path: &Path) -> Result<()> {
    let content = r#"# Dependencies
/node_modules
/.pnp
.pnp.js

# Testing
/coverage

# Next.js
/.next/
/out/

# Production
/build

# Misc
.DS_Store
*.pem

# Debug
npm-debug.log*
yarn-debug.log*
yarn-error.log*
pnpm-debug.log*

# Local env files
.env*.local

# Vercel
.vercel

# TypeScript
*.tsbuildinfo
next-env.d.ts
"#;

    fs::write(project_path.join(".gitignore"), content)?;
    println!("   Created: {}", ".gitignore".green());
    Ok(())
}

fn create_npmrc(project_path: &Path) -> Result<()> {
    let content = r#"auto-install-peers=true
strict-peer-dependencies=false
"#;

    fs::write(project_path.join(".npmrc"), content)?;
    println!("   Created: {}", ".npmrc".green());
    Ok(())
}

fn create_app_layout(project_path: &Path, project_name: &str) -> Result<()> {
    let content = format!(r#"import type {{ Metadata }} from 'next'
import {{ Inter }} from 'next/font/google'
import '@/styles/globals.css'

const inter = Inter({{ subsets: ['latin'] }})

export const metadata: Metadata = {{
  title: '{}',
  description: 'Generated with AUI Next.js Generator',
}}

export default function RootLayout({{
  children,
}}: {{
  children: React.ReactNode
}}) {{
  return (
    <html lang="en">
      <body className={{inter.className}}>{{children}}</body>
    </html>
  )
}}
"#, project_name);

    fs::write(project_path.join("src/app/layout.tsx"), content)?;
    println!("   Created: {}", "src/app/layout.tsx".green());
    Ok(())
}

fn create_app_page(project_path: &Path, project_name: &str) -> Result<()> {
    let content = format!(r#"export default function Home() {{
  return (
    <main className="flex min-h-screen flex-col items-center justify-center p-24">
      <div className="z-10 max-w-5xl w-full items-center justify-between font-mono text-sm lg:flex">
        <h1 className="text-4xl font-bold text-center lg:text-left">
          Welcome to{{' '}}
          <span className="text-blue-600">{}</span>
        </h1>
      </div>
      
      <div className="mt-8 grid text-center lg:max-w-5xl lg:w-full lg:mb-0 lg:grid-cols-3 lg:text-left">
        <div className="group rounded-lg border border-transparent px-5 py-4 transition-colors hover:border-gray-300 hover:bg-gray-100">
          <h2 className="mb-3 text-2xl font-semibold">
            Next.js 14
          </h2>
          <p className="m-0 max-w-[30ch] text-sm opacity-50">
            The React Framework for Production with App Router
          </p>
        </div>

        <div className="group rounded-lg border border-transparent px-5 py-4 transition-colors hover:border-gray-300 hover:bg-gray-100">
          <h2 className="mb-3 text-2xl font-semibold">
            Tailwind CSS
          </h2>
          <p className="m-0 max-w-[30ch] text-sm opacity-50">
            Utility-first CSS framework for rapid UI development
          </p>
        </div>

        <div className="group rounded-lg border border-transparent px-5 py-4 transition-colors hover:border-gray-300 hover:bg-gray-100">
          <h2 className="mb-3 text-2xl font-semibold">
            TypeScript
          </h2>
          <p className="m-0 max-w-[30ch] text-sm opacity-50">
            Type safety and enhanced developer experience
          </p>
        </div>
      </div>
    </main>
  )
}}
"#, project_name);

    fs::write(project_path.join("src/app/page.tsx"), content)?;
    println!("   Created: {}", "src/app/page.tsx".green());
    Ok(())
}

fn create_globals_css(project_path: &Path) -> Result<()> {
    let content = r#"@import "tailwindcss";

/* Custom CSS Variables */
:root {
  --background: #ffffff;
  --foreground: #171717;
}

@media (prefers-color-scheme: dark) {
  :root {
    --background: #0a0a0a;
    --foreground: #ededed;
  }
}

/* Base Styles */
body {
  color: var(--foreground);
  background: var(--background);
  font-family: Inter, system-ui, -apple-system, sans-serif;
}

/* Custom Utility Classes */
@utility text-balance {
  text-wrap: balance;
}

/* Component Styles */
@layer components {
  .btn {
    @apply font-medium rounded-md transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2;
  }
  
  .btn-primary {
    @apply bg-blue-600 text-white hover:bg-blue-700 focus:ring-blue-500;
  }
  
  .btn-secondary {
    @apply bg-gray-600 text-white hover:bg-gray-700 focus:ring-gray-500;
  }
  
  .btn-outline {
    @apply border border-gray-300 text-gray-700 hover:bg-gray-50 focus:ring-blue-500;
  }
}
"#;

    fs::write(project_path.join("src/styles/globals.css"), content)?;
    println!("   Created: {}", "src/styles/globals.css".green());
    Ok(())
}

fn create_button_component(project_path: &Path) -> Result<()> {
    let content = r#"import React from 'react'

interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: 'primary' | 'secondary' | 'outline'
  size?: 'sm' | 'md' | 'lg'
}

export const Button: React.FC<ButtonProps> = ({
  children,
  variant = 'primary',
  size = 'md',
  className = '',
  ...props
}) => {
  const baseClasses = 'font-medium rounded-md transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2'
  
  const variantClasses = {
    primary: 'bg-blue-600 text-white hover:bg-blue-700 focus:ring-blue-500',
    secondary: 'bg-gray-600 text-white hover:bg-gray-700 focus:ring-gray-500',
    outline: 'border border-gray-300 text-gray-700 hover:bg-gray-50 focus:ring-blue-500'
  }
  
  const sizeClasses = {
    sm: 'px-3 py-1.5 text-sm',
    md: 'px-4 py-2 text-base',
    lg: 'px-6 py-3 text-lg'
  }
  
  return (
    <button
      className={`${baseClasses} ${variantClasses[variant]} ${sizeClasses[size]} ${className}`}
      {...props}
    >
      {children}
    </button>
  )
}
"#;

    fs::write(project_path.join("src/components/Button.tsx"), content)?;
    println!("   Created: {}", "src/components/Button.tsx".green());
    Ok(())
}

fn create_readme(project_path: &Path, project_name: &str) -> Result<()> {
    let content = format!(r#"# {}

A modern Next.js application with Tailwind CSS, ESLint, and TypeScript.

## Getting Started

Install dependencies:

```bash
pnpm install
```

Run the development server:

```bash
pnpm dev
```

Open [http://localhost:3000](http://localhost:3000) with your browser to see the result.

## Features

- ⚡ Next.js 14 with App Router
- 🎨 Tailwind CSS for styling
- 📝 TypeScript for type safety
- 🔧 ESLint for code linting
- 🚀 pnpm for fast package management

## Project Structure

```
{}
├── src/
│   ├── app/               # Next.js App Router pages
│   │   ├── layout.tsx
│   │   └── page.tsx
│   ├── components/        # UI Components (Table, Box, Text, Spinner, etc.)
│   │   └── Button.tsx
│   ├── constants/         # Static constants (Tabs, Roles, etc.)
│   ├── hooks/             # Custom Hooks (React Query, Zustand store)
│   ├── libs/              # Utilities (api.ts, dropdown.ts, formatter.ts)
│   ├── assets/            # Images, animations (e.g., Lottie files)
│   ├── types/             # Shared TypeScript types (API, DTOs)
│   ├── fonts/             # Custom fonts (THSarabun for PDFs)
│   └── styles/            # Tailwind config and global styles
│       └── globals.css
├── public/
├── tsconfig.json
└── package.json
```

## Tailwind CSS v4

This project uses Tailwind CSS v4 with CSS-based configuration. No separate config file needed - all customization is done through CSS imports and layers in `src/styles/globals.css`.

## Learn More

To learn more about the technologies used in this project:

- [Next.js Documentation](https://nextjs.org/docs)
- [Tailwind CSS](https://tailwindcss.com/docs)
- [TypeScript](https://www.typescriptlang.org/)

## Deploy

Deploy easily with [Vercel](https://vercel.com/):

[![Deploy with Vercel](https://vercel.com/button)](https://vercel.com/new/clone?repository-url=https://github.com/your-username/your-repo)
"#, project_name, project_name);

    fs::write(project_path.join("README.md"), content)?;
    println!("   Created: {}", "README.md".green());
    Ok(())
}