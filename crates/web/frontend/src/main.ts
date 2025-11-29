import { renderTablesList } from './tables-list'
import { renderTableContents } from './table-contents'

function getTableNameFromPath(): string | null {
  const match = window.location.pathname.match(/^\/tables\/(.+)$/)
  return match ? decodeURIComponent(match[1]) : null
}

async function main(): Promise<void> {
  const app = document.getElementById('app')
  if (!app) return

  const tableName = getTableNameFromPath()

  if (tableName) {
    await renderTableContents(app, tableName)
  } else {
    await renderTablesList(app)
  }
}

main()
