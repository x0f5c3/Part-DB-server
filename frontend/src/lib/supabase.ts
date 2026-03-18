/**
 * Supabase browser client.
 *
 * This module exports a singleton Supabase client for use in Client Components.
 * Server Components and Route Handlers should use `createServerSupabaseClient`
 * from `@/lib/supabase-server` instead.
 *
 * Required environment variables (set in .env.local):
 *   NEXT_PUBLIC_SUPABASE_URL      – your Supabase project URL
 *   NEXT_PUBLIC_SUPABASE_ANON_KEY – your Supabase project anon/public key
 */

import { createBrowserClient } from "@supabase/ssr";

/**
 * Returns a Supabase client configured for use in browser (client) contexts.
 * Relies on the two public environment variables that are safe to expose to the
 * browser.
 */
export function createSupabaseClient() {
  return createBrowserClient(
    process.env.NEXT_PUBLIC_SUPABASE_URL!,
    process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY!
  );
}

/**
 * Singleton Supabase browser client for use in Client Components.
 *
 * @example
 * import { supabase } from "@/lib/supabase";
 * const { data, error } = await supabase.auth.getSession();
 */
export const supabase = createSupabaseClient();
