# Herblytics

![CI Quality](https://img.shields.io/badge/Status-Active-success?style=flat-square)
[![Vue](https://img.shields.io/badge/Vue-3.5+-4FC08D?logo=vue.js)](https://vuejs.org/)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.9+-3178C6?logo=typescript)](https://www.typescriptlang.org/)
[![Vite](https://img.shields.io/badge/Vite-7.0+-646CFF?logo=vite)](https://vitejs.dev/)
[![Tailwind CSS](https://img.shields.io/badge/Tailwind_CSS-4.1+-06B6D4?logo=tailwindcss)](https://tailwindcss.com/)
[![Bun](https://img.shields.io/badge/Bun-1.0+-000000?logo=bun)](https://bun.sh/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

> **A data visualization dashboard for tracking and analyzing the procurement value of herbal medicines at Sabot Hospital.**

This application provides actionable insights into purchasing trends, helping hospital staff and administrators identify key metrics such as total annual purchase value and top-performing herbal products.

---

## Features

### Data Visualization

- **Interactive Charts**: Powered by **[Chart.js](https://www.chartjs.org/)**, visualize the top 10 herbal medicines by total purchase value.
- **Dynamic Summaries**: Real-time summary cards displaying high-level metrics for the selected fiscal year.

### Core Stack

- **[Vue 3.5+](https://vuejs.org/)**: Utilizing the Composition API with `<script setup>` for concise and performant components.
- **[TypeScript 5.9+](https://www.typescriptlang.org/)**: Configured for type safety and developer productivity.
- **[Vite 7](https://vitejs.dev/)**: Next-generation frontend tooling with instant server start and lightning-fast HMR.
- **[Tailwind CSS 4.1](https://tailwindcss.com/)**: Utility-first CSS framework for rapid UI development.

### Architecture

- **[Pinia](https://pinia.vuejs.org/)**: Intuitive, type-safe state management for handling data fetching and filtering logic.
- **Google Sheets Backend**: A lightweight, maintenance-free backend using **Google Apps Script** to serve data directly from Google Sheets.
- **[Zod](https://zod.dev/)**: Schema declaration and validation to ensure data integrity from the API.

---

## IDE Setup

For the best experience, we recommend **[VS Code](https://code.visualstudio.com/)** with the following configuration:

1.  **Install Extensions**:
    - [Vue - Official](https://marketplace.visualstudio.com/items?itemName=Vue.volar) (formerly Volar)
    - [Tailwind CSS IntelliSense](https://marketplace.visualstudio.com/items?itemName=bradlc.vscode-tailwindcss)
    - [ESLint](https://marketplace.visualstudio.com/items?itemName=dbaeumer.vscode-eslint)

2.  **Workspace Settings**:
    Ensure your editor is configured to auto-fix lint errors on save and use the workspace TypeScript version.

---

## Prerequisites

Ensure your environment meets the following requirements:

| Requirement | Version | Note                                          |
| :---------- | :------ | :-------------------------------------------- |
| **Node.js** | `18+`   | Required for modern build tools.              |
| **Bun**     | `1.0+`  | Preferred package manager (lockfile present). |

---

## Getting Started

### 1. Clone the repository

```bash
git clone https://github.com/suradet-ps/herb-lytics.git
cd herb-lytics
```

### 2. Install dependencies

```bash
bun install
```

### 3. Start development server

```bash
bun dev
```

The application will be available at `http://localhost:5173/`.

> **Note:** No environment variables are required for local development as the Google Apps Script API endpoint is public.

---

## Available Scripts

| Script         | Description                               |
| :------------- | :---------------------------------------- |
| `bun dev`      | Start the development server with HMR.    |
| `bun build`    | Run type-checks and build for production. |
| `bun preview`  | Preview the production build locally.     |
| `bun lint`     | Lint and format all files.                |
| `bun lint:fix` | Auto-fix linting and formatting issues.   |

---

## Project Structure

```text
.
├── .vscode/             # VS Code workspace settings
├── src/
│   ├── assets/          # Static assets
│   ├── components/      # Reusable UI components
│   ├── composables/     # Shared logic (Vue Composables)
│   ├── router/          # Routing configuration
│   ├── stores/          # Global state management (Pinia)
│   ├── types/           # TypeScript type definitions
│   ├── views/           # Page-level components
│   ├── App.vue          # Root component
│   └── main.ts          # Application entry point
├── eslint.config.mjs    # ESLint configuration
├── tailwind.config.js   # Tailwind CSS configuration (if applicable)
├── tsconfig.json        # TypeScript configuration
└── vite.config.ts       # Vite configuration
```

---

## Deployment

### Static Hosting (Vercel, Netlify, etc.)

This project builds a static SPA.

1. **Build Command**: `bun build`
2. **Output Directory**: `dist`
3. **Node Version**: `18.x` or higher

---

## Contributing

1.  **Fork** the repository.
2.  **Create** a feature branch: `git checkout -b feat/my-feature`.
3.  **Commit** your changes: `git commit -m "feat: add amazing feature"`.
4.  **Push** to the branch: `git push origin feat/my-feature`.
5.  **Open** a Pull Request.

---

## License

This project is licensed under the [MIT License](LICENSE).
