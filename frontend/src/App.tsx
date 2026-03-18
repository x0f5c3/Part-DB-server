import { BrowserRouter, Routes, Route } from "react-router-dom";
import { AuthProvider } from "@/components/auth/AuthProvider";
import Home from "@/pages/Home";
import Login from "@/pages/Login";
import Parts from "@/pages/Parts";
import Categories from "@/pages/Categories";
import Storage from "@/pages/Storage";
import AuthCallback from "@/pages/auth/Callback";

/**
 * Root application component.
 *
 * Wraps the entire application in a BrowserRouter and provides the
 * authentication context via AuthProvider.  All client-side routes are
 * declared here.
 */
export default function App() {
  return (
    <BrowserRouter>
      <AuthProvider>
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/login" element={<Login />} />
          <Route path="/parts" element={<Parts />} />
          <Route path="/categories" element={<Categories />} />
          <Route path="/storage" element={<Storage />} />
          <Route path="/auth/callback" element={<AuthCallback />} />
        </Routes>
      </AuthProvider>
    </BrowserRouter>
  );
}
