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

use std::result::Result as StdResult;
use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("Node error: {0}")]
    Node(#[from] edp_node::Error),

    #[error("Term conversion error: {0}")]
    TermConversion(#[from] erltf::errors::TermConversionError),

    #[error("Deserialization error: {0}")]
    Deserialization(#[from] erltf_serde::Error),

    #[error("Cookie not found: {0}")]
    CookieNotFound(String),

    #[error("Invalid regex pattern: {0}")]
    InvalidPattern(String),

    #[error("Table not found: {0}")]
    TableNotFound(String),

    #[error("Unexpected response format: {0}")]
    UnexpectedResponse(String),
}

pub type Result<T, E = Error> = StdResult<T, E>;
