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

use clap::{Arg, Command};

pub fn clap_parser() -> Command {
    Command::new("ets-web")
        .version(env!("CARGO_PKG_VERSION"))
        .about("ETS Web UI - Browse ETS tables on running Erlang nodes")
        .arg(
            Arg::new("node")
                .long("node")
                .short('n')
                .required(true)
                .value_name("NODE")
                .help("Target Erlang node name (e.g., rabbit@localhost)"),
        )
        .arg(
            Arg::new("erlang_cookie")
                .long("erlang-cookie")
                .short('c')
                .env("ERLANG_COOKIE")
                .value_name("COOKIE")
                .help("Erlang distribution cookie (defaults to ~/.erlang.cookie)"),
        )
        .arg(
            Arg::new("host")
                .long("host")
                .short('H')
                .value_name("HOST")
                .default_value("127.0.0.1")
                .help("Host address to bind to"),
        )
        .arg(
            Arg::new("port")
                .long("port")
                .short('p')
                .value_name("PORT")
                .default_value("15692")
                .help("Port to listen on"),
        )
}
