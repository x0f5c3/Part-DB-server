import { Link } from "react-router-dom";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";

// Placeholder parts data
const placeholderParts = [
  {
    id: 1,
    name: "10k Resistor",
    description: "1/4W, 5% tolerance",
    category: "Resistors",
    manufacturer: "Generic",
    inStock: 150,
  },
  {
    id: 2,
    name: "100uF Capacitor",
    description: "Electrolytic, 25V",
    category: "Capacitors",
    manufacturer: "Nichicon",
    inStock: 45,
  },
  {
    id: 3,
    name: "ATmega328P",
    description: "8-bit AVR Microcontroller",
    category: "Microcontrollers",
    manufacturer: "Microchip",
    inStock: 12,
  },
];

export default function Parts() {
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
              <Button variant="ghost" className="bg-accent">
                Parts
              </Button>
            </Link>
            <Link to="/categories">
              <Button variant="ghost">Categories</Button>
            </Link>
            <Link to="/storage">
              <Button variant="ghost">Storage</Button>
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
              <h1 className="text-3xl font-bold tracking-tight">Parts</h1>
              <p className="text-muted-foreground">
                Browse and manage your electronic components
              </p>
            </div>
            <Button>Add New Part</Button>
          </div>

          {/* Search and Filters */}
          <Card>
            <CardContent className="pt-6">
              <div className="flex gap-4">
                <Input placeholder="Search parts..." className="max-w-sm" />
                <Button variant="outline">Filter</Button>
              </div>
            </CardContent>
          </Card>

          {/* Parts Table */}
          <Card>
            <CardHeader>
              <CardTitle>All Parts</CardTitle>
            </CardHeader>
            <CardContent>
              <Table>
                <TableHeader>
                  <TableRow>
                    <TableHead>Name</TableHead>
                    <TableHead>Description</TableHead>
                    <TableHead>Category</TableHead>
                    <TableHead>Manufacturer</TableHead>
                    <TableHead className="text-right">In Stock</TableHead>
                    <TableHead>Actions</TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {placeholderParts.map((part) => (
                    <TableRow key={part.id}>
                      <TableCell className="font-medium">
                        <Link
                          to={`/parts/${part.id}`}
                          className="hover:underline"
                        >
                          {part.name}
                        </Link>
                      </TableCell>
                      <TableCell>{part.description}</TableCell>
                      <TableCell>{part.category}</TableCell>
                      <TableCell>{part.manufacturer}</TableCell>
                      <TableCell className="text-right">
                        {part.inStock}
                      </TableCell>
                      <TableCell>
                        <div className="flex gap-2">
                          <Button variant="ghost" size="sm">
                            Edit
                          </Button>
                          <Button variant="ghost" size="sm">
                            Delete
                          </Button>
                        </div>
                      </TableCell>
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
            </CardContent>
          </Card>

          {/* Pagination */}
          <div className="flex items-center justify-center gap-2">
            <Button variant="outline" disabled>
              Previous
            </Button>
            <Button variant="outline">1</Button>
            <Button variant="outline" disabled>
              Next
            </Button>
          </div>
        </div>
      </div>
    </main>
  );
}
