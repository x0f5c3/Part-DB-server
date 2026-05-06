import { Link } from "react-router-dom";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { useAuthStore } from "@/store/auth";

export default function Home() {
  const { isAuthenticated, user, signOut } = useAuthStore();

  return (
    <main className="min-h-screen bg-gradient-to-b from-slate-50 to-slate-100 dark:from-slate-900 dark:to-slate-800">
      {/* Header */}
      <header className="border-b bg-white dark:bg-slate-950">
        <div className="container flex h-16 items-center justify-between">
          <div className="flex items-center gap-2">
            <span className="text-2xl font-bold text-primary">Part-DB</span>
          </div>
          <nav className="flex items-center gap-4">
            <Link to="/parts">
              <Button variant="ghost">Parts</Button>
            </Link>
            <Link to="/categories">
              <Button variant="ghost">Categories</Button>
            </Link>
            <Link to="/storage">
              <Button variant="ghost">Storage</Button>
            </Link>
            {isAuthenticated ? (
              <div className="flex items-center gap-3">
                <span className="text-sm text-muted-foreground">
                  {user?.name ?? user?.email}
                </span>
                <Button variant="outline" onClick={() => signOut()}>
                  Sign out
                </Button>
              </div>
            ) : (
              <Link to="/login">
                <Button>Sign in</Button>
              </Link>
            )}
          </nav>
        </div>
      </header>

      {/* Main Content */}
      <div className="container py-12">
        <div className="mx-auto max-w-4xl space-y-8">
          {/* Welcome Section */}
          <div className="text-center">
            <h1 className="text-4xl font-bold tracking-tight">
              Welcome to Part-DB
            </h1>
            <p className="mt-4 text-lg text-muted-foreground">
              A modern electronic parts inventory management system. Manage your
              components, track stock levels, and organize your projects.
            </p>
          </div>

          {/* Quick Actions */}
          <div className="grid gap-6 md:grid-cols-3">
            <Card>
              <CardHeader>
                <CardTitle>📦 Parts</CardTitle>
                <CardDescription>
                  Browse and manage your electronic components
                </CardDescription>
              </CardHeader>
              <CardContent>
                <Link to="/parts">
                  <Button className="w-full">View Parts</Button>
                </Link>
              </CardContent>
            </Card>

            <Card>
              <CardHeader>
                <CardTitle>📁 Categories</CardTitle>
                <CardDescription>
                  Organize parts into categories and subcategories
                </CardDescription>
              </CardHeader>
              <CardContent>
                <Link to="/categories">
                  <Button className="w-full">View Categories</Button>
                </Link>
              </CardContent>
            </Card>

            <Card>
              <CardHeader>
                <CardTitle>🏷️ Storage</CardTitle>
                <CardDescription>
                  Track where your components are stored
                </CardDescription>
              </CardHeader>
              <CardContent>
                <Link to="/storage">
                  <Button className="w-full">View Storage</Button>
                </Link>
              </CardContent>
            </Card>
          </div>

          {/* Statistics */}
          <Card>
            <CardHeader>
              <CardTitle>Quick Statistics</CardTitle>
              <CardDescription>Overview of your inventory</CardDescription>
            </CardHeader>
            <CardContent>
              <div className="grid gap-4 md:grid-cols-4">
                <div className="text-center">
                  <div className="text-3xl font-bold">-</div>
                  <div className="text-sm text-muted-foreground">
                    Total Parts
                  </div>
                </div>
                <div className="text-center">
                  <div className="text-3xl font-bold">-</div>
                  <div className="text-sm text-muted-foreground">
                    Categories
                  </div>
                </div>
                <div className="text-center">
                  <div className="text-3xl font-bold">-</div>
                  <div className="text-sm text-muted-foreground">
                    Storage Locations
                  </div>
                </div>
                <div className="text-center">
                  <div className="text-3xl font-bold">-</div>
                  <div className="text-sm text-muted-foreground">Suppliers</div>
                </div>
              </div>
            </CardContent>
          </Card>
        </div>
      </div>

      {/* Footer */}
      <footer className="border-t bg-white dark:bg-slate-950 py-6">
        <div className="container text-center text-sm text-muted-foreground">
          <p>Part-DB - Open Source Electronic Parts Management</p>
          <p className="mt-1">Licensed under AGPL-3.0</p>
        </div>
      </footer>
    </main>
  );
}
