/**
 * Categories store using Zustand.
 *
 * Manages categories data, loading states, and CRUD operations.
 */

import { create } from "zustand";
import type { Category, PaginatedResponse, PaginationParams } from "@/types/api";
import * as api from "@/lib/api";

interface CategoriesState {
  /** List of categories */
  categories: Category[];
  /** Selected category for viewing/editing */
  selectedCategory: Category | null;
  /** Total number of categories */
  total: number;
  /** Current page */
  page: number;
  /** Items per page */
  perPage: number;
  /** Total pages */
  totalPages: number;
  /** Loading state */
  isLoading: boolean;
  /** Error message */
  error: string | null;
  /** Fetch categories with pagination */
  fetchCategories: (params?: PaginationParams) => Promise<void>;
  /** Get a single category by ID */
  fetchCategory: (id: number) => Promise<void>;
  /** Create a new category */
  createCategory: (data: api.CreateCategory) => Promise<Category>;
  /** Update a category */
  updateCategory: (id: number, data: api.UpdateCategory) => Promise<Category>;
  /** Delete a category */
  deleteCategory: (id: number) => Promise<void>;
  /** Select a category */
  selectCategory: (category: Category | null) => void;
  /** Clear error */
  clearError: () => void;
}

export const useCategoriesStore = create<CategoriesState>((set) => ({
  categories: [],
  selectedCategory: null,
  total: 0,
  page: 1,
  perPage: 30,
  totalPages: 0,
  isLoading: false,
  error: null,

  fetchCategories: async (params = {}) => {
    set({ isLoading: true, error: null });
    try {
      const response = await api.listCategories(params);
      set({
        categories: response.items,
        total: response.total,
        page: response.page,
        perPage: response.per_page,
        totalPages: response.total_pages,
        isLoading: false,
      });
    } catch (error) {
      set({
        error: error instanceof Error ? error.message : "Failed to fetch categories",
        isLoading: false,
      });
    }
  },

  fetchCategory: async (id) => {
    set({ isLoading: true, error: null });
    try {
      const category = await api.getCategory(id);
      set({ selectedCategory: category, isLoading: false });
    } catch (error) {
      set({
        error: error instanceof Error ? error.message : "Failed to fetch category",
        isLoading: false,
      });
    }
  },

  createCategory: async (data) => {
    set({ isLoading: true, error: null });
    try {
      const category = await api.createCategory(data);
      set((state) => ({
        categories: [...state.categories, category],
        total: state.total + 1,
        isLoading: false,
      }));
      return category;
    } catch (error) {
      set({
        error: error instanceof Error ? error.message : "Failed to create category",
        isLoading: false,
      });
      throw error;
    }
  },

  updateCategory: async (id, data) => {
    set({ isLoading: true, error: null });
    try {
      const category = await api.updateCategory(id, data);
      set((state) => ({
        categories: state.categories.map((c) => (c.id === id ? category : c)),
        selectedCategory:
          state.selectedCategory?.id === id ? category : state.selectedCategory,
        isLoading: false,
      }));
      return category;
    } catch (error) {
      set({
        error: error instanceof Error ? error.message : "Failed to update category",
        isLoading: false,
      });
      throw error;
    }
  },

  deleteCategory: async (id) => {
    set({ isLoading: true, error: null });
    try {
      await api.deleteCategory(id);
      set((state) => ({
        categories: state.categories.filter((c) => c.id !== id),
        total: state.total - 1,
        selectedCategory:
          state.selectedCategory?.id === id ? null : state.selectedCategory,
        isLoading: false,
      }));
    } catch (error) {
      set({
        error: error instanceof Error ? error.message : "Failed to delete category",
        isLoading: false,
      });
      throw error;
    }
  },

  selectCategory: (category) => set({ selectedCategory: category }),

  clearError: () => set({ error: null }),
}));
