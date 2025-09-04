use anyhow::Result;
use colored::Colorize;
use std::fs;
use std::path::Path;

use crate::config::ProjectConfig;

pub fn create_package_json(project_path: &Path, config: &ProjectConfig) -> Result<()> {
    let dev_script = if config.use_turbo {
        "next dev --turbo"
    } else {
        "next dev"
    };

    let react_query_deps = if config.use_react_query {
        r#",
    "@tanstack/react-query": "^5.59.0",
    "@tanstack/react-query-devtools": "^5.59.0""#
    } else {
        ""
    };

    let content = format!(
        r#"{{
  "name": "{}",
  "version": "0.1.0",
  "private": true,
  "scripts": {{
    "dev": "{}",
    "build": "next build",
    "start": "next start",
    "lint": "next lint",
    "lint:fix": "next lint --fix"
  }},
  "dependencies": {{
    "next": "^15.0.0",
    "react": "^19.0.0",
    "react-dom": "^19.0.0"{}
  }},
  "devDependencies": {{
    "@types/node": "^20.0.0",
    "@types/react": "^19.0.0",
    "@types/react-dom": "^19.0.0",
    "eslint": "^9.0.0",
    "eslint-config-next": "^15.0.0",
    "tailwindcss": "^4.0.0-alpha.31",
    "@tailwindcss/postcss": "^4.0.0-alpha.31",
    "typescript": "^5.0.0",
    "clsx": "^2.0.0",
    "tailwind-merge": "^2.0.0"
  }}
}}"#,
        config.name, dev_script, react_query_deps
    );

    fs::write(project_path.join("package.json"), content)?;
    println!("   Created: {}", "package.json".green());
    Ok(())
}

pub fn create_tsconfig(project_path: &Path) -> Result<()> {
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

pub fn create_postcss_config(project_path: &Path) -> Result<()> {
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

pub fn create_next_config(project_path: &Path) -> Result<()> {
    let content = r#"import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  reactStrictMode: true,
  typescript: {
    ignoreBuildErrors: false,
  },
  eslint: {
    ignoreDuringBuilds: false,
  },
};

export default nextConfig;"#;

    fs::write(project_path.join("next.config.ts"), content)?;
    println!("   Created: {}", "next.config.ts".green());
    Ok(())
}

pub fn create_eslint_config(project_path: &Path) -> Result<()> {
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

pub fn create_gitignore(project_path: &Path) -> Result<()> {
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

pub fn create_npmrc(project_path: &Path) -> Result<()> {
    let content = r#"auto-install-peers=true
strict-peer-dependencies=false
"#;

    fs::write(project_path.join(".npmrc"), content)?;
    println!("   Created: {}", ".npmrc".green());
    Ok(())
}

pub fn create_app_layout(
    project_path: &Path,
    project_name: &str,
    config: &ProjectConfig,
) -> Result<()> {
    let (query_import, query_wrapper_open, query_wrapper_close) = if config.use_react_query {
        (
            "import { QueryProvider } from '@/libs/query-provider'\n",
            "<QueryProvider>",
            "</QueryProvider>",
        )
    } else {
        ("", "", "")
    };

    let content = format!(
        r#"import type {{ Metadata }} from 'next'
import {{ Inter }} from 'next/font/google'
import '@/styles/globals.css'
{}
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
      <body className={{inter.className}}>
        {}
          {{children}}
        {}
      </body>
    </html>
  )
}}
"#,
        query_import, project_name, query_wrapper_open, query_wrapper_close
    );

    fs::write(project_path.join("src/app/layout.tsx"), content)?;
    println!("   Created: {}", "src/app/layout.tsx".green());
    Ok(())
}

pub fn create_app_page(project_path: &Path, project_name: &str) -> Result<()> {
    let content = format!(
        r#"export default function Home() {{
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
            Next.js 15
          </h2>
          <p className="m-0 max-w-[30ch] text-sm opacity-50">
            The React Framework for Production with App Router
          </p>
        </div>

        <div className="group rounded-lg border border-transparent px-5 py-4 transition-colors hover:border-gray-300 hover:bg-gray-100">
          <h2 className="mb-3 text-2xl font-semibent">
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
"#,
        project_name
    );

    fs::write(project_path.join("src/app/page.tsx"), content)?;
    println!("   Created: {}", "src/app/page.tsx".green());
    Ok(())
}

pub fn create_globals_css(project_path: &Path) -> Result<()> {
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

pub fn create_button_component(project_path: &Path) -> Result<()> {
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

pub fn create_query_provider(project_path: &Path) -> Result<()> {
    let content = r#"'use client'

import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { ReactQueryDevtools } from '@tanstack/react-query-devtools'
import { useState, type ReactNode } from 'react'

interface QueryProviderProps {
  children: ReactNode
}

export function QueryProvider({ children }: QueryProviderProps) {
  const [queryClient] = useState(
    () =>
      new QueryClient({
        defaultOptions: {
          queries: {
            // With SSR, we usually want to set some default staleTime
            // above 0 to avoid refetching immediately on the client
            staleTime: 60 * 1000, // 1 minute
            retry: 1,
          },
        },
      })
  )

  return (
    <QueryClientProvider client={queryClient}>
      {children}
      <ReactQueryDevtools initialIsOpen={false} />
    </QueryClientProvider>
  )
}
"#;

    fs::write(project_path.join("src/libs/query-provider.tsx"), content)?;
    println!("   Created: {}", "src/libs/query-provider.tsx".green());
    Ok(())
}

pub fn create_api_client(project_path: &Path) -> Result<()> {
    let content = r#"// API configuration and utilities for React Query

const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || 'https://jsonplaceholder.typicode.com'

export class ApiError extends Error {
  constructor(public status: number, message: string) {
    super(message)
    this.name = 'ApiError'
  }
}

export async function apiRequest<T>(
  endpoint: string,
  options: RequestInit = {}
): Promise<T> {
  const url = `${API_BASE_URL}${endpoint}`

  const response = await fetch(url, {
    headers: {
      'Content-Type': 'application/json',
      ...options.headers,
    },
    ...options,
  })

  if (!response.ok) {
    throw new ApiError(response.status, `HTTP ${response.status}: ${response.statusText}`)
  }

  return response.json()
}

// Example API functions using React Query patterns
export const api = {
  // GET requests
  get: <T>(endpoint: string) => apiRequest<T>(endpoint),

  // POST requests
  post: <T>(endpoint: string, data: unknown) =>
    apiRequest<T>(endpoint, {
      method: 'POST',
      body: JSON.stringify(data),
    }),

  // PUT requests
  put: <T>(endpoint: string, data: unknown) =>
    apiRequest<T>(endpoint, {
      method: 'PUT',
      body: JSON.stringify(data),
    }),

  // DELETE requests
  delete: <T>(endpoint: string) =>
    apiRequest<T>(endpoint, {
      method: 'DELETE',
    }),
}
"#;

    fs::write(project_path.join("src/libs/api.ts"), content)?;
    println!("   Created: {}", "src/libs/api.ts".green());
    Ok(())
}

pub fn create_example_hooks(project_path: &Path) -> Result<()> {
    let content = r#"// Example React Query hooks

import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { api } from '@/libs/api'

// Example types (replace with your actual types)
interface Post {
  id: number
  title: string
  body: string
  userId: number
}

interface User {
  id: number
  name: string
  email: string
}

// Query hooks
export function usePosts() {
  return useQuery({
    queryKey: ['posts'],
    queryFn: () => api.get<Post[]>('/posts'),
  })
}

export function usePost(id: number) {
  return useQuery({
    queryKey: ['posts', id],
    queryFn: () => api.get<Post>(`/posts/${id}`),
    enabled: !!id, // Only run query if id exists
  })
}

export function useUser(id: number) {
  return useQuery({
    queryKey: ['users', id],
    queryFn: () => api.get<User>(`/users/${id}`),
    enabled: !!id,
  })
}

// Mutation hooks
export function useCreatePost() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (newPost: Omit<Post, 'id'>) => api.post<Post>('/posts', newPost),
    onSuccess: () => {
      // Invalidate and refetch posts
      queryClient.invalidateQueries({ queryKey: ['posts'] })
    },
  })
}

export function useUpdatePost() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (post: Post) => api.put<Post>(`/posts/${post.id}`, post),
    onSuccess: (data) => {
      // Update the specific post in cache
      queryClient.setQueryData(['posts', data.id], data)
      // Also invalidate the posts list
      queryClient.invalidateQueries({ queryKey: ['posts'] })
    },
  })
}

export function useDeletePost() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (id: number) => api.delete(`/posts/${id}`),
    onSuccess: () => {
      // Invalidate posts list
      queryClient.invalidateQueries({ queryKey: ['posts'] })
    },
  })
}
"#;

    fs::write(project_path.join("src/hooks/use-api.ts"), content)?;
    println!("   Created: {}", "src/hooks/use-api.ts".green());
    Ok(())
}

pub fn create_readme(project_path: &Path, project_name: &str) -> Result<()> {
    let content = format!(
        r#"# {}

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

- ⚡ Next.js 15 with App Router
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
"#,
        project_name, project_name
    );

    fs::write(project_path.join("README.md"), content)?;
    println!("   Created: {}", "README.md".green());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn setup_test_dir() -> TempDir {
        TempDir::new().expect("Failed to create temp directory")
    }

    fn create_test_config(name: &str, use_turbo: bool, use_react_query: bool) -> ProjectConfig {
        ProjectConfig::new(name.to_string(), true, use_turbo, use_react_query)
    }

    #[test]
    fn test_create_package_json_without_turbo() {
        let temp_dir = setup_test_dir();
        let config = create_test_config("test-project", false, false);

        let result = create_package_json(temp_dir.path(), &config);
        assert!(result.is_ok());

        let content = fs::read_to_string(temp_dir.path().join("package.json")).unwrap();
        assert!(content.contains("\"name\": \"test-project\""));
        assert!(content.contains("\"dev\": \"next dev\""));
        assert!(!content.contains("--turbo"));
        assert!(content.contains("\"next\": \"^15.0.0\""));
        assert!(content.contains("\"react\": \"^19.0.0\""));
    }

    #[test]
    fn test_create_package_json_with_turbo() {
        let temp_dir = setup_test_dir();
        let config = create_test_config("turbo-project", true, false);

        let result = create_package_json(temp_dir.path(), &config);
        assert!(result.is_ok());

        let content = fs::read_to_string(temp_dir.path().join("package.json")).unwrap();
        assert!(content.contains("\"name\": \"turbo-project\""));
        assert!(content.contains("\"dev\": \"next dev --turbo\""));
        assert!(content.contains("--turbo"));
    }

    #[test]
    fn test_create_tsconfig() {
        let temp_dir = setup_test_dir();

        let result = create_tsconfig(temp_dir.path());
        assert!(result.is_ok());

        let content = fs::read_to_string(temp_dir.path().join("tsconfig.json")).unwrap();
        assert!(content.contains("\"target\": \"es5\""));
        assert!(content.contains("\"strict\": true"));
        assert!(content.contains("\"@/*\": [\"./src/*\"]"));
        assert!(content.contains("\"name\": \"next\""));
    }

    #[test]
    fn test_create_next_config() {
        let temp_dir = setup_test_dir();

        let result = create_next_config(temp_dir.path());
        assert!(result.is_ok());

        let content = fs::read_to_string(temp_dir.path().join("next.config.ts")).unwrap();
        assert!(content.contains("NextConfig"));
        assert!(content.contains("reactStrictMode: true"));
        assert!(content.contains("ignoreBuildErrors: false"));
        assert!(content.contains("export default nextConfig"));
    }

    #[test]
    fn test_create_eslint_config() {
        let temp_dir = setup_test_dir();

        let result = create_eslint_config(temp_dir.path());
        assert!(result.is_ok());

        let content = fs::read_to_string(temp_dir.path().join(".eslintrc.json")).unwrap();
        assert!(content.contains("\"next/core-web-vitals\""));
        assert!(content.contains("\"prefer-const\": \"error\""));
        assert!(content.contains("\"no-unused-vars\": \"warn\""));
    }

    #[test]
    fn test_create_postcss_config() {
        let temp_dir = setup_test_dir();

        let result = create_postcss_config(temp_dir.path());
        assert!(result.is_ok());

        let content = fs::read_to_string(temp_dir.path().join("postcss.config.mjs")).unwrap();
        assert!(content.contains("@tailwindcss/postcss"));
        assert!(content.contains("export default config"));
    }

    #[test]
    fn test_create_gitignore() {
        let temp_dir = setup_test_dir();

        let result = create_gitignore(temp_dir.path());
        assert!(result.is_ok());

        let content = fs::read_to_string(temp_dir.path().join(".gitignore")).unwrap();
        assert!(content.contains("node_modules"));
        assert!(content.contains(".next/"));
        assert!(content.contains(".env*.local"));
        assert!(content.contains("*.tsbuildinfo"));
    }

    #[test]
    fn test_create_app_layout() {
        let temp_dir = setup_test_dir();
        // Create src/app directory
        fs::create_dir_all(temp_dir.path().join("src/app")).unwrap();

        let config = create_test_config("my-awesome-app", false, false);
        let result = create_app_layout(temp_dir.path(), "my-awesome-app", &config);
        assert!(result.is_ok());

        let content = fs::read_to_string(temp_dir.path().join("src/app/layout.tsx")).unwrap();
        assert!(content.contains("title: 'my-awesome-app'"));
        assert!(content.contains("import { Inter }"));
        assert!(content.contains("@/styles/globals.css"));
        assert!(content.contains("RootLayout"));
        assert!(!content.contains("QueryProvider"));
    }

    #[test]
    fn test_create_app_layout_with_react_query() {
        let temp_dir = setup_test_dir();
        // Create src/app directory
        fs::create_dir_all(temp_dir.path().join("src/app")).unwrap();
        let config = create_test_config("my-awesome-app", false, true);

        let result = create_app_layout(temp_dir.path(), "my-awesome-app", &config);
        assert!(result.is_ok());

        let content = fs::read_to_string(temp_dir.path().join("src/app/layout.tsx")).unwrap();
        assert!(content.contains("title: 'my-awesome-app'"));
        assert!(content.contains("import { QueryProvider }"));
        assert!(content.contains("<QueryProvider>"));
        assert!(content.contains("</QueryProvider>"));
    }

    #[test]
    fn test_create_app_page() {
        let temp_dir = setup_test_dir();
        // Create src/app directory
        fs::create_dir_all(temp_dir.path().join("src/app")).unwrap();

        let result = create_app_page(temp_dir.path(), "test-app");
        assert!(result.is_ok());

        let content = fs::read_to_string(temp_dir.path().join("src/app/page.tsx")).unwrap();
        assert!(content.contains("Welcome to"));
        assert!(content.contains("test-app"));
        assert!(content.contains("Next.js 15"));
        assert!(content.contains("Tailwind CSS"));
        assert!(content.contains("TypeScript"));
    }

    #[test]
    fn test_create_globals_css() {
        let temp_dir = setup_test_dir();
        // Create src/styles directory
        fs::create_dir_all(temp_dir.path().join("src/styles")).unwrap();

        let result = create_globals_css(temp_dir.path());
        assert!(result.is_ok());

        let content = fs::read_to_string(temp_dir.path().join("src/styles/globals.css")).unwrap();
        assert!(content.contains("@import \"tailwindcss\""));
        assert!(content.contains("--background:"));
        assert!(content.contains("@layer components"));
        assert!(content.contains(".btn-primary"));
    }

    #[test]
    fn test_create_button_component() {
        let temp_dir = setup_test_dir();
        // Create src/components directory
        fs::create_dir_all(temp_dir.path().join("src/components")).unwrap();

        let result = create_button_component(temp_dir.path());
        assert!(result.is_ok());

        let content =
            fs::read_to_string(temp_dir.path().join("src/components/Button.tsx")).unwrap();
        assert!(content.contains("interface ButtonProps"));
        assert!(content.contains("variant?: 'primary'"));
        assert!(content.contains("size?: 'sm'"));
        assert!(content.contains("export const Button"));
    }

    #[test]
    fn test_create_readme() {
        let temp_dir = setup_test_dir();

        let result = create_readme(temp_dir.path(), "sample-project");
        assert!(result.is_ok());

        let content = fs::read_to_string(temp_dir.path().join("README.md")).unwrap();
        assert!(content.contains("# sample-project"));
        assert!(content.contains("Next.js 15"));
        assert!(content.contains("pnpm dev"));
        assert!(content.contains("Project Structure"));
        assert!(content.contains("Tailwind CSS v4"));
    }

    #[test]
    fn test_create_npmrc() {
        let temp_dir = setup_test_dir();

        let result = create_npmrc(temp_dir.path());
        assert!(result.is_ok());

        let content = fs::read_to_string(temp_dir.path().join(".npmrc")).unwrap();
        assert!(content.contains("auto-install-peers=true"));
        assert!(content.contains("strict-peer-dependencies=false"));
    }
}
