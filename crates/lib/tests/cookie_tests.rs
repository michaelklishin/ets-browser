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

#[test]
fn test_explicit_cookie_takes_priority() {
    let result = get_erlang_cookie(Some("explicit_cookie"), Some("env_cookie"));
    assert_eq!(result.unwrap(), "explicit_cookie");
}

#[test]
fn test_env_cookie_used_when_no_explicit() {
    let result = get_erlang_cookie(None, Some("env_cookie"));
    assert_eq!(result.unwrap(), "env_cookie");
}

#[test]
fn test_explicit_cookie_preserves_whitespace() {
    let result = get_erlang_cookie(Some("  cookie_value  "), None);
    assert_eq!(result.unwrap(), "  cookie_value  ");
}

#[test]
fn test_env_cookie_used_as_is() {
    let result = get_erlang_cookie(None, Some("env_value"));
    assert_eq!(result.unwrap(), "env_value");
}
