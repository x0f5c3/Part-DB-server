import { Link } from "react-router-dom";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";

// Placeholder storage data
const placeholderLocations = [
  {
    id: 1,
    name: "Lab Bench A",
    description: "Main workbench storage",
    partCount: 125,
    sublocations: ["Drawer 1", "Drawer 2", "Bin A"],
  },
  {
    id: 2,
    name: "Component Cabinet",
    description: "SMD components storage",
    partCount: 450,
    sublocations: ["Shelf 1", "Shelf 2", "Shelf 3"],
  },
  {
    id: 3,
    name: "Project Box",
    description: "Active project components",
    partCount: 35,
    sublocations: ["Current", "Pending"],
  },
];

export default function Storage() {
  return (
    <main className="min-h-screen bg-slate-50 dark:bg-slate-900">
      {/* Header */}
      <header className="border-b bg-white dark:bg-slate-950">
        <div className="container flex h-16 items-center justify-between">
          <Link to="/" className="flex items-center gap-2">
            <span className="text-2xl font-bold text-primary">Part-DB</span>
          </Link>
          <nav className="flex items-center gap-4">
            <Link to="/parts">
              <Button variant="ghost">Parts</Button>
            </Link>
            <Link to="/categories">
              <Button variant="ghost">Categories</Button>
            </Link>
            <Link to="/storage">
              <Button variant="ghost" className="bg-accent">
                Storage
              </Button>
            </Link>
            <Link to="/login">
              <Button>Login</Button>
            </Link>
          </nav>
        </div>
      </header>

      {/* Main Content */}
      <div className="container py-8">
        <div className="space-y-6">
          {/* Page Header */}
          <div className="flex items-center justify-between">
            <div>
              <h1 className="text-3xl font-bold tracking-tight">
                Storage Locations
              </h1>
              <p className="text-muted-foreground">
                Track where your components are stored
              </p>
            </div>
            <Button>Add New Location</Button>
          </div>

          {/* Storage Locations Grid */}
          <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
            {placeholderLocations.map((location) => (
              <Card
                key={location.id}
                className="hover:shadow-md transition-shadow"
              >
                <CardHeader>
                  <CardTitle className="flex items-center justify-between">
                    <span>📦 {location.name}</span>
                    <span className="text-sm font-normal text-muted-foreground">
                      {location.partCount} items
                    </span>
                  </CardTitle>
                </CardHeader>
                <CardContent>
                  <p className="text-sm text-muted-foreground mb-4">
                    {location.description}
                  </p>
                  <div className="flex flex-wrap gap-2">
                    {location.sublocations.map((sub) => (
                      <span
                        key={sub}
                        className="px-2 py-1 text-xs rounded-full bg-secondary text-secondary-foreground"
                      >
                        {sub}
                      </span>
                    ))}
                  </div>
                  <div className="mt-4 flex gap-2">
                    <Link to={`/storage/${location.id}`}>
                      <Button variant="outline" size="sm">
                        View Contents
                      </Button>
                    </Link>
                    <Button variant="ghost" size="sm">
                      Edit
                    </Button>
                  </div>
                </CardContent>
              </Card>
            ))}
          </div>
        </div>
      </div>
    </main>
  );
}
