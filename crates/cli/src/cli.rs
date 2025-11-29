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
    let tables_group = Command::new("tables")
        .about("ETS table operations")
        .subcommand_required(true)
        .subcommands(tables_subcommands());

    Command::new("ets-cli")
        .version(env!("CARGO_PKG_VERSION"))
        .about("ETS CLI - Inspect ETS tables on running Erlang nodes")
        .subcommand_required(true)
        .subcommand(tables_group)
}

fn node_arg() -> Arg {
    Arg::new("node")
        .long("node")
        .short('n')
        .required(true)
        .value_name("NODE")
        .help("Target Erlang node name (e.g., rabbit@localhost)")
}

fn cookie_arg() -> Arg {
    Arg::new("erlang_cookie")
        .long("erlang-cookie")
        .short('c')
        .env("ERLANG_COOKIE")
        .value_name("COOKIE")
        .help("Erlang distribution cookie (defaults to ~/.erlang.cookie)")
}

fn pattern_arg() -> Arg {
    Arg::new("pattern")
        .long("pattern")
        .short('p')
        .value_name("REGEX")
        .help("Filter table names by regex pattern")
}

fn tables_subcommands() -> Vec<Command> {
    let list_cmd = Command::new("list")
        .about("List all ETS tables on the node")
        .arg(node_arg())
        .arg(cookie_arg())
        .arg(pattern_arg());

    let memory_breakdown_cmd = Command::new("memory_breakdown")
        .about("List ETS tables sorted by memory usage (descending)")
        .arg(node_arg())
        .arg(cookie_arg())
        .arg(pattern_arg());

    let dump_cmd = Command::new("dump")
        .about("Dump contents of an ETS table")
        .arg(node_arg())
        .arg(cookie_arg())
        .arg(
            Arg::new("name")
                .long("name")
                .required(true)
                .value_name("TABLE")
                .help("Name of the ETS table to dump"),
        );

    vec![list_cmd, memory_breakdown_cmd, dump_cmd]
}
