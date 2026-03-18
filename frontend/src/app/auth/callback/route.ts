/**
 * OAuth / Magic-link callback route handler.
 *
 * Supabase redirects back to this URL after an OAuth sign-in or when the user
 * clicks a magic-link email.  The URL contains an authorization `code` that
 * this handler exchanges for a session, then redirects the user to the
 * originally requested page (or the app root).
 *
 * @see https://supabase.com/docs/guides/auth/server-side/nextjs
 */

import { NextResponse } from "next/server";
import { createServerSupabaseClient } from "@/lib/supabase-server";

export async function GET(request: Request) {
  const { searchParams, origin } = new URL(request.url);
  const code = searchParams.get("code");
  // `next` is an optional redirect target set by ProtectedRoute / login page.
  const next = searchParams.get("next") ?? "/";

  if (code) {
    const supabase = createServerSupabaseClient();
    const { error } = await supabase.auth.exchangeCodeForSession(code);

    if (!error) {
      // Redirect to the original destination after successful auth.
      return NextResponse.redirect(`${origin}${next}`);
    }
  }

  // Something went wrong – send the user back to the login page with an error.
  return NextResponse.redirect(`${origin}/login?error=auth_callback_failed`);
}
