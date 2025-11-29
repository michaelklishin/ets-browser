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

use axum::body::Body;
use axum::http::{Request, StatusCode};
use ets_web::{AppState, create_router};
use tower::ServiceExt;

fn create_test_state() -> AppState {
    AppState::new("test@localhost".to_string(), "test_cookie".to_string())
}

#[tokio::test]
async fn test_index_returns_html() {
    let router = create_router(create_test_state());

    let response = router
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let content_type = response
        .headers()
        .get("content-type")
        .map(|v| v.to_str().unwrap_or(""));
    assert!(content_type.unwrap_or("").contains("text/html"));
}

#[tokio::test]
async fn test_table_page_returns_html() {
    let router = create_router(create_test_state());

    let response = router
        .oneshot(
            Request::builder()
                .uri("/tables/my_table")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let content_type = response
        .headers()
        .get("content-type")
        .map(|v| v.to_str().unwrap_or(""));
    assert!(content_type.unwrap_or("").contains("text/html"));
}

#[tokio::test]
async fn test_table_page_with_url_encoded_name() {
    let router = create_router(create_test_state());

    let response = router
        .oneshot(
            Request::builder()
                .uri("/tables/my%20table%2Fwith%2Fslashes")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_unknown_route_returns_404() {
    let router = create_router(create_test_state());

    let response = router
        .oneshot(
            Request::builder()
                .uri("/nonexistent/path")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
