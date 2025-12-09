/**
 * TypeScript type definitions for the Part-DB API.
 * These types match the Rust backend models.
 */

export interface Category {
  id: number;
  name: string;
  parent_id: number | null;
  comment: string;
  partname_hint: string;
  partname_regex: string;
  disable_footprints: boolean;
  disable_manufacturers: boolean;
  disable_autodatasheets: boolean;
  disable_properties: boolean;
  default_description: string;
  default_comment: string;
  datetime_added: string | null;
  last_modified: string | null;
}

export interface Footprint {
  id: number;
  name: string;
  parent_id: number | null;
  comment: string;
  datetime_added: string | null;
  last_modified: string | null;
}

export interface Manufacturer {
  id: number;
  name: string;
  parent_id: number | null;
  comment: string;
  address: string;
  phone_number: string;
  fax_number: string;
  email_address: string;
  website: string;
  datetime_added: string | null;
  last_modified: string | null;
}

export interface StorageLocation {
  id: number;
  name: string;
  parent_id: number | null;
  comment: string;
  is_full: boolean;
  datetime_added: string | null;
  last_modified: string | null;
}

export interface Supplier {
  id: number;
  name: string;
  parent_id: number | null;
  comment: string;
  address: string;
  phone_number: string;
  fax_number: string;
  email_address: string;
  website: string;
  shipping_costs: number | null;
  datetime_added: string | null;
  last_modified: string | null;
}

export interface Part {
  id: number;
  name: string;
  description: string;
  comment: string;
  visible: boolean;
  favorite: boolean;
  id_category: number;
  id_footprint: number | null;
  id_manufacturer: number | null;
  ipn: string | null;
  mass: number | null;
  tags: string;
  manufacturer_product_number: string;
  manufacturer_product_url: string;
  minamount: number;
  needs_review: boolean;
  datetime_added: string | null;
  last_modified: string | null;
}

export interface PartLot {
  id: number;
  id_part: number;
  id_storage_location: number | null;
  description: string;
  comment: string;
  expiration_date: string | null;
  amount: number;
  instock_unknown: boolean;
  needs_refill: boolean;
  datetime_added: string | null;
  last_modified: string | null;
}

export interface User {
  id: number;
  name: string;
  first_name: string | null;
  last_name: string | null;
  email: string | null;
  disabled: boolean;
  config_theme: string | null;
  datetime_added: string | null;
  last_modified: string | null;
}

// DTOs for creating/updating

export interface CreatePart {
  name: string;
  description?: string;
  comment?: string;
  category_id: number;
  footprint_id?: number;
  manufacturer_id?: number;
}

export interface UpdatePart {
  name?: string;
  description?: string;
  comment?: string;
  category_id?: number;
  footprint_id?: number;
  manufacturer_id?: number;
  favorite?: boolean;
}

export interface CreateCategory {
  name: string;
  parent_id?: number;
  comment?: string;
}

export interface UpdateCategory {
  name?: string;
  parent_id?: number;
  comment?: string;
}

// Pagination

export interface PaginationParams {
  page?: number;
  per_page?: number;
}

export interface PaginatedResponse<T> {
  items: T[];
  total: number;
  page: number;
  per_page: number;
  total_pages: number;
}

// API Info

export interface ApiInfo {
  name: string;
  version: string;
  description: string;
}
