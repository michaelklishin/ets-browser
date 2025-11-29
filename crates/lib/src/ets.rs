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
use erltf::OwnedTerm;
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
    pub size: i64,
    pub memory_bytes: i64,
    pub owner: String,
    pub protection: Protection,
}

#[derive(Debug, Deserialize)]
struct EtsTableInfoPartial {
    name: String,
    #[serde(rename = "type")]
    table_type: TableType,
    size: i64,
    memory: i64,
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

fn unwrap_rex_response(response: OwnedTerm) -> Result<OwnedTerm> {
    match response {
        OwnedTerm::Tuple(mut elements) if elements.len() == 2 => {
            if let Some(atom) = elements[0].as_atom()
                && atom.as_ref() == "rex"
            {
                return Ok(elements.pop().unwrap());
            }
            Err(Error::UnexpectedResponse(format!(
                "Expected {{rex, Result}}, got: {:?}",
                elements
            )))
        }
        other => Err(Error::UnexpectedResponse(format!(
            "Expected {{rex, Result}} tuple, got: {}",
            other
        ))),
    }
}

async fn get_word_size(node: &Node, remote_node: &str) -> Result<i64> {
    let response = node
        .rpc_call(
            remote_node,
            "erlang",
            "system_info",
            vec![OwnedTerm::atom("wordsize")],
        )
        .await?;

    let word_size = unwrap_rex_response(response)?;

    match word_size {
        OwnedTerm::Integer(n) => Ok(n),
        other => Err(Error::UnexpectedResponse(format!(
            "Expected integer for wordsize, got: {}",
            other
        ))),
    }
}

fn parse_table_info(info_list: OwnedTerm, word_size: i64) -> Result<EtsTableInfo> {
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

    let response = node.rpc_call(remote_node, "ets", "all", vec![]).await?;
    let tables_list = unwrap_rex_response(response)?;

    let table_refs: Vec<OwnedTerm> = match tables_list {
        OwnedTerm::List(refs) => refs,
        OwnedTerm::Nil => vec![],
        other => {
            return Err(Error::UnexpectedResponse(format!(
                "Expected list of table references, got: {}",
                other
            )));
        }
    };

    let mut tables = Vec::with_capacity(table_refs.len());

    for table_ref in table_refs {
        let info_response = node
            .rpc_call(remote_node, "ets", "info", vec![table_ref])
            .await?;

        let info_list = unwrap_rex_response(info_response)?;

        if let OwnedTerm::Atom(atom) = &info_list
            && atom.as_ref() == "undefined"
        {
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

    let table_ref = OwnedTerm::atom(table_name);

    let info_response = node
        .rpc_call(remote_node, "ets", "info", vec![table_ref.clone()])
        .await?;

    let info = unwrap_rex_response(info_response)?;
    if let OwnedTerm::Atom(atom) = &info
        && atom.as_ref() == "undefined"
    {
        return Err(Error::TableNotFound(table_name.to_string()));
    }

    let response = node
        .rpc_call(remote_node, "ets", "tab2list", vec![table_ref])
        .await?;

    let entries = unwrap_rex_response(response)?;

    match entries {
        OwnedTerm::List(list) => Ok(list),
        OwnedTerm::Nil => Ok(vec![]),
        other => Err(Error::UnexpectedResponse(format!(
            "Expected list of entries, got: {}",
            other
        ))),
    }
}
