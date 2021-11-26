# ecs-logger

[![CI status][ci badge]][ci link]
[![MIT or Apache 2.0 Licenses][license badge]][license link]

A Rust logger compatible with [Elastic Common Schema (ECS) Logging](https://www.elastic.co/guide/en/ecs-logging/overview/current/intro.html).

## Features

- Configurable via the RUST_LOG environment variable.
  - Uses [env_logger][env_logger docs] under the hood.
  - By default, all logging is disabled except for the error level.
- By default logs are written to stderr.

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

More filtering config examples are available at [env_logger][env_logger docs]’s documentation.

### Write to stdout

```rust
use log::info;

// Initialize custom logger
env_logger::builder()
    .format(ecs_logger::format) // Configure ECS logger
    .target(env_logger::Target::Stdout) // Write to stdout
    .init();

info!("Hello {}!", "world");
```

### Configure log filters

```rust
use log::info;

// Initialize custom logger
env_logger::builder()
    .parse_filters("info,my_app=debug") // Set filters
    .format(ecs_logger::format) // Configure ECS logger
    .init();

info!("Hello {}!", "world");
```

## Log fields

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

## Acknowledgments

The implementation of this software is based on [env_logger](https://github.com/env-logger-rs/env_logger), which is dual licenced as well.

[ci badge]: https://github.com/ciffelia/ecs-logger/workflows/CI/badge.svg?branch=main
[ci link]: https://github.com/ciffelia/ecs-logger/actions?query=workflow%3ACI+branch%3Amain

[license badge]: https://img.shields.io/badge/license-MIT%20or%20Apache%202.0-blue
[license link]: #license

[env_logger docs]: https://docs.rs/env_logger
