export type TableType = 'set' | 'ordered_set' | 'bag' | 'duplicate_bag'
export type Protection = 'public' | 'protected' | 'private'

export interface EtsTableInfo {
  name: string
  table_type: TableType
  size: number
  memory_bytes: number
  owner: string
  protection: Protection
}

export interface TablesListResponse {
  tables: EtsTableInfo[]
  total: number
}

export interface TableEntry {
  index: number
  entry: string
}

export interface TableContentsResponse {
  table_name: string
  entries: TableEntry[]
  total: number
}

export interface ApiError {
  error: string
}

export async function fetchTablesList(): Promise<TablesListResponse> {
  const response = await fetch('/api/v1/tables/list')
  if (!response.ok) {
    const error: ApiError = await response.json()
    throw new Error(error.error)
  }
  return response.json()
}

export async function fetchTableContents(name: string): Promise<TableContentsResponse> {
  const response = await fetch(`/api/v1/tables/${encodeURIComponent(name)}/contents`)
  if (!response.ok) {
    const error: ApiError = await response.json()
    throw new Error(error.error)
  }
  return response.json()
}

export function formatBytes(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KiB`
  if (bytes < 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(1)} MiB`
  return `${(bytes / (1024 * 1024 * 1024)).toFixed(1)} GiB`
}
