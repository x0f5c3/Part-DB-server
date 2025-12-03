import Link from "next/link";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";

// Placeholder categories data
const placeholderCategories = [
  {
    id: 1,
    name: "Resistors",
    description: "All resistor types",
    partCount: 45,
    subcategories: ["SMD", "Through-hole", "Power"],
  },
  {
    id: 2,
    name: "Capacitors",
    description: "Capacitors of all types",
    partCount: 32,
    subcategories: ["Electrolytic", "Ceramic", "Film"],
  },
  {
    id: 3,
    name: "Microcontrollers",
    description: "MCUs and development boards",
    partCount: 15,
    subcategories: ["AVR", "ARM", "PIC"],
  },
  {
    id: 4,
    name: "Connectors",
    description: "Various connector types",
    partCount: 28,
    subcategories: ["Headers", "USB", "Power"],
  },
];

export default function CategoriesPage() {
  return (
    <main className="min-h-screen bg-slate-50 dark:bg-slate-900">
      {/* Header */}
      <header className="border-b bg-white dark:bg-slate-950">
        <div className="container flex h-16 items-center justify-between">
          <Link href="/" className="flex items-center gap-2">
            <span className="text-2xl font-bold text-primary">Part-DB</span>
          </Link>
          <nav className="flex items-center gap-4">
            <Link href="/parts">
              <Button variant="ghost">Parts</Button>
            </Link>
            <Link href="/categories">
              <Button variant="ghost" className="bg-accent">Categories</Button>
            </Link>
            <Link href="/storage">
              <Button variant="ghost">Storage</Button>
            </Link>
            <Button>Login</Button>
          </nav>
        </div>
      </header>

      {/* Main Content */}
      <div className="container py-8">
        <div className="space-y-6">
          {/* Page Header */}
          <div className="flex items-center justify-between">
            <div>
              <h1 className="text-3xl font-bold tracking-tight">Categories</h1>
              <p className="text-muted-foreground">
                Organize your parts into categories
              </p>
            </div>
            <Button>Add New Category</Button>
          </div>

          {/* Categories Grid */}
          <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
            {placeholderCategories.map((category) => (
              <Card key={category.id} className="hover:shadow-md transition-shadow">
                <CardHeader>
                  <CardTitle className="flex items-center justify-between">
                    <span>{category.name}</span>
                    <span className="text-sm font-normal text-muted-foreground">
                      {category.partCount} parts
                    </span>
                  </CardTitle>
                </CardHeader>
                <CardContent>
                  <p className="text-sm text-muted-foreground mb-4">
                    {category.description}
                  </p>
                  <div className="flex flex-wrap gap-2">
                    {category.subcategories.map((sub) => (
                      <span
                        key={sub}
                        className="px-2 py-1 text-xs rounded-full bg-secondary text-secondary-foreground"
                      >
                        {sub}
                      </span>
                    ))}
                  </div>
                  <div className="mt-4 flex gap-2">
                    <Link href={`/parts?category=${category.id}`}>
                      <Button variant="outline" size="sm">View Parts</Button>
                    </Link>
                    <Button variant="ghost" size="sm">Edit</Button>
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
