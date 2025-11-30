# (Example) ets Browser in Rust: both CLI and Web UI

This tiny project is an example of how [`edp-rs`](https://github.com/michaelklishin/edp-rs) can be used
to interact with Erlang/Elixir/BEAM-based systems from Rust.

## Running

Since this is an example, all the commands below are `cargo run`-based.

### List All ETS Tables

```shell
cargo run --bin 'ets-cli' '--' tables list --node rabbit@sunnyside
```

# Display Contents of the `rabbit_registry` ETS Table on a RabbitMQ Node

```shell
cargo run --bin 'ets-cli' '--' tables dump --node rabbit@sunnyside --name rabbit_registry
```

### Produce a Memory Breakdown for Ra ETS Tables

```shell
cargo run --bin 'ets-cli' '--' tables memory_breakdown --pattern '^ra_' --node rabbit@sunnyside
```

### Start a Web Browser

```shell
cargo run --bin ets-web -- --node rabbit@sunnyside --port 3458
```

then navigate to [`http://localhost:3458`](http://localhost:3458).


## License

This software is dual-licensed under the MIT License and the Apache License, Version 2.0.


## Copyright

(c) 2025-2026 Michael S. Klishin and Contributors.
