# UI Components

Part-DB uses shadcn/ui for its component library. This page documents the available components and how to use them.

## Available Components

The following shadcn/ui components are installed:

| Component | Description |
|-----------|-------------|
| Button | Clickable button with variants |
| Card | Container with header and content |
| Input | Text input field |
| Table | Data table with headers |

## Button

Buttons support multiple variants and sizes.

### Usage

```tsx
import { Button } from "@/components/ui/button";

// Variants
<Button variant="default">Primary</Button>
<Button variant="secondary">Secondary</Button>
<Button variant="destructive">Delete</Button>
<Button variant="outline">Outline</Button>
<Button variant="ghost">Ghost</Button>
<Button variant="link">Link</Button>

// Sizes
<Button size="default">Default</Button>
<Button size="sm">Small</Button>
<Button size="lg">Large</Button>
<Button size="icon">🔍</Button>

// With loading state
<Button disabled>
  <Loader2 className="mr-2 h-4 w-4 animate-spin" />
  Loading
</Button>
```

### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `variant` | string | `"default"` | Visual style |
| `size` | string | `"default"` | Button size |
| `asChild` | boolean | `false` | Render as child component |
| `disabled` | boolean | `false` | Disabled state |

## Card

Cards are containers for related content.

### Usage

```tsx
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
  CardFooter,
} from "@/components/ui/card";

<Card>
  <CardHeader>
    <CardTitle>Part Name</CardTitle>
    <CardDescription>Electronic component</CardDescription>
  </CardHeader>
  <CardContent>
    <p>Quantity: 100</p>
    <p>Location: Drawer A1</p>
  </CardContent>
  <CardFooter>
    <Button>Edit</Button>
  </CardFooter>
</Card>
```

### Components

| Component | Description |
|-----------|-------------|
| `Card` | Container |
| `CardHeader` | Header section |
| `CardTitle` | Title text |
| `CardDescription` | Subtitle/description |
| `CardContent` | Main content area |
| `CardFooter` | Footer actions |

## Input

Text input for forms.

### Usage

```tsx
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";

// Basic input
<Input type="text" placeholder="Enter part name" />

// With label
<div className="grid w-full max-w-sm items-center gap-1.5">
  <Label htmlFor="name">Name</Label>
  <Input type="text" id="name" placeholder="Part name" />
</div>

// Different types
<Input type="email" placeholder="Email" />
<Input type="password" placeholder="Password" />
<Input type="number" placeholder="Quantity" />

// Disabled
<Input disabled placeholder="Disabled" />
```

### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `type` | string | `"text"` | Input type |
| `placeholder` | string | - | Placeholder text |
| `disabled` | boolean | `false` | Disabled state |
| `value` | string | - | Controlled value |
| `onChange` | function | - | Change handler |

## Table

Data tables for lists.

### Usage

```tsx
import {
  Table,
  TableHeader,
  TableBody,
  TableFooter,
  TableHead,
  TableRow,
  TableCell,
  TableCaption,
} from "@/components/ui/table";

<Table>
  <TableCaption>Parts Inventory</TableCaption>
  <TableHeader>
    <TableRow>
      <TableHead>Name</TableHead>
      <TableHead>Category</TableHead>
      <TableHead className="text-right">Quantity</TableHead>
    </TableRow>
  </TableHeader>
  <TableBody>
    {parts.map((part) => (
      <TableRow key={part.id}>
        <TableCell className="font-medium">{part.name}</TableCell>
        <TableCell>{part.category}</TableCell>
        <TableCell className="text-right">{part.quantity}</TableCell>
      </TableRow>
    ))}
  </TableBody>
  <TableFooter>
    <TableRow>
      <TableCell colSpan={2}>Total</TableCell>
      <TableCell className="text-right">{totalQuantity}</TableCell>
    </TableRow>
  </TableFooter>
</Table>
```

## Adding New Components

To add more shadcn/ui components:

```bash
# Add a single component
npx shadcn-ui@latest add dialog

# Add multiple components
npx shadcn-ui@latest add dialog dropdown-menu select

# View all available components
npx shadcn-ui@latest add --help
```

Commonly needed components:

```bash
npx shadcn-ui@latest add dialog          # Modal dialogs
npx shadcn-ui@latest add dropdown-menu   # Dropdown menus
npx shadcn-ui@latest add select          # Select/combobox
npx shadcn-ui@latest add form            # Form validation
npx shadcn-ui@latest add toast           # Toast notifications
npx shadcn-ui@latest add alert           # Alert messages
```

## Utility Functions

The `cn()` utility from `lib/utils.ts` merges Tailwind classes:

```tsx
import { cn } from "@/lib/utils";

// Merge classes conditionally
<div className={cn(
  "px-4 py-2 rounded",
  isActive && "bg-blue-500",
  isDisabled && "opacity-50"
)}>
  Content
</div>
```

## Theming

Components support light and dark themes through CSS variables:

```css
/* globals.css */
:root {
  --primary: 222.2 47.4% 11.2%;
  --primary-foreground: 210 40% 98%;
}

.dark {
  --primary: 210 40% 98%;
  --primary-foreground: 222.2 47.4% 11.2%;
}
```

Toggle dark mode:

```tsx
// Add 'dark' class to html element
document.documentElement.classList.toggle('dark');
```
