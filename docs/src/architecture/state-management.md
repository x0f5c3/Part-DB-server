# State Management

Part-DB uses a simple approach to state management suitable for a CRUD application.

## Overview

The frontend uses:

- **React State** - Component-level state with `useState`
- **Server State** - Data fetching with `fetch` API
- **URL State** - Routing parameters for navigation

## Component State

For local UI state, use React's `useState`:

```tsx
import { useState } from 'react';

export function PartsList() {
  const [searchQuery, setSearchQuery] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [parts, setParts] = useState<Part[]>([]);

  // ...
}
```

## Server State

Data from the API is fetched and managed in components:

```tsx
import { useEffect, useState } from 'react';
import { Part } from '@/types/api';

export function PartsList() {
  const [parts, setParts] = useState<Part[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    async function fetchParts() {
      try {
        const response = await fetch('/api/parts');
        if (!response.ok) throw new Error('Failed to fetch parts');
        const data = await response.json();
        setParts(data.items);
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Unknown error');
      } finally {
        setIsLoading(false);
      }
    }

    fetchParts();
  }, []);

  if (isLoading) return <div>Loading...</div>;
  if (error) return <div>Error: {error}</div>;

  return (
    <ul>
      {parts.map(part => (
        <li key={part.id}>{part.name}</li>
      ))}
    </ul>
  );
}
```

## Custom Hooks

Extract reusable data fetching logic into hooks:

```tsx
// hooks/use-parts.ts
import { useState, useEffect } from 'react';
import { Part, PaginatedResponse } from '@/types/api';

interface UsePartsOptions {
  page?: number;
  perPage?: number;
}

export function useParts(options: UsePartsOptions = {}) {
  const { page = 1, perPage = 30 } = options;
  const [data, setData] = useState<PaginatedResponse<Part> | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  useEffect(() => {
    const controller = new AbortController();

    async function fetchParts() {
      try {
        setIsLoading(true);
        const response = await fetch(
          `/api/parts?page=${page}&per_page=${perPage}`,
          { signal: controller.signal }
        );
        if (!response.ok) throw new Error('Failed to fetch parts');
        const data = await response.json();
        setData(data);
      } catch (err) {
        if (err instanceof Error && err.name !== 'AbortError') {
          setError(err);
        }
      } finally {
        setIsLoading(false);
      }
    }

    fetchParts();

    return () => controller.abort();
  }, [page, perPage]);

  return { data, isLoading, error };
}
```

Usage:

```tsx
export function PartsPage() {
  const [page, setPage] = useState(1);
  const { data, isLoading, error } = useParts({ page });

  // ...
}
```

## Authentication State

Store authentication state in a React Context:

```tsx
// store/auth-context.tsx
import { createContext, useContext, useState, ReactNode } from 'react';

interface User {
  id: number;
  username: string;
  email: string;
  is_admin: boolean;
}

interface AuthContextType {
  user: User | null;
  token: string | null;
  login: (username: string, password: string) => Promise<void>;
  logout: () => void;
  isAuthenticated: boolean;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export function AuthProvider({ children }: { children: ReactNode }) {
  const [user, setUser] = useState<User | null>(null);
  const [token, setToken] = useState<string | null>(null);

  async function login(username: string, password: string) {
    const response = await fetch('/api/auth/login', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ username, password }),
    });

    if (!response.ok) throw new Error('Login failed');

    const data = await response.json();
    setUser(data.user);
    setToken(data.token);
    localStorage.setItem('token', data.token);
  }

  function logout() {
    setUser(null);
    setToken(null);
    localStorage.removeItem('token');
  }

  return (
    <AuthContext.Provider value={{
      user,
      token,
      login,
      logout,
      isAuthenticated: !!token,
    }}>
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth() {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error('useAuth must be used within AuthProvider');
  }
  return context;
}
```

## URL State

Use Next.js routing for URL-based state:

```tsx
'use client';

import { useRouter, useSearchParams } from 'next/navigation';

export function PartsList() {
  const router = useRouter();
  const searchParams = useSearchParams();
  const page = Number(searchParams.get('page')) || 1;

  function goToPage(newPage: number) {
    router.push(`/parts?page=${newPage}`);
  }

  // ...
}
```

## Form State

For forms, use controlled components:

```tsx
import { useState } from 'react';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';

export function CreatePartForm() {
  const [name, setName] = useState('');
  const [description, setDescription] = useState('');
  const [isSubmitting, setIsSubmitting] = useState(false);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setIsSubmitting(true);

    try {
      await fetch('/api/parts', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ name, description }),
      });
      // Reset form or redirect
      setName('');
      setDescription('');
    } catch (error) {
      console.error('Failed to create part:', error);
    } finally {
      setIsSubmitting(false);
    }
  }

  return (
    <form onSubmit={handleSubmit}>
      <Input
        value={name}
        onChange={(e) => setName(e.target.value)}
        placeholder="Part name"
        required
      />
      <Input
        value={description}
        onChange={(e) => setDescription(e.target.value)}
        placeholder="Description"
      />
      <Button type="submit" disabled={isSubmitting}>
        {isSubmitting ? 'Creating...' : 'Create Part'}
      </Button>
    </form>
  );
}
```

## Best Practices

1. **Keep state local** - Only lift state when needed by multiple components
2. **Fetch on mount** - Load data in `useEffect` with cleanup
3. **Handle loading/error states** - Always show loading and error UI
4. **Use TypeScript** - Define types for all state
5. **Abort requests** - Clean up fetch requests when components unmount
