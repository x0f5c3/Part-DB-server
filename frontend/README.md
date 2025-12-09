# Part-DB Frontend

This directory contains the React frontend for the migrated Part-DB application.
Built with **Next.js 14**, **Tailwind CSS**, and **shadcn/ui** components.

## Tech Stack

- **Next.js 14** - React framework with App Router
- **TypeScript** - Type-safe development
- **Tailwind CSS** - Utility-first styling
- **shadcn/ui** - High-quality UI component library

## Getting Started

### Prerequisites

- Node.js 18+ 
- npm or yarn

### Installation

```bash
npm install
```

### Development

```bash
npm run dev
```

The frontend will start on `http://localhost:3001` by default (Next.js dev server).
API requests are proxied to the Rust backend at `http://localhost:3000`.

### Production Build

```bash
npm run build
npm start
```

## Project Structure

```
frontend/
├── src/
│   ├── app/                 # Next.js App Router pages
│   │   ├── page.tsx         # Homepage
│   │   ├── layout.tsx       # Root layout
│   │   ├── globals.css      # Global styles
│   │   ├── parts/           # Parts pages
│   │   ├── categories/      # Categories pages
│   │   └── storage/         # Storage pages
│   ├── components/
│   │   └── ui/              # shadcn/ui components
│   ├── lib/
│   │   └── utils.ts         # Utility functions
│   └── types/
│       └── api.ts           # API type definitions
├── package.json
├── tailwind.config.ts
├── tsconfig.json
└── next.config.js
```

## shadcn/ui Components

Available components:
- Button
- Card
- Input
- Table

To add more components:

```bash
npx shadcn-ui@latest add [component-name]
```

## API Integration

The frontend communicates with the Rust backend through `/api/*` endpoints.
During development, requests are proxied via Next.js rewrites (see `next.config.js`).

Type definitions for API responses are in `src/types/api.ts`.