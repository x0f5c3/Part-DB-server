/**
 * Parts store using Zustand.
 *
 * Manages parts data, loading states, and CRUD operations.
 */

import { create } from "zustand";
import type { Part, PaginationParams } from "@/types/api";
import * as api from "@/lib/api";

interface PartsState {
  /** List of parts */
  parts: Part[];
  /** Selected part for viewing/editing */
  selectedPart: Part | null;
  /** Total number of parts */
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
  /** Fetch parts with pagination */
  fetchParts: (params?: PaginationParams) => Promise<void>;
  /** Get a single part by ID */
  fetchPart: (id: number) => Promise<void>;
  /** Create a new part */
  createPart: (data: api.CreatePart) => Promise<Part>;
  /** Update a part */
  updatePart: (id: number, data: api.UpdatePart) => Promise<Part>;
  /** Delete a part */
  deletePart: (id: number) => Promise<void>;
  /** Select a part */
  selectPart: (part: Part | null) => void;
  /** Clear error */
  clearError: () => void;
}

export const usePartsStore = create<PartsState>((set) => ({
  parts: [],
  selectedPart: null,
  total: 0,
  page: 1,
  perPage: 30,
  totalPages: 0,
  isLoading: false,
  error: null,

  fetchParts: async (params = {}) => {
    set({ isLoading: true, error: null });
    try {
      const response = await api.listParts(params);
      set({
        parts: response.items,
        total: response.total,
        page: response.page,
        perPage: response.per_page,
        totalPages: response.total_pages,
        isLoading: false,
      });
    } catch (error) {
      set({
        error: error instanceof Error ? error.message : "Failed to fetch parts",
        isLoading: false,
      });
    }
  },

  fetchPart: async (id) => {
    set({ isLoading: true, error: null });
    try {
      const part = await api.getPart(id);
      set({ selectedPart: part, isLoading: false });
    } catch (error) {
      set({
        error: error instanceof Error ? error.message : "Failed to fetch part",
        isLoading: false,
      });
    }
  },

  createPart: async (data) => {
    set({ isLoading: true, error: null });
    try {
      const part = await api.createPart(data);
      set((state) => ({
        parts: [...state.parts, part],
        total: state.total + 1,
        isLoading: false,
      }));
      return part;
    } catch (error) {
      set({
        error: error instanceof Error ? error.message : "Failed to create part",
        isLoading: false,
      });
      throw error;
    }
  },

  updatePart: async (id, data) => {
    set({ isLoading: true, error: null });
    try {
      const part = await api.updatePart(id, data);
      set((state) => ({
        parts: state.parts.map((p) => (p.id === id ? part : p)),
        selectedPart: state.selectedPart?.id === id ? part : state.selectedPart,
        isLoading: false,
      }));
      return part;
    } catch (error) {
      set({
        error: error instanceof Error ? error.message : "Failed to update part",
        isLoading: false,
      });
      throw error;
    }
  },

  deletePart: async (id) => {
    set({ isLoading: true, error: null });
    try {
      await api.deletePart(id);
      set((state) => ({
        parts: state.parts.filter((p) => p.id !== id),
        total: state.total - 1,
        selectedPart: state.selectedPart?.id === id ? null : state.selectedPart,
        isLoading: false,
      }));
    } catch (error) {
      set({
        error: error instanceof Error ? error.message : "Failed to delete part",
        isLoading: false,
      });
      throw error;
    }
  },

  selectPart: (part) => set({ selectedPart: part }),

  clearError: () => set({ error: null }),
}));
