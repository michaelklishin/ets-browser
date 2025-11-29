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

use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use std::result::Result as StdResult;
use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum ServerError {
    #[error("ETS error: {0}")]
    Ets(#[from] ets_lib::Error),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            ServerError::Ets(ets_lib::Error::TableNotFound(name)) => {
                (StatusCode::NOT_FOUND, format!("Table not found: {}", name))
            }
            ServerError::Ets(ets_lib::Error::CookieNotFound(msg)) => {
                (StatusCode::UNAUTHORIZED, msg.clone())
            }
            ServerError::Ets(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}

pub type ServerResult<T, E = ServerError> = StdResult<T, E>;
