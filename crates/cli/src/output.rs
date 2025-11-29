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

use erltf::OwnedTerm;
use ets_lib::EtsTableInfo;
use humansize::{BINARY, format_size};
use tabled::settings::object::Rows;
use tabled::settings::{Format, Modify, Style};
use tabled::{Table, Tabled};

fn apply_table_style<T: Tabled>(rows: Vec<T>) -> String {
    Table::new(rows)
        .with(Style::modern())
        .with(Modify::new(Rows::first()).with(Format::content(|s| format!("\x1b[1m{}\x1b[0m", s))))
        .to_string()
}

#[derive(Tabled)]
struct TableRow {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Type")]
    table_type: String,
    #[tabled(rename = "Objects")]
    size: i64,
    #[tabled(rename = "Memory")]
    memory: String,
    #[tabled(rename = "Owner")]
    owner: String,
    #[tabled(rename = "Protection")]
    protection: String,
}

#[derive(Tabled)]
struct MemoryBreakdownRow {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Memory")]
    memory: String,
    #[tabled(rename = "Objects")]
    size: i64,
    #[tabled(rename = "Type")]
    table_type: String,
}

pub fn format_memory(bytes: i64) -> String {
    if bytes < 0 {
        return format!("{} bytes", bytes);
    }
    format_size(bytes as u64, BINARY)
}

pub fn print_table_list(tables: Vec<EtsTableInfo>) {
    if tables.is_empty() {
        println!("No ETS tables found.");
        return;
    }

    let count = tables.len();
    let rows: Vec<TableRow> = tables
        .into_iter()
        .map(|t| TableRow {
            name: t.name,
            table_type: t.table_type.to_string(),
            size: t.size,
            memory: format_memory(t.memory_bytes),
            owner: t.owner,
            protection: t.protection.to_string(),
        })
        .collect();

    println!("{}", apply_table_style(rows));
    println!("\nTotal: {} tables", count);
}

pub fn print_memory_breakdown(tables: Vec<EtsTableInfo>) {
    if tables.is_empty() {
        println!("No ETS tables found.");
        return;
    }

    let count = tables.len();
    let total_memory: i64 = tables.iter().map(|t| t.memory_bytes).sum();

    let rows: Vec<MemoryBreakdownRow> = tables
        .into_iter()
        .map(|t| MemoryBreakdownRow {
            name: t.name,
            memory: format_memory(t.memory_bytes),
            size: t.size,
            table_type: t.table_type.to_string(),
        })
        .collect();

    println!("{}", apply_table_style(rows));
    println!(
        "\nTotal: {} tables, {} memory",
        count,
        format_memory(total_memory)
    );
}

#[derive(Tabled)]
struct DumpRow {
    #[tabled(rename = "#")]
    index: usize,
    #[tabled(rename = "Entry")]
    entry: String,
}

pub fn print_table_dump(table_name: &str, entries: Vec<OwnedTerm>) {
    if entries.is_empty() {
        println!("Table '{}' is empty.", table_name);
        return;
    }

    let count = entries.len();
    let rows: Vec<DumpRow> = entries
        .into_iter()
        .enumerate()
        .map(|(i, entry)| DumpRow {
            index: i + 1,
            entry: entry.to_string(),
        })
        .collect();

    println!("Table: {}", table_name);
    println!("{}", apply_table_style(rows));
    println!("\nTotal: {} entries", count);
}
