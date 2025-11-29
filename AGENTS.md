# Instructions for AI Agents

## Overview

This library implements an [Erlang ETS table](https://www.erlang.org/docs/27/apps/stdlib/ets.html) browsing tool on top of [`edp-rs`](https://github.com/michaelklishin/edp-rs).





## Repository Layout

This is a Rust workspace managed by `cargo`. The repository layout is as follows:

 * `Cargo.toml`: the workspace manifest file
 * `crates/cli`: a CLI version
 * `crates/web`: a Web version

### The CLI

The CLI version of this tool can

 * List ETS tables on target node: `tables list`
 * List ETS tables on target node sorted by memory footprint: `tables memory_breakdown`
 * List ETS table contents (objects): `tables dump`

### The Web UI

TBD


## Build System

 * To build the workspace, run `cargo build --all`
 * To run the tests, run `cargo nextest run --all`


## Target Rust Version

 * This tool targets cutting edge Rust (currently `1.91.0`)


## Key Dependencies

 * `edp-rs` for parsing log files
 * `clap` for the CLI interface


## Rust Code Style

 * Use top-level `use` statements (imports) to fully-qualified names, e.g. `Display` or `fmt::Display` with a `use` statement, to `std::fmt::Display`
 * Never use function-local `use` statements (imports)
 * Add tests to the modules under `tests`, never in the implementation files
 * At the end of each task, run `cargo fmt --all`
 * At the end of each task, run `cargo clippy --all` and fix any warnings it might emit

## Comments

 * Only add very important comments, both in tests and in the implementation

## Git Instructions

 * Never add yourself to the list of commit co-authors

## Style Guide

 * Never add full stops to Markdown list items
