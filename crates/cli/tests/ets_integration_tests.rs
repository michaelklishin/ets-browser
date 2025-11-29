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

mod test_helpers;

use predicates::prelude::PredicateBooleanExt;
use test_helpers::{get_test_cookie, get_test_node, output_includes, run_fails, run_succeeds};

#[test]
#[ignore]
fn test_list_tables_on_live_node() {
    let node = get_test_node();
    let cookie = get_test_cookie();

    run_succeeds([
        "tables",
        "list",
        "--node",
        &node,
        "--erlang-cookie",
        &cookie,
    ])
    .stdout(output_includes("Total:"))
    .stdout(output_includes("tables"));
}

#[test]
#[ignore]
fn test_dump_ac_table_on_live_node() {
    let node = get_test_node();
    let cookie = get_test_cookie();

    run_succeeds([
        "tables",
        "dump",
        "--node",
        &node,
        "--erlang-cookie",
        &cookie,
        "--name",
        "ac_tab",
    ])
    .stdout(output_includes("Table: ac_tab"));
}

#[test]
#[ignore]
fn test_dump_nonexistent_table() {
    let node = get_test_node();
    let cookie = get_test_cookie();

    run_fails([
        "tables",
        "dump",
        "--node",
        &node,
        "--erlang-cookie",
        &cookie,
        "--name",
        "nonexistent_table_12345",
    ])
    .stderr(output_includes("not found").or(output_includes("Table not found")));
}

#[test]
#[ignore]
fn test_memory_breakdown_on_live_node() {
    let node = get_test_node();
    let cookie = get_test_cookie();

    run_succeeds([
        "tables",
        "memory_breakdown",
        "--node",
        &node,
        "--erlang-cookie",
        &cookie,
    ])
    .stdout(output_includes("Total:"))
    .stdout(output_includes("memory"))
    .stdout(
        output_includes("KiB")
            .or(output_includes("MiB"))
            .or(output_includes("B")),
    );
}

#[test]
#[ignore]
fn test_list_tables_with_pattern_filter() {
    let node = get_test_node();
    let cookie = get_test_cookie();

    run_succeeds([
        "tables",
        "list",
        "--node",
        &node,
        "--erlang-cookie",
        &cookie,
        "--pattern",
        "^ra_",
    ])
    .stdout(output_includes("Total:"));
}

#[test]
#[ignore]
fn test_memory_breakdown_with_pattern_filter() {
    let node = get_test_node();
    let cookie = get_test_cookie();

    run_succeeds([
        "tables",
        "memory_breakdown",
        "--node",
        &node,
        "--erlang-cookie",
        &cookie,
        "--pattern",
        "rabbit_",
    ])
    .stdout(output_includes("Total:"))
    .stdout(output_includes("memory"));
}
