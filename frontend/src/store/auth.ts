/**
 * Authentication store using Zustand + Supabase Auth.
 *
 * Manages user authentication state, Supabase session handling, and
 * login/logout flows.  Supports:
 *   • Email + password login
 *   • OAuth SSO providers (Google, GitHub, …)
 *   • Magic-link (passwordless) email login
 *   • Session persistence via Supabase's built-in cookie/localStorage layer
 */

import { create } from "zustand";
import type { Session, User as SupabaseUser, Provider } from "@supabase/supabase-js";
import { supabase } from "@/lib/supabase";

/** Application-level user, mapped from a Supabase auth User object. */
export interface AppUser {
  id: string;
  email: string | null;
  name: string | null;
  avatarUrl: string | null;
}

/** Convert a Supabase `User` into our leaner `AppUser`. */
function toAppUser(user: SupabaseUser): AppUser {
  const meta = user.user_metadata ?? {};
  return {
    id: user.id,
    email: user.email ?? null,
    name: (meta.full_name as string) ?? (meta.name as string) ?? user.email ?? null,
    avatarUrl: (meta.avatar_url as string) ?? null,
  };
}

interface AuthState {
  /** The current application user (null if not authenticated). */
  user: AppUser | null;
  /** The raw Supabase session (contains access_token, refresh_token, …). */
  session: Session | null;
  /** Whether the user is currently authenticated. */
  isAuthenticated: boolean;
  /** Whether the initial auth check is still in progress. */
  isLoading: boolean;
  /** Last auth error, if any. */
  error: string | null;

  /** Initialise auth state from the current Supabase session.
   *  Returns a cleanup function that unsubscribes the auth listener. */
  initialize: () => Promise<() => void>;

  /** Sign in with email + password. */
  signInWithPassword: (email: string, password: string) => Promise<void>;

  /** Sign in via an OAuth provider (Google, GitHub, …). */
  signInWithOAuth: (provider: Provider) => Promise<void>;

  /** Send a magic-link (passwordless) sign-in email. */
  signInWithMagicLink: (email: string) => Promise<void>;

  /** Sign out and clear session. */
  signOut: () => Promise<void>;

  /** Internal: update session + user derived state. */
  _setSession: (session: Session | null) => void;
}

export const useAuthStore = create<AuthState>()((set) => ({
  user: null,
  session: null,
  isAuthenticated: false,
  isLoading: true,
  error: null,

  initialize: async () => {
    set({ isLoading: true, error: null });
    try {
      const { data: { session } } = await supabase.auth.getSession();
      set({
        session,
        user: session?.user ? toAppUser(session.user) : null,
        isAuthenticated: !!session,
        isLoading: false,
      });

      // Subscribe to future auth state changes; return unsubscribe for cleanup
      const { data: { subscription } } = supabase.auth.onAuthStateChange((_event, session) => {
        set({
          session,
          user: session?.user ? toAppUser(session.user) : null,
          isAuthenticated: !!session,
          isLoading: false,
        });
      });
      return () => subscription.unsubscribe();
    } catch (err) {
      set({
        error: err instanceof Error ? err.message : "Failed to initialise auth",
        isLoading: false,
      });
      return () => {};
    }
  },

  signInWithPassword: async (email, password) => {
    set({ isLoading: true, error: null });
    const { data, error } = await supabase.auth.signInWithPassword({ email, password });
    if (error) {
      set({ error: error.message, isLoading: false });
      throw error;
    }
    set({
      session: data.session,
      user: data.user ? toAppUser(data.user) : null,
      isAuthenticated: !!data.session,
      isLoading: false,
    });
  },

  signInWithOAuth: async (provider) => {
    set({ isLoading: true, error: null });
    const { error } = await supabase.auth.signInWithOAuth({
      provider,
      options: {
        redirectTo: `${window.location.origin}/auth/callback`,
      },
    });
    if (error) {
      set({ error: error.message, isLoading: false });
      throw error;
    }
    // Browser will be redirected; loading state stays true until callback
  },

  signInWithMagicLink: async (email) => {
    set({ isLoading: true, error: null });
    const { error } = await supabase.auth.signInWithOtp({
      email,
      options: {
        emailRedirectTo: `${window.location.origin}/auth/callback`,
      },
    });
    if (error) {
      set({ error: error.message, isLoading: false });
      throw error;
    }
    set({ isLoading: false });
  },

  signOut: async () => {
    await supabase.auth.signOut();
    set({ user: null, session: null, isAuthenticated: false, isLoading: false });
  },

  _setSession: (session) => {
    set({
      session,
      user: session?.user ? toAppUser(session.user) : null,
      isAuthenticated: !!session,
      isLoading: false,
    });
  },
}));

