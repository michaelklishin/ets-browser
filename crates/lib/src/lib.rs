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

pub mod errors;
pub mod ets;

use regex::Regex;
use std::fs;
use std::path::PathBuf;

pub use errors::{Error, Result};
pub use ets::{EtsTableInfo, Protection, TableType, dump_table, list_tables};

pub fn filter_tables(tables: Vec<EtsTableInfo>, pattern: Option<&Regex>) -> Vec<EtsTableInfo> {
    match pattern {
        Some(re) => tables
            .into_iter()
            .filter(|t| re.is_match(&t.name))
            .collect(),
        None => tables,
    }
}

pub fn get_erlang_cookie(
    explicit_cookie: Option<&str>,
    env_cookie: Option<&str>,
) -> Result<String> {
    if let Some(cookie) = explicit_cookie {
        return Ok(cookie.to_string());
    }

    if let Some(cookie) = env_cookie {
        return Ok(cookie.to_string());
    }

    let cookie_path = dirs::home_dir()
        .map(|h| h.join(".erlang.cookie"))
        .unwrap_or_else(|| PathBuf::from("~/.erlang.cookie"));

    fs::read_to_string(&cookie_path)
        .map(|s| s.trim().to_string())
        .map_err(|_| {
            Error::CookieNotFound(format!(
                "Could not read {}. Provide --erlang-cookie or set ERLANG_COOKIE",
                cookie_path.display()
            ))
        })
}
