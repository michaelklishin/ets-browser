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

use clap::ArgMatches;
use ets_cli::cli::clap_parser;
use ets_cli::output;
use ets_cli::{Error, Result, dump_table, filter_tables, get_erlang_cookie, list_tables};
use regex::Regex;
use std::io::stderr;
use std::process::exit;
use sysexits::ExitCode;

fn init_logging() {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}] {}",
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Warn)
        .chain(stderr())
        .apply()
        .unwrap_or_else(|e| {
            eprintln!("Failed to initialize logging: {}", e);
            exit(1);
        });
}

#[tokio::main]
async fn main() {
    init_logging();

    let matches = clap_parser().get_matches();

    let exit_code = match matches.subcommand() {
        Some(("tables", sub_matches)) => handle_tables_command(sub_matches).await,
        _ => {
            eprintln!("Unknown command. Use --help for usage information.");
            ExitCode::Usage
        }
    };

    exit(exit_code as i32);
}

async fn handle_tables_command(args: &ArgMatches) -> ExitCode {
    match args.subcommand() {
        Some(("list", sub_matches)) => handle_list_command(sub_matches).await,
        Some(("memory_breakdown", sub_matches)) => {
            handle_memory_breakdown_command(sub_matches).await
        }
        Some(("dump", sub_matches)) => handle_dump_command(sub_matches).await,
        _ => {
            eprintln!("Unknown tables subcommand. Use --help for usage information.");
            ExitCode::Usage
        }
    }
}

fn handle_result(result: Result<()>, operation: &str) -> ExitCode {
    match result {
        Ok(()) => ExitCode::Ok,
        Err(e) => {
            log::error!("Failed to {}: {}", operation, e);
            eprintln!("Error: {}", e);
            ExitCode::Software
        }
    }
}

async fn handle_list_command(args: &ArgMatches) -> ExitCode {
    handle_result(do_list_tables(args).await, "list tables")
}

async fn handle_memory_breakdown_command(args: &ArgMatches) -> ExitCode {
    handle_result(memory_breakdown(args).await, "get memory breakdown")
}

async fn handle_dump_command(args: &ArgMatches) -> ExitCode {
    handle_result(do_dump_table(args).await, "dump table")
}

fn resolve_erlang_cookie(args: &ArgMatches) -> Result<String> {
    let explicit = args.get_one::<String>("erlang_cookie").map(|s| s.as_str());
    get_erlang_cookie(explicit, None)
}

fn get_pattern_filter(args: &ArgMatches) -> Result<Option<Regex>> {
    args.get_one::<String>("pattern")
        .map(|p| Regex::new(p).map_err(|e| Error::InvalidPattern(e.to_string())))
        .transpose()
}

async fn do_list_tables(args: &ArgMatches) -> Result<()> {
    let node = args
        .get_one::<String>("node")
        .expect("node is required by clap");
    let cookie = resolve_erlang_cookie(args)?;
    let pattern = get_pattern_filter(args)?;

    let tables = list_tables(node, &cookie).await?;
    let tables = filter_tables(tables, pattern.as_ref());
    output::print_table_list(tables);

    Ok(())
}

async fn memory_breakdown(args: &ArgMatches) -> Result<()> {
    let node = args
        .get_one::<String>("node")
        .expect("node is required by clap");
    let cookie = resolve_erlang_cookie(args)?;
    let pattern = get_pattern_filter(args)?;

    let tables = list_tables(node, &cookie).await?;
    let mut tables = filter_tables(tables, pattern.as_ref());
    tables.sort_by(|a, b| b.memory_bytes.cmp(&a.memory_bytes));
    output::print_memory_breakdown(tables);

    Ok(())
}

async fn do_dump_table(args: &ArgMatches) -> Result<()> {
    let node = args
        .get_one::<String>("node")
        .expect("node is required by clap");
    let cookie = resolve_erlang_cookie(args)?;
    let table_name = args
        .get_one::<String>("name")
        .expect("name is required by clap");

    let entries = dump_table(node, &cookie, table_name).await?;
    output::print_table_dump(table_name, entries);

    Ok(())
}
