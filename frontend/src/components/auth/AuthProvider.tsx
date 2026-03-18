/**
 * AuthProvider – initialises Supabase auth on first render.
 *
 * Wrap the root layout (App.tsx) with this component so every page has
 * access to the latest session via `useAuthStore`.
 */

import { useEffect } from "react";
import { useAuthStore } from "@/store/auth";

interface AuthProviderProps {
  children: React.ReactNode;
}

/**
 * Renders children and, on mount, kicks off the Supabase session check.
 * No visible UI is rendered by this component itself.
 */
export function AuthProvider({ children }: AuthProviderProps) {
  const initialize = useAuthStore((s) => s.initialize);

  useEffect(() => {
    initialize();
  }, [initialize]);

  return <>{children}</>;
}
