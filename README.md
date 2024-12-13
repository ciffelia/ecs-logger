# ecs-logger

[![CI status][ci badge]][ci link]
[![crate version][crates.io badge]][crates.io link]
[![docs online][docs badge]][docs link]
[![MIT or Apache 2.0 Licenses][license badge]][license link]

A Rust logger compatible with [Elastic Common Schema (ECS) Logging](https://www.elastic.co/guide/en/ecs-logging/overview/current/intro.html).

## Features

- Configurable via the RUST_LOG environment variable.
  - Uses [env_logger][env_logger docs] under the hood.
  - All logging is disabled except for the `error` level by default.
- Logs are written to stderr by default.

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
log = "0.4"
ecs-logger = "1"
```

## Documentation

[Available at docs.rs][docs link]

## Example

In the following examples we assume the binary is `./example`.

### Basic logging

```rust
use log::{debug, error};

ecs_logger::init();

debug!(
    "this is a debug {}, which is NOT printed by default",
    "message"
);
error!("this is printed by default");
```

```bash
$ ./example
{"@timestamp":"2021-11-26T15:25:22.321002600Z","log.level":"ERROR","message":"this is printed by default","ecs.version":"1.12.1","log.origin":{"file":{"line":13,"name":"example.rs"},"rust":{"target":"example::tests","module_path":"example::tests","file_path":"tests/example.rs"}}}
```

```bash
$ RUST_LOG=debug ./example
{"@timestamp":"2021-11-26T15:26:13.524069Z","log.level":"DEBUG","message":"this is a debug message, which is NOT printed by default","ecs.version":"1.12.1","log.origin":{"file":{"line":9,"name":"example.rs"},"rust":{"target":"example::tests","module_path":"example::tests","file_path":"tests/example.rs"}}}
{"@timestamp":"2021-11-26T15:26:13.524193100Z","log.level":"ERROR","message":"this is printed by default","ecs.version":"1.12.1","log.origin":{"file":{"line":13,"name":"example.rs"},"rust":{"target":"example::tests","module_path":"example::tests","file_path":"tests/example.rs"}}}
```

More filtering config examples are available at [env_logger’s documentation][env_logger docs].

### Extra Fields

You can add extra fields to the log output by using the `extra_fields` module.

```rust
use ecs_logger::extra_fields;
use serde::Serialize;

#[derive(Serialize)]
struct MyExtraFields {
  my_field: String,
}

ecs_logger::init();

extra_fields::set_extra_fields(MyExtraFields {
  my_field: "my_value".to_string(),
}).unwrap();

log::error!("Hello {}!", "world");
log::info!("Goodbye {}!", "world");

extra_fields::clear_extra_fields();
```

### Custom logging

You need to add [`env_logger`][env_logger docs] to your `Cargo.toml` for the following examples.

```toml
[dependencies]
log = "0.4"
env_logger = "0.9"
ecs-logger = "1"
```

#### Write to stdout

```rust
use log::info;

// Initialize custom logger
env_logger::builder()
    .format(ecs_logger::format) // Configure ECS logger
    .target(env_logger::Target::Stdout) // Write to stdout
    .init();

info!("Hello {}!", "world");
```

#### Configure log filters

```rust
use log::info;

// Initialize custom logger
env_logger::builder()
    .parse_filters("info,my_app=debug") // Set filters
    .format(ecs_logger::format) // Configure ECS logger
    .init();

info!("Hello {}!", "world");
```

## Default log fields

```json
{
    "@timestamp": "2021-11-26T15:25:22.321002600Z",
    "log.level": "ERROR",
    "message": "this is printed by default",
    "ecs.version": "1.12.1",
    "log.origin": {
        "file": {
            "line": 13,
            "name": "example.rs"
        },
        "rust": {
            "target": "example::tests",
            "module_path": "example::tests",
            "file_path": "tests/example.rs"
        }
    }
}
```

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Acknowledgment

The implementation of this software is based on [env_logger](https://github.com/env-logger-rs/env_logger), which is dual licenced as well.

[ci badge]: https://github.com/ciffelia/ecs-logger/actions/workflows/ci.yaml/badge.svg
[ci link]: https://github.com/ciffelia/ecs-logger/actions/workflows/ci.yaml

[crates.io badge]: https://img.shields.io/crates/v/ecs-logger
[crates.io link]: https://crates.io/crates/ecs-logger

[docs badge]: https://img.shields.io/badge/docs-online-green
[docs link]: https://docs.rs/ecs-logger

[license badge]: https://img.shields.io/badge/license-MIT%20or%20Apache%202.0-blue
[license link]: #license

[env_logger docs]: https://docs.rs/env_logger
