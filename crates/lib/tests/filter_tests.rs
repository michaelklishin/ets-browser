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

use ets_lib::{EtsTableInfo, Protection, TableType, filter_tables};
use regex::Regex;

fn make_table(name: &str) -> EtsTableInfo {
    EtsTableInfo {
        name: name.to_string(),
        table_type: TableType::Set,
        size: 0,
        memory_bytes: 0,
        owner: "<0.0.0>".to_string(),
        protection: Protection::Public,
    }
}

#[test]
fn test_filter_with_no_pattern_returns_all() {
    let tables = vec![
        make_table("rabbit_queue"),
        make_table("ra_log"),
        make_table("mnesia_schema"),
    ];

    let result = filter_tables(tables, None);
    assert_eq!(result.len(), 3);
}

#[test]
fn test_filter_with_prefix_pattern() {
    let tables = vec![
        make_table("rabbit_queue"),
        make_table("rabbit_exchange"),
        make_table("ra_log"),
        make_table("mnesia_schema"),
    ];

    let pattern = Regex::new("^rabbit_").unwrap();
    let result = filter_tables(tables, Some(&pattern));

    assert_eq!(result.len(), 2);
    assert!(result.iter().all(|t| t.name.starts_with("rabbit_")));
}

#[test]
fn test_filter_with_suffix_pattern() {
    let tables = vec![
        make_table("rabbit_queue"),
        make_table("message_queue"),
        make_table("ra_log"),
    ];

    let pattern = Regex::new("_queue$").unwrap();
    let result = filter_tables(tables, Some(&pattern));

    assert_eq!(result.len(), 2);
    assert!(result.iter().all(|t| t.name.ends_with("_queue")));
}

#[test]
fn test_filter_with_contains_pattern() {
    let tables = vec![
        make_table("rabbit_queue"),
        make_table("ra_log_wal"),
        make_table("mnesia_schema"),
    ];

    let pattern = Regex::new("log").unwrap();
    let result = filter_tables(tables, Some(&pattern));

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].name, "ra_log_wal");
}

#[test]
fn test_filter_with_no_matches_returns_empty() {
    let tables = vec![make_table("rabbit_queue"), make_table("ra_log")];

    let pattern = Regex::new("^nonexistent").unwrap();
    let result = filter_tables(tables, Some(&pattern));

    assert!(result.is_empty());
}

#[test]
fn test_filter_empty_input_returns_empty() {
    let tables: Vec<EtsTableInfo> = vec![];

    let pattern = Regex::new(".*").unwrap();
    let result = filter_tables(tables, Some(&pattern));

    assert!(result.is_empty());
}

#[test]
fn test_filter_with_case_sensitive_pattern() {
    let tables = vec![make_table("Rabbit_Queue"), make_table("rabbit_queue")];

    let pattern = Regex::new("^rabbit").unwrap();
    let result = filter_tables(tables, Some(&pattern));

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].name, "rabbit_queue");
}

#[test]
fn test_filter_with_case_insensitive_pattern() {
    let tables = vec![make_table("Rabbit_Queue"), make_table("rabbit_queue")];

    let pattern = Regex::new("(?i)^rabbit").unwrap();
    let result = filter_tables(tables, Some(&pattern));

    assert_eq!(result.len(), 2);
}
