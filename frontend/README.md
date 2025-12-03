# Frontend Setup

This directory contains the React frontend for the migrated application. The Main agent
will determine whether to use **Next.js** or **Vite** based on project requirements.

## Running Locally

Install dependencies and start the development server:

```bash
npm install
npm run dev
```

## shadcn/ui Integration

This project uses [shadcn/ui](https://ui.shadcn.com/) for component scaffolding. Initialize
the library and add components using the `npx shadcn-ui@latest` CLI. If the MCP server is
configured, the React agent will invoke it automatically.

```bash
npx shadcn-ui@latest init
npx shadcn-ui@latest add button input card
```

Consult the official documentation for available components and registries.