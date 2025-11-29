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

use ets_lib::get_erlang_cookie;
use ets_web::cli::clap_parser;
use ets_web::server::{AppState, create_router};
use std::process::exit;
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

fn init_logging() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();
}

#[tokio::main]
async fn main() {
    init_logging();

    let matches = clap_parser().get_matches();

    let node = matches
        .get_one::<String>("node")
        .expect("node is required by clap")
        .clone();

    let cookie = match get_erlang_cookie(
        matches
            .get_one::<String>("erlang_cookie")
            .map(|s| s.as_str()),
        None,
    ) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1);
        }
    };

    let host = matches
        .get_one::<String>("host")
        .expect("host has default value")
        .clone();

    let port = matches
        .get_one::<String>("port")
        .expect("port has default value")
        .clone();

    let state = AppState::new(node.clone(), cookie);
    let router = create_router(state);

    let addr = format!("{}:{}", host, port);
    tracing::info!("Starting ETS Web UI on http://{}", addr);
    tracing::info!("Connected to Erlang node: {}", node);

    let listener = match TcpListener::bind(&addr).await {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Failed to bind to {}: {}", addr, e);
            exit(1);
        }
    };

    if let Err(e) = axum::serve(listener, router).await {
        eprintln!("Server error: {}", e);
        exit(1);
    }
}
