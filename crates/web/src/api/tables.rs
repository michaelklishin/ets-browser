// Copyright (C) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::errors::ServerResult;
use crate::server::AppState;
use axum::Json;
use axum::extract::{Path, State};
use ets_lib::EtsTableInfo;
use serde::Serialize;

#[derive(Serialize)]
pub struct TablesListResponse {
    pub tables: Vec<EtsTableInfo>,
    pub total: usize,
}

pub async fn list_tables(State(state): State<AppState>) -> ServerResult<Json<TablesListResponse>> {
    let tables = ets_lib::list_tables(&state.node, &state.cookie).await?;
    let total = tables.len();

    Ok(Json(TablesListResponse { tables, total }))
}

#[derive(Serialize)]
pub struct TableEntry {
    pub index: usize,
    pub entry: String,
}

#[derive(Serialize)]
pub struct TableContentsResponse {
    pub table_name: String,
    pub entries: Vec<TableEntry>,
    pub total: usize,
}

pub async fn get_table_contents(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> ServerResult<Json<TableContentsResponse>> {
    let entries = ets_lib::dump_table(&state.node, &state.cookie, &name).await?;
    let total = entries.len();

    let entries: Vec<TableEntry> = entries
        .into_iter()
        .enumerate()
        .map(|(i, entry)| TableEntry {
            index: i + 1,
            entry: entry.to_string(),
        })
        .collect();

    Ok(Json(TableContentsResponse {
        table_name: name,
        entries,
        total,
    }))
}
