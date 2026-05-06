import { useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { supabase } from "@/lib/supabase";

/**
 * OAuth / Magic-link callback page.
 *
 * Supabase redirects the browser here after an OAuth sign-in or magic-link
 * click.  For PKCE flows the URL carries a `code` parameter that we exchange
 * for a session.  For implicit grant flows Supabase's `onAuthStateChange`
 * listener (set up in AuthProvider) picks up the hash tokens automatically.
 */
export default function AuthCallback() {
  const navigate = useNavigate();

  useEffect(() => {
    const handleCallback = async () => {
      const params = new URLSearchParams(window.location.search);
      const code = params.get("code");
      const next = params.get("next") ?? "/";

      if (code) {
        const { error } = await supabase.auth.exchangeCodeForSession(code);
        if (!error) {
          navigate(next, { replace: true });
          return;
        }
      }

      // No code present – could be an implicit grant (hash-based) or an error.
      // Hash-based sessions are handled automatically by supabase-js; if the
      // session was set the AuthProvider will pick it up.  Either way redirect
      // home and let the auth store sort it out.
      navigate("/", { replace: true });
    };

    handleCallback();
  }, [navigate]);

  return (
    <div className="flex min-h-screen items-center justify-center">
      <p className="text-muted-foreground animate-pulse">Signing in…</p>
    </div>
  );
}
