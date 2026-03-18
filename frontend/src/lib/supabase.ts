/**
 * Supabase browser client.
 *
 * Exports a singleton Supabase client for use throughout the application.
 *
 * Required environment variables (set in .env.local):
 *   VITE_SUPABASE_URL      – your Supabase project URL
 *   VITE_SUPABASE_ANON_KEY – your Supabase project anon/public key
 */

import { createClient } from "@supabase/supabase-js";

/**
 * Returns a new Supabase client configured with the Vite env vars.
 */
export function createSupabaseClient() {
  return createClient(
    import.meta.env.VITE_SUPABASE_URL as string,
    import.meta.env.VITE_SUPABASE_ANON_KEY as string
  );
}

/**
 * Singleton Supabase client for use across the app.
 *
 * @example
 * import { supabase } from "@/lib/supabase";
 * const { data, error } = await supabase.auth.getSession();
 */
export const supabase = createSupabaseClient();
