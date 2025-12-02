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

use crate::errors::{Error, Result};
use edp_node::Node;
use erltf::{OwnedTerm, erl_atom};
use erltf_serde::from_term;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TableType {
    Set,
    OrderedSet,
    Bag,
    DuplicateBag,
}

impl fmt::Display for TableType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TableType::Set => write!(f, "set"),
            TableType::OrderedSet => write!(f, "ordered_set"),
            TableType::Bag => write!(f, "bag"),
            TableType::DuplicateBag => write!(f, "duplicate_bag"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Protection {
    Public,
    Protected,
    Private,
}

impl fmt::Display for Protection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Protection::Public => write!(f, "public"),
            Protection::Protected => write!(f, "protected"),
            Protection::Private => write!(f, "private"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EtsTableInfo {
    pub name: String,
    pub table_type: TableType,
    pub size: u64,
    pub memory_bytes: u64,
    pub owner: String,
    pub protection: Protection,
}

#[derive(Debug, Deserialize)]
struct EtsTableInfoPartial {
    name: String,
    #[serde(rename = "type")]
    table_type: TableType,
    size: u64,
    memory: u64,
    protection: Protection,
}

fn generate_local_node_name() -> String {
    let mut rng = rand::rng();
    let suffix: u32 = rng.random_range(10000..99999);
    let host = hostname::get()
        .map(|h| h.to_string_lossy().to_string())
        .unwrap_or_else(|_| "localhost".to_string());
    format!("ets_browser_{}@{}", suffix, host)
}

async fn create_connected_node(remote_node: &str, cookie: &str) -> Result<Node> {
    let local_name = generate_local_node_name();
    let mut node = Node::new_hidden(&local_name, cookie);
    node.start(0).await?;
    node.connect(remote_node).await?;
    Ok(node)
}

async fn get_word_size(node: &Node, remote_node: &str) -> Result<u64> {
    let word_size = node
        .rpc_call(
            remote_node,
            "erlang",
            "system_info",
            vec![erl_atom!("wordsize")],
        )
        .await?;

    match word_size {
        OwnedTerm::Integer(n) if n > 0 => Ok(n as u64),
        other => Err(Error::UnexpectedResponse(format!(
            "Expected positive integer for wordsize, got: {}",
            other
        ))),
    }
}

fn parse_table_info(info_list: OwnedTerm, word_size: u64) -> Result<EtsTableInfo> {
    let owner = info_list
        .proplist_get_atom_key("owner")
        .map(|t| t.to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let info_map = info_list.to_map_recursive()?;
    let partial: EtsTableInfoPartial = from_term(&info_map)?;

    Ok(EtsTableInfo {
        name: partial.name,
        table_type: partial.table_type,
        size: partial.size,
        memory_bytes: partial.memory * word_size,
        owner,
        protection: partial.protection,
    })
}

pub async fn list_tables(remote_node: &str, cookie: &str) -> Result<Vec<EtsTableInfo>> {
    let node = create_connected_node(remote_node, cookie).await?;
    let word_size = get_word_size(&node, remote_node).await?;

    let tables_list = node.rpc_call(remote_node, "ets", "all", vec![]).await?;
    let table_refs = tables_list.try_into_list()?;

    let mut tables = Vec::with_capacity(table_refs.len());

    for table_ref in table_refs {
        let info_list = node
            .rpc_call(remote_node, "ets", "info", vec![table_ref])
            .await?;

        if info_list.is_undefined() {
            continue;
        }

        tables.push(parse_table_info(info_list, word_size)?);
    }

    tables.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(tables)
}

pub async fn dump_table(
    remote_node: &str,
    cookie: &str,
    table_name: &str,
) -> Result<Vec<OwnedTerm>> {
    let node = create_connected_node(remote_node, cookie).await?;

    let table_ref = erl_atom!(table_name);

    let info = node
        .rpc_call(remote_node, "ets", "info", vec![table_ref.clone()])
        .await?;

    if info.is_undefined() {
        return Err(Error::TableNotFound(table_name.to_string()));
    }

    let entries = node
        .rpc_call(remote_node, "ets", "tab2list", vec![table_ref])
        .await?;

    Ok(entries.try_into_list()?)
}
