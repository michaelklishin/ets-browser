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
        .stdout(output_includes("ETS Web UI"))
        .stdout(output_includes("--node"));
}

#[test]
fn test_version_displays_version() {
    run_succeeds(["--version"]).stdout(output_includes(env!("CARGO_PKG_VERSION")));
}

#[test]
fn test_requires_node_argument() {
    run_fails(["--erlang-cookie", "test"])
        .stderr(output_includes("--node").or(output_includes("required")));
}

#[test]
fn test_help_shows_host_option() {
    run_succeeds(["--help"])
        .stdout(output_includes("--host"))
        .stdout(output_includes("127.0.0.1"));
}

#[test]
fn test_help_shows_port_option() {
    run_succeeds(["--help"])
        .stdout(output_includes("--port"))
        .stdout(output_includes("15692"));
}

#[test]
fn test_help_shows_cookie_option() {
    run_succeeds(["--help"])
        .stdout(output_includes("--erlang-cookie"))
        .stdout(output_includes("ERLANG_COOKIE"));
}
