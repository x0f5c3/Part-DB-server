/**
 * API client for communicating with the Part-DB backend.
 */

import type {
  Part,
  Category,
  Footprint,
  Manufacturer,
  StorageLocation,
  Supplier,
  User,
  CreatePart,
  UpdatePart,
  CreateCategory,
  UpdateCategory,
  PaginatedResponse,
  PaginationParams,
  ApiInfo,
} from "@/types/api";

const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || "/api";

/**
 * Custom error class for API errors.
 */
export class ApiError extends Error {
  constructor(
    public status: number,
    public code: string,
    message: string
  ) {
    super(message);
    this.name = "ApiError";
  }
}

/**
 * Fetch wrapper with error handling.
 */
async function fetchApi<T>(
  endpoint: string,
  options: RequestInit = {}
): Promise<T> {
  const url = `${API_BASE_URL}${endpoint}`;

  const response = await fetch(url, {
    ...options,
    headers: {
      "Content-Type": "application/json",
      ...options.headers,
    },
  });

  if (!response.ok) {
    const errorData = await response.json().catch(() => ({}));
    throw new ApiError(
      response.status,
      errorData.code || "UNKNOWN_ERROR",
      errorData.message || "An error occurred"
    );
  }

  // Handle 204 No Content
  if (response.status === 204) {
    return undefined as T;
  }

  return response.json();
}

/**
 * Build query string from pagination params.
 */
function buildQueryString(params: PaginationParams): string {
  const searchParams = new URLSearchParams();
  if (params.page) searchParams.set("page", params.page.toString());
  if (params.per_page) searchParams.set("per_page", params.per_page.toString());
  const queryString = searchParams.toString();
  return queryString ? `?${queryString}` : "";
}

// ============================================================================
// API Info
// ============================================================================

/**
 * Get API information.
 */
export async function getApiInfo(): Promise<ApiInfo> {
  return fetchApi<ApiInfo>("/");
}

// ============================================================================
// Parts
// ============================================================================

/**
 * List all parts with pagination.
 */
export async function listParts(
  params: PaginationParams = {}
): Promise<PaginatedResponse<Part>> {
  return fetchApi<PaginatedResponse<Part>>(`/parts${buildQueryString(params)}`);
}

/**
 * Get a single part by ID.
 */
export async function getPart(id: number): Promise<Part> {
  return fetchApi<Part>(`/parts/${id}`);
}

/**
 * Create a new part.
 */
export async function createPart(data: CreatePart): Promise<Part> {
  return fetchApi<Part>("/parts", {
    method: "POST",
    body: JSON.stringify(data),
  });
}

/**
 * Update an existing part.
 */
export async function updatePart(id: number, data: UpdatePart): Promise<Part> {
  return fetchApi<Part>(`/parts/${id}`, {
    method: "PATCH",
    body: JSON.stringify(data),
  });
}

/**
 * Delete a part.
 */
export async function deletePart(id: number): Promise<void> {
  return fetchApi<void>(`/parts/${id}`, {
    method: "DELETE",
  });
}

// ============================================================================
// Categories
// ============================================================================

/**
 * List all categories with pagination.
 */
export async function listCategories(
  params: PaginationParams = {}
): Promise<PaginatedResponse<Category>> {
  return fetchApi<PaginatedResponse<Category>>(
    `/categories${buildQueryString(params)}`
  );
}

/**
 * Get a single category by ID.
 */
export async function getCategory(id: number): Promise<Category> {
  return fetchApi<Category>(`/categories/${id}`);
}

/**
 * Create a new category.
 */
export async function createCategory(data: CreateCategory): Promise<Category> {
  return fetchApi<Category>("/categories", {
    method: "POST",
    body: JSON.stringify(data),
  });
}

/**
 * Update an existing category.
 */
export async function updateCategory(
  id: number,
  data: UpdateCategory
): Promise<Category> {
  return fetchApi<Category>(`/categories/${id}`, {
    method: "PATCH",
    body: JSON.stringify(data),
  });
}

/**
 * Delete a category.
 */
export async function deleteCategory(id: number): Promise<void> {
  return fetchApi<void>(`/categories/${id}`, {
    method: "DELETE",
  });
}

// ============================================================================
// Footprints (Read-only)
// ============================================================================

/**
 * List all footprints with pagination.
 */
export async function listFootprints(
  params: PaginationParams = {}
): Promise<PaginatedResponse<Footprint>> {
  return fetchApi<PaginatedResponse<Footprint>>(
    `/footprints${buildQueryString(params)}`
  );
}

/**
 * Get a single footprint by ID.
 */
export async function getFootprint(id: number): Promise<Footprint> {
  return fetchApi<Footprint>(`/footprints/${id}`);
}

// ============================================================================
// Manufacturers (Read-only)
// ============================================================================

/**
 * List all manufacturers with pagination.
 */
export async function listManufacturers(
  params: PaginationParams = {}
): Promise<PaginatedResponse<Manufacturer>> {
  return fetchApi<PaginatedResponse<Manufacturer>>(
    `/manufacturers${buildQueryString(params)}`
  );
}

/**
 * Get a single manufacturer by ID.
 */
export async function getManufacturer(id: number): Promise<Manufacturer> {
  return fetchApi<Manufacturer>(`/manufacturers/${id}`);
}

// ============================================================================
// Storage Locations (Read-only)
// ============================================================================

/**
 * List all storage locations with pagination.
 */
export async function listStorageLocations(
  params: PaginationParams = {}
): Promise<PaginatedResponse<StorageLocation>> {
  return fetchApi<PaginatedResponse<StorageLocation>>(
    `/storage_locations${buildQueryString(params)}`
  );
}

/**
 * Get a single storage location by ID.
 */
export async function getStorageLocation(id: number): Promise<StorageLocation> {
  return fetchApi<StorageLocation>(`/storage_locations/${id}`);
}

// ============================================================================
// Suppliers (Read-only)
// ============================================================================

/**
 * List all suppliers with pagination.
 */
export async function listSuppliers(
  params: PaginationParams = {}
): Promise<PaginatedResponse<Supplier>> {
  return fetchApi<PaginatedResponse<Supplier>>(
    `/suppliers${buildQueryString(params)}`
  );
}

/**
 * Get a single supplier by ID.
 */
export async function getSupplier(id: number): Promise<Supplier> {
  return fetchApi<Supplier>(`/suppliers/${id}`);
}

// ============================================================================
// Users (Read-only)
// ============================================================================

/**
 * List all users with pagination.
 */
export async function listUsers(
  params: PaginationParams = {}
): Promise<PaginatedResponse<User>> {
  return fetchApi<PaginatedResponse<User>>(`/users${buildQueryString(params)}`);
}

/**
 * Get a single user by ID.
 */
export async function getUser(id: number): Promise<User> {
  return fetchApi<User>(`/users/${id}`);
}
