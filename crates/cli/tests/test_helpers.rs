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
#![allow(dead_code)]

use std::env;
use std::ffi::OsStr;
use std::fs;
use std::process::Command;

use assert_cmd::assert::Assert;
use assert_cmd::prelude::*;
use predicates::prelude::predicate;

const DEFAULT_NODE: &str = "rabbit@localhost";

pub fn get_test_node() -> String {
    env::var("TEST_NODE").unwrap_or_else(|_| DEFAULT_NODE.to_string())
}

pub fn get_test_cookie() -> String {
    env::var("ERLANG_COOKIE").unwrap_or_else(|_| {
        dirs::home_dir()
            .and_then(|home| fs::read_to_string(home.join(".erlang.cookie")).ok())
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| "secret".to_string())
    })
}

pub fn run_succeeds<I, S>(args: I) -> Assert
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("ets-cli"));
    cmd.args(args).assert().success()
}

pub fn run_fails<I, S>(args: I) -> Assert
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("ets-cli"));
    cmd.args(args).assert().failure()
}

pub fn output_includes(content: &str) -> predicates::str::ContainsPredicate {
    predicate::str::contains(content)
}
