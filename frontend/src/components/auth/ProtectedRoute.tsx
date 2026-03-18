/**
 * ProtectedRoute – redirects unauthenticated users to the login page.
 *
 * @example
 * import { ProtectedRoute } from "@/components/auth/ProtectedRoute";
 *
 * export default function DashboardPage() {
 *   return (
 *     <ProtectedRoute>
 *       <YourContent />
 *     </ProtectedRoute>
 *   );
 * }
 */

import { useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { useAuthStore } from "@/store/auth";

interface ProtectedRouteProps {
  children: React.ReactNode;
  /** Route to redirect to when unauthenticated (default: "/login"). */
  redirectTo?: string;
}

export function ProtectedRoute({
  children,
  redirectTo = "/login",
}: ProtectedRouteProps) {
  const navigate = useNavigate();
  const { isAuthenticated, isLoading } = useAuthStore();

  useEffect(() => {
    if (!isLoading && !isAuthenticated) {
      navigate(redirectTo, { replace: true });
    }
  }, [isAuthenticated, isLoading, redirectTo, navigate]);

  if (isLoading) {
    return (
      <div className="flex min-h-screen items-center justify-center">
        <div className="text-muted-foreground animate-pulse">Loading…</div>
      </div>
    );
  }

  if (!isAuthenticated) {
    return null;
  }

  return <>{children}</>;
}
