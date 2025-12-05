# Frontend Architecture

The Part-DB frontend is built with React using Next.js 14, Tailwind CSS, and shadcn/ui components.

## Overview

```
┌─────────────────────────────────────────────────┐
│                    Next.js                       │
│  ┌───────────┐  ┌───────────┐  ┌───────────┐   │
│  │   Pages   │  │   Layout  │  │   API     │   │
│  │ (App Dir) │  │           │  │  Routes   │   │
│  └─────┬─────┘  └─────┬─────┘  └─────┬─────┘   │
│        │              │              │          │
│  ┌─────▼──────────────▼──────────────▼─────┐   │
│  │              Components                  │   │
│  │  ┌─────────┐  ┌─────────┐  ┌─────────┐  │   │
│  │  │shadcn/ui│  │  Custom │  │ Layouts │  │   │
│  │  └─────────┘  └─────────┘  └─────────┘  │   │
│  └──────────────────┬──────────────────────┘   │
│                     │                           │
│  ┌──────────────────▼──────────────────────┐   │
│  │            State & API                   │   │
│  │  ┌─────────┐  ┌─────────┐  ┌─────────┐  │   │
│  │  │  Store  │  │  Types  │  │   Lib   │  │   │
│  │  └─────────┘  └─────────┘  └─────────┘  │   │
│  └─────────────────────────────────────────┘   │
└─────────────────────────────────────────────────┘
```

## Tech Stack

| Technology | Purpose |
|------------|---------|
| **Next.js 14** | React framework with App Router |
| **TypeScript** | Type-safe JavaScript |
| **Tailwind CSS** | Utility-first CSS framework |
| **shadcn/ui** | High-quality UI components |
| **Fetch API** | HTTP requests to backend |

## Directory Structure

```
frontend/
├── src/
│   ├── app/                 # Next.js App Router
│   │   ├── page.tsx         # Homepage
│   │   ├── layout.tsx       # Root layout
│   │   ├── globals.css      # Global styles
│   │   ├── parts/           # Parts pages
│   │   │   ├── page.tsx     # Parts list
│   │   │   └── [id]/        # Part detail
│   │   ├── categories/      # Categories pages
│   │   └── storage/         # Storage pages
│   │
│   ├── components/
│   │   └── ui/              # shadcn/ui components
│   │       ├── button.tsx
│   │       ├── card.tsx
│   │       ├── input.tsx
│   │       └── table.tsx
│   │
│   ├── lib/
│   │   └── utils.ts         # Utility functions (cn, etc.)
│   │
│   ├── store/               # State management
│   │
│   └── types/
│       └── api.ts           # API type definitions
│
├── package.json
├── next.config.js           # Next.js configuration
├── tailwind.config.ts       # Tailwind configuration
├── tsconfig.json            # TypeScript configuration
└── postcss.config.js        # PostCSS configuration
```

## App Router

The frontend uses Next.js 14's App Router for file-based routing:

| Path | File | Description |
|------|------|-------------|
| `/` | `app/page.tsx` | Homepage/Dashboard |
| `/parts` | `app/parts/page.tsx` | Parts list |
| `/parts/[id]` | `app/parts/[id]/page.tsx` | Part detail |
| `/categories` | `app/categories/page.tsx` | Categories list |
| `/storage` | `app/storage/page.tsx` | Storage locations |

## Styling

### Tailwind CSS

Tailwind is configured with custom colors and dark mode:

```typescript
// tailwind.config.ts
import type { Config } from "tailwindcss";

const config: Config = {
  darkMode: ["class"],
  content: ["./src/**/*.{ts,tsx}"],
  theme: {
    extend: {
      colors: {
        border: "hsl(var(--border))",
        background: "hsl(var(--background))",
        foreground: "hsl(var(--foreground))",
        primary: {
          DEFAULT: "hsl(var(--primary))",
          foreground: "hsl(var(--primary-foreground))",
        },
        // ... more colors
      },
    },
  },
  plugins: [require("tailwindcss-animate")],
};

export default config;
```

### Global Styles

CSS variables for theming in `globals.css`:

```css
@tailwind base;
@tailwind components;
@tailwind utilities;

@layer base {
  :root {
    --background: 0 0% 100%;
    --foreground: 222.2 84% 4.9%;
    --primary: 222.2 47.4% 11.2%;
    /* ... */
  }
  
  .dark {
    --background: 222.2 84% 4.9%;
    --foreground: 210 40% 98%;
    /* ... */
  }
}
```

## Components

### shadcn/ui Components

Pre-built, accessible components:

```tsx
import { Button } from "@/components/ui/button";
import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Table, TableHeader, TableRow, TableHead, TableBody, TableCell } from "@/components/ui/table";
```

Adding new components:

```bash
npx shadcn-ui@latest add [component-name]
```

### Custom Components

Build on top of shadcn/ui:

```tsx
// components/part-card.tsx
import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/card";
import { Part } from "@/types/api";

interface PartCardProps {
  part: Part;
}

export function PartCard({ part }: PartCardProps) {
  return (
    <Card>
      <CardHeader>
        <CardTitle>{part.name}</CardTitle>
      </CardHeader>
      <CardContent>
        <p>{part.description}</p>
        <p>Quantity: {part.quantity}</p>
      </CardContent>
    </Card>
  );
}
```

## Configuration Files

### next.config.js

```javascript
/** @type {import('next').NextConfig} */
const nextConfig = {
  async rewrites() {
    return [
      {
        source: '/api/:path*',
        destination: 'http://localhost:3000/api/:path*',
      },
    ];
  },
};

module.exports = nextConfig;
```

### tsconfig.json

```json
{
  "compilerOptions": {
    "target": "es5",
    "lib": ["dom", "dom.iterable", "esnext"],
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
    "plugins": [{ "name": "next" }],
    "paths": {
      "@/*": ["./src/*"]
    }
  }
}
```

## Development

```bash
# Install dependencies
npm install

# Development server
npm run dev

# Build for production
npm run build

# Start production server
npm start
```

See [Components](./components.md) for component documentation.
