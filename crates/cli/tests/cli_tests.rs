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
use test_helpers::{output_includes, run_fails, run_succeeds};

#[test]
fn test_help_displays_usage() {
    run_succeeds(["--help"])
        .stdout(output_includes("ETS CLI"))
        .stdout(output_includes("tables"));
}

#[test]
fn test_tables_list_requires_node() {
    run_fails(["tables", "list", "--erlang-cookie", "test"])
        .stderr(output_includes("--node").or(output_includes("required")));
}

#[test]
fn test_tables_dump_requires_name() {
    run_fails([
        "tables",
        "dump",
        "--node",
        "test@localhost",
        "--erlang-cookie",
        "test",
    ])
    .stderr(output_includes("--name").or(output_includes("required")));
}

#[test]
fn test_tables_help_shows_subcommands() {
    run_succeeds(["tables", "--help"])
        .stdout(output_includes("list"))
        .stdout(output_includes("memory_breakdown"))
        .stdout(output_includes("dump"));
}

#[test]
fn test_tables_memory_breakdown_requires_node() {
    run_fails(["tables", "memory_breakdown", "--erlang-cookie", "test"])
        .stderr(output_includes("--node").or(output_includes("required")));
}

#[test]
fn test_tables_list_help_shows_pattern_option() {
    run_succeeds(["tables", "list", "--help"]).stdout(output_includes("--pattern"));
}

#[test]
fn test_tables_memory_breakdown_help_shows_pattern_option() {
    run_succeeds(["tables", "memory_breakdown", "--help"]).stdout(output_includes("--pattern"));
}

#[test]
fn test_invalid_regex_pattern_fails() {
    run_fails([
        "tables",
        "list",
        "--node",
        "test@localhost",
        "--erlang-cookie",
        "test",
        "--pattern",
        "[invalid(regex",
    ])
    .stderr(
        output_includes("Invalid")
            .or(output_includes("regex"))
            .or(output_includes("pattern")),
    );
}
