/**
 * Authentication store using Zustand.
 *
 * Manages user authentication state, token storage, and login/logout flows.
 */

import { create } from "zustand";
import { persist, createJSONStorage } from "zustand/middleware";
import type { User } from "@/types/api";

interface AuthState {
  /** The current user (null if not authenticated) */
  user: User | null;
  /** The JWT access token */
  token: string | null;
  /** Whether the user is currently authenticated */
  isAuthenticated: boolean;
  /** Whether authentication is being loaded/checked */
  isLoading: boolean;
  /** Set the current user */
  setUser: (user: User | null) => void;
  /** Set the JWT token */
  setToken: (token: string | null) => void;
  /** Login with user and token */
  login: (user: User, token: string) => void;
  /** Logout and clear state */
  logout: () => void;
  /** Set loading state */
  setLoading: (loading: boolean) => void;
}

/**
 * Auth store with persistence to localStorage.
 */
export const useAuthStore = create<AuthState>()(
  persist(
    (set) => ({
      user: null,
      token: null,
      isAuthenticated: false,
      isLoading: true,

      setUser: (user) =>
        set({
          user,
          isAuthenticated: user !== null,
        }),

      setToken: (token) =>
        set({
          token,
        }),

      login: (user, token) =>
        set({
          user,
          token,
          isAuthenticated: true,
          isLoading: false,
        }),

      logout: () =>
        set({
          user: null,
          token: null,
          isAuthenticated: false,
          isLoading: false,
        }),

      setLoading: (loading) =>
        set({
          isLoading: loading,
        }),
    }),
    {
      name: "partdb-auth",
      storage: createJSONStorage(() => localStorage),
      partialize: (state) => ({
        token: state.token,
        // Don't persist user data - fetch fresh on app load
      }),
    }
  )
);
