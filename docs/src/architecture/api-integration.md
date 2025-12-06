# API Integration

This page documents how the frontend communicates with the backend API.

## API Configuration

The frontend proxies API requests to the backend via Next.js rewrites:

```javascript
// next.config.js
module.exports = {
  async rewrites() {
    return [
      {
        source: '/api/:path*',
        destination: 'http://localhost:3000/api/:path*',
      },
    ];
  },
};
```

In production, configure the backend URL via environment variable:

```env
NEXT_PUBLIC_API_URL=https://api.example.com
```

## Type Definitions

API types are defined in `src/types/api.ts`:

```typescript
// types/api.ts

export interface Part {
  id: number;
  name: string;
  description: string | null;
  category_id: number | null;
  footprint_id: number | null;
  manufacturer_id: number | null;
  storage_location_id: number | null;
  quantity: number;
  minimum_quantity: number;
  created_at: string;
  updated_at: string;
}

export interface Category {
  id: number;
  name: string;
  description: string | null;
  parent_id: number | null;
  created_at: string;
  updated_at: string;
}

export interface PaginatedResponse<T> {
  items: T[];
  total: number;
  page: number;
  per_page: number;
  total_pages: number;
}

export interface ErrorResponse {
  message: string;
  code: string;
}

export interface LoginRequest {
  username: string;
  password: string;
}

export interface LoginResponse {
  token: string;
  user: User;
}

export interface User {
  id: number;
  username: string;
  email: string | null;
  is_admin: boolean;
}
```

## API Client

Create a reusable API client:

```typescript
// lib/api.ts

const API_BASE = process.env.NEXT_PUBLIC_API_URL || '';

interface FetchOptions extends RequestInit {
  token?: string;
}

export async function apiFetch<T>(
  endpoint: string,
  options: FetchOptions = {}
): Promise<T> {
  const { token, ...fetchOptions } = options;
  
  const headers: HeadersInit = {
    'Content-Type': 'application/json',
    ...options.headers,
  };
  
  if (token) {
    headers['Authorization'] = `Bearer ${token}`;
  }

  const response = await fetch(`${API_BASE}${endpoint}`, {
    ...fetchOptions,
    headers,
  });

  if (!response.ok) {
    const error = await response.json();
    throw new Error(error.message || 'API request failed');
  }

  return response.json();
}

// Typed API methods
export const api = {
  // Parts
  async listParts(page = 1, perPage = 30, token?: string) {
    return apiFetch<PaginatedResponse<Part>>(
      `/api/parts?page=${page}&per_page=${perPage}`,
      { token }
    );
  },

  async getPart(id: number, token?: string) {
    return apiFetch<Part>(`/api/parts/${id}`, { token });
  },

  async createPart(data: Partial<Part>, token?: string) {
    return apiFetch<Part>('/api/parts', {
      method: 'POST',
      body: JSON.stringify(data),
      token,
    });
  },

  async updatePart(id: number, data: Partial<Part>, token?: string) {
    return apiFetch<Part>(`/api/parts/${id}`, {
      method: 'PATCH',
      body: JSON.stringify(data),
      token,
    });
  },

  async deletePart(id: number, token?: string) {
    return apiFetch<void>(`/api/parts/${id}`, {
      method: 'DELETE',
      token,
    });
  },

  // Authentication
  async login(username: string, password: string) {
    return apiFetch<LoginResponse>('/api/auth/login', {
      method: 'POST',
      body: JSON.stringify({ username, password }),
    });
  },

  async getCurrentUser(token: string) {
    return apiFetch<User>('/api/auth/me', { token });
  },
};
```

## Usage Examples

### Fetching Data

```tsx
import { api } from '@/lib/api';
import { useAuth } from '@/store/auth-context';

export function PartsList() {
  const { token } = useAuth();
  const [parts, setParts] = useState<Part[]>([]);

  useEffect(() => {
    async function load() {
      const data = await api.listParts(1, 30, token);
      setParts(data.items);
    }
    load();
  }, [token]);

  return (
    <ul>
      {parts.map(part => (
        <li key={part.id}>{part.name}</li>
      ))}
    </ul>
  );
}
```

### Creating Data

```tsx
import { api } from '@/lib/api';

export function CreatePartForm() {
  const { token } = useAuth();

  async function handleSubmit(data: { name: string; description: string }) {
    try {
      const newPart = await api.createPart(data, token);
      console.log('Created part:', newPart.id);
    } catch (error) {
      console.error('Failed to create part:', error);
    }
  }

  // ...
}
```

### Updating Data

```tsx
async function handleUpdate(id: number, updates: Partial<Part>) {
  try {
    const updated = await api.updatePart(id, updates, token);
    console.log('Updated part:', updated);
  } catch (error) {
    console.error('Failed to update part:', error);
  }
}
```

### Deleting Data

```tsx
async function handleDelete(id: number) {
  if (!confirm('Are you sure?')) return;
  
  try {
    await api.deletePart(id, token);
    // Remove from local state
    setParts(parts.filter(p => p.id !== id));
  } catch (error) {
    console.error('Failed to delete part:', error);
  }
}
```

## Error Handling

Handle API errors gracefully:

```tsx
import { useState } from 'react';
import { api } from '@/lib/api';

export function PartsList() {
  const [error, setError] = useState<string | null>(null);

  async function fetchParts() {
    try {
      setError(null);
      const data = await api.listParts();
      setParts(data.items);
    } catch (err) {
      if (err instanceof Error) {
        setError(err.message);
      } else {
        setError('An unexpected error occurred');
      }
    }
  }

  if (error) {
    return (
      <div className="text-red-500">
        Error: {error}
        <button onClick={fetchParts}>Retry</button>
      </div>
    );
  }

  // ...
}
```

## Request Cancellation

Cancel pending requests when components unmount:

```tsx
useEffect(() => {
  const controller = new AbortController();

  async function fetchParts() {
    try {
      const response = await fetch('/api/parts', {
        signal: controller.signal,
      });
      const data = await response.json();
      setParts(data.items);
    } catch (err) {
      if (err.name !== 'AbortError') {
        setError(err.message);
      }
    }
  }

  fetchParts();

  return () => controller.abort();
}, []);
```

## Authentication Headers

Include JWT token in authenticated requests:

```tsx
const { token } = useAuth();

const response = await fetch('/api/parts', {
  headers: {
    'Authorization': `Bearer ${token}`,
    'Content-Type': 'application/json',
  },
});
```
