"use client";

/**
 * ProtectedRoute – redirects unauthenticated users to the login page.
 *
 * @example
 * // In a page component:
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
import { useRouter } from "next/navigation";
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
  const router = useRouter();
  const { isAuthenticated, isLoading } = useAuthStore();

  useEffect(() => {
    if (!isLoading && !isAuthenticated) {
      router.replace(redirectTo);
    }
  }, [isAuthenticated, isLoading, redirectTo, router]);

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
