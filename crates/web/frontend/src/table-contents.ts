import { fetchTableContents, TableEntry } from './api'
import { escapeHtml } from './utils'

function createEntryRow(entry: TableEntry): string {
  return `
    <tr>
      <td class="text-end" style="width: 60px;">${entry.index}</td>
      <td class="entry-text">${escapeHtml(entry.entry)}</td>
    </tr>
  `
}

export async function renderTableContents(container: HTMLElement, tableName: string): Promise<void> {
  container.innerHTML = `
    <div class="loading">
      <div class="spinner-border text-primary" role="status">
        <span class="visually-hidden">Loading...</span>
      </div>
    </div>
  `

  try {
    const response = await fetchTableContents(tableName)

    const entriesHtml = response.entries.length > 0
      ? `
        <div class="table-responsive">
          <table class="table table-striped table-hover">
            <thead class="table-dark">
              <tr>
                <th class="text-end" style="width: 60px;">#</th>
                <th>Entry</th>
              </tr>
            </thead>
            <tbody>
              ${response.entries.map(createEntryRow).join('')}
            </tbody>
          </table>
        </div>
      `
      : `<div class="alert alert-info">This table is empty.</div>`

    container.innerHTML = `
      <nav aria-label="breadcrumb" class="mb-3">
        <ol class="breadcrumb">
          <li class="breadcrumb-item"><a href="/">Tables</a></li>
          <li class="breadcrumb-item active" aria-current="page">${escapeHtml(tableName)}</li>
        </ol>
      </nav>
      <div class="d-flex justify-content-between align-items-center mb-3">
        <h2>Table: ${escapeHtml(tableName)}</h2>
        <span class="badge bg-secondary">${response.total} entries</span>
      </div>
      ${entriesHtml}
    `
  } catch (error) {
    const message = error instanceof Error ? error.message : 'Unknown error'
    container.innerHTML = `
      <nav aria-label="breadcrumb" class="mb-3">
        <ol class="breadcrumb">
          <li class="breadcrumb-item"><a href="/">Tables</a></li>
          <li class="breadcrumb-item active" aria-current="page">${escapeHtml(tableName)}</li>
        </ol>
      </nav>
      <div class="alert alert-danger" role="alert">
        <strong>Error:</strong> ${escapeHtml(message)}
      </div>
    `
  }
}
