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

use crate::api::tables;
use axum::Router;
use axum::http::header;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

#[derive(Clone)]
pub struct AppState {
    pub node: Arc<str>,
    pub cookie: Arc<str>,
}

impl AppState {
    pub fn new(node: String, cookie: String) -> Self {
        Self {
            node: Arc::from(node),
            cookie: Arc::from(cookie),
        }
    }
}

pub fn create_router(state: AppState) -> Router {
    let api_routes = Router::new()
        .route("/tables/list", get(tables::list_tables))
        .route("/tables/{name}/contents", get(tables::get_table_contents))
        .with_state(state.clone());

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let assets_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/frontend/dist/assets");

    Router::new()
        .route("/", get(spa_handler))
        .route("/tables/{name}", get(spa_handler))
        .nest("/api/v1", api_routes)
        .nest_service("/assets", ServeDir::new(assets_dir))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
}

async fn spa_handler() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
        Html(include_str!("../frontend/dist/index.html")),
    )
}
