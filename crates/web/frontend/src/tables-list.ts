import { fetchTablesList, formatBytes, EtsTableInfo } from './api'
import { escapeHtml } from './utils'

function createTableRow(table: EtsTableInfo): string {
  return `
    <tr>
      <td><a href="/tables/${encodeURIComponent(table.name)}" class="table-link">${escapeHtml(table.name)}</a></td>
      <td>${escapeHtml(table.table_type)}</td>
      <td class="text-end">${table.size.toLocaleString()}</td>
      <td class="text-end">${formatBytes(table.memory_bytes)}</td>
      <td>${escapeHtml(table.owner)}</td>
      <td>${escapeHtml(table.protection)}</td>
    </tr>
  `
}

export async function renderTablesList(container: HTMLElement): Promise<void> {
  container.innerHTML = `
    <div class="loading">
      <div class="spinner-border text-primary" role="status">
        <span class="visually-hidden">Loading...</span>
      </div>
    </div>
  `

  try {
    const response = await fetchTablesList()

    container.innerHTML = `
      <div class="d-flex justify-content-between align-items-center mb-3">
        <h2>ETS Tables</h2>
        <span class="badge bg-secondary">${response.total} tables</span>
      </div>
      <div class="table-responsive">
        <table class="table table-striped table-hover">
          <thead class="table-dark">
            <tr>
              <th>Name</th>
              <th>Type</th>
              <th class="text-end">Objects</th>
              <th class="text-end">Memory</th>
              <th>Owner</th>
              <th>Protection</th>
            </tr>
          </thead>
          <tbody>
            ${response.tables.map(createTableRow).join('')}
          </tbody>
        </table>
      </div>
    `
  } catch (error) {
    const message = error instanceof Error ? error.message : 'Unknown error'
    container.innerHTML = `
      <div class="alert alert-danger" role="alert">
        <strong>Error:</strong> ${escapeHtml(message)}
      </div>
    `
  }
}
