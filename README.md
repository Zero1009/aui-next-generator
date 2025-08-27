# AUI Next.js Generator

A fast CLI tool to generate Next.js projects with modern tooling and best practices.

## Features

- ⚡ **Next.js 15** with App Router
- 🎨 **Tailwind CSS v4** with CSS-based configuration  
- 📝 **TypeScript** for type safety
- 🔧 **ESLint** with Next.js optimized rules
- 🚀 **Turbopack** support for faster development
- 📦 **pnpm** for efficient package management
- 🏗️ **Organized project structure** with common directories
- ✨ **Interactive prompts** for configuration options

## Installation

### Via Cargo (Recommended)

```bash
cargo install aui-next-generator
```

### From Source

```bash
git clone https://github.com/yourusername/aui-next-generator
cd aui-next-generator
cargo install --path .
```

## Usage

### Basic Usage

```bash
# Interactive mode - prompts for project name
aui-next-gen

# Specify project name directly
aui-next-gen my-awesome-app
```

### Options

```bash
# Skip dependency installation
aui-next-gen my-app --skip-install
```

### What Gets Generated

```
my-project/
├── src/
│   ├── app/               # Next.js App Router
│   │   ├── layout.tsx     # Root layout
│   │   └── page.tsx       # Home page
│   ├── components/        # Reusable UI components
│   │   └── Button.tsx     # Example component
│   ├── constants/         # Application constants
│   ├── hooks/             # Custom React hooks
│   ├── libs/              # Utility libraries
│   ├── assets/            # Static assets
│   ├── types/             # TypeScript type definitions
│   ├── fonts/             # Custom fonts
│   └── styles/            # Global styles
│       └── globals.css    # Tailwind imports & custom styles
├── public/                # Static files
├── next.config.ts         # Next.js configuration
├── tsconfig.json          # TypeScript configuration
├── postcss.config.mjs     # PostCSS configuration
├── .eslintrc.json         # ESLint rules
├── .gitignore             # Git ignore rules
├── .npmrc                 # pnpm configuration
└── package.json           # Project dependencies & scripts
```

## Requirements

- **Node.js**: 18.18.0 or higher
- **pnpm**: Automatically installed if not present

The CLI validates your Node.js version and can automatically install pnpm if needed.

## Interactive Setup

The generator provides interactive prompts for:

1. **Project Name**: Enter your project name
2. **Dependencies**: Choose whether to install dependencies automatically  
3. **Turbopack**: Enable faster development builds with Turbopack

## Generated Stack

- **Next.js 15**: Latest version with App Router
- **React 19**: Latest React with new features
- **TypeScript 5**: Full type safety
- **Tailwind CSS v4**: Utility-first CSS framework
- **ESLint 9**: Code linting with Next.js rules
- **Inter Font**: Loaded via `next/font/google`

## Scripts

Generated projects include these npm scripts:

```json
{
  "dev": "next dev",           // or "next dev --turbo" if enabled
  "build": "next build",
  "start": "next start", 
  "lint": "next lint",
  "lint:fix": "next lint --fix"
}
```

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.