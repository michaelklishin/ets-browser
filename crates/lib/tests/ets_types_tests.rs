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
use erltf_serde::from_term;
use ets_lib::{EtsTableInfo, Protection, TableType};
use std::collections::BTreeMap;

#[test]
fn test_table_type_deserialization_from_atoms() {
    let term = OwnedTerm::atom("set");
    let table_type: TableType = from_term(&term).unwrap();
    assert_eq!(table_type, TableType::Set);

    let term = OwnedTerm::atom("ordered_set");
    let table_type: TableType = from_term(&term).unwrap();
    assert_eq!(table_type, TableType::OrderedSet);

    let term = OwnedTerm::atom("bag");
    let table_type: TableType = from_term(&term).unwrap();
    assert_eq!(table_type, TableType::Bag);

    let term = OwnedTerm::atom("duplicate_bag");
    let table_type: TableType = from_term(&term).unwrap();
    assert_eq!(table_type, TableType::DuplicateBag);
}

#[test]
fn test_protection_deserialization_from_atoms() {
    let term = OwnedTerm::atom("public");
    let protection: Protection = from_term(&term).unwrap();
    assert_eq!(protection, Protection::Public);

    let term = OwnedTerm::atom("protected");
    let protection: Protection = from_term(&term).unwrap();
    assert_eq!(protection, Protection::Protected);

    let term = OwnedTerm::atom("private");
    let protection: Protection = from_term(&term).unwrap();
    assert_eq!(protection, Protection::Private);
}

#[test]
fn test_table_type_display() {
    assert_eq!(format!("{}", TableType::Set), "set");
    assert_eq!(format!("{}", TableType::OrderedSet), "ordered_set");
    assert_eq!(format!("{}", TableType::Bag), "bag");
    assert_eq!(format!("{}", TableType::DuplicateBag), "duplicate_bag");
}

#[test]
fn test_protection_display() {
    assert_eq!(format!("{}", Protection::Public), "public");
    assert_eq!(format!("{}", Protection::Protected), "protected");
    assert_eq!(format!("{}", Protection::Private), "private");
}

#[test]
fn test_ets_table_info_from_map() {
    let mut map = BTreeMap::new();
    map.insert(OwnedTerm::atom("name"), OwnedTerm::atom("test_table"));
    map.insert(OwnedTerm::atom("type"), OwnedTerm::atom("set"));
    map.insert(OwnedTerm::atom("size"), OwnedTerm::Integer(100));
    map.insert(OwnedTerm::atom("memory"), OwnedTerm::Integer(1024));
    map.insert(OwnedTerm::atom("protection"), OwnedTerm::atom("public"));
    map.insert(
        OwnedTerm::atom("owner"),
        OwnedTerm::String("<0.123.0>".to_string()),
    );

    #[derive(serde::Deserialize)]
    struct PartialInfo {
        name: String,
        #[serde(rename = "type")]
        table_type: TableType,
        size: u64,
        memory: u64,
        protection: Protection,
    }

    let term = OwnedTerm::Map(map);
    let info: PartialInfo = from_term(&term).unwrap();

    assert_eq!(info.name, "test_table");
    assert_eq!(info.table_type, TableType::Set);
    assert_eq!(info.size, 100);
    assert_eq!(info.memory, 1024);
    assert_eq!(info.protection, Protection::Public);
}

#[test]
fn test_ets_table_info_serialization() {
    let info = EtsTableInfo {
        name: "my_table".to_string(),
        table_type: TableType::OrderedSet,
        size: 500,
        memory_bytes: 8192,
        owner: "<0.99.0>".to_string(),
        protection: Protection::Protected,
    };

    let json = serde_json::to_string(&info).unwrap();
    let deserialized: EtsTableInfo = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.name, info.name);
    assert_eq!(deserialized.table_type, info.table_type);
    assert_eq!(deserialized.size, info.size);
    assert_eq!(deserialized.memory_bytes, info.memory_bytes);
    assert_eq!(deserialized.owner, info.owner);
    assert_eq!(deserialized.protection, info.protection);
}

#[test]
fn test_table_type_deserialization_rejects_invalid_atom() {
    let term = OwnedTerm::atom("invalid_type");
    let result: Result<TableType, _> = from_term(&term);
    assert!(result.is_err());
}

#[test]
fn test_protection_deserialization_rejects_invalid_atom() {
    let term = OwnedTerm::atom("invalid_protection");
    let result: Result<Protection, _> = from_term(&term);
    assert!(result.is_err());
}

#[test]
fn test_table_type_deserialization_rejects_non_atom() {
    let term = OwnedTerm::Integer(42);
    let result: Result<TableType, _> = from_term(&term);
    assert!(result.is_err());
}

#[test]
fn test_protection_deserialization_rejects_non_atom() {
    let term = OwnedTerm::Integer(42);
    let result: Result<Protection, _> = from_term(&term);
    assert!(result.is_err());
}
