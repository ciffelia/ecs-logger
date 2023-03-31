//! A logger compatible with [Elastic Common Schema (ECS) Logging](https://www.elastic.co/guide/en/ecs-logging/overview/current/intro.html).
//!
//! ## Features
//!
//! - Configurable via the `RUST_LOG` environment variable.
//!   - Uses [env_logger] under the hood.
//!   - **All logging is disabled except for the `error` level by default.**
//! - Logs are written to stderr by default.
//!
//! ## Installation
//!
//! Add the following to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! log = "0.4"
//! ecs-logger = "1"
//! ```
//!
//! ## Example
//!
//! In the following examples we assume the binary is `./example`.
//!
//! ### Basic logging
//!
//! ```
//! use log::{debug, error};
//!
//! ecs_logger::init();
//!
//! debug!(
//!     "this is a debug {}, which is NOT printed by default",
//!     "message"
//! );
//! error!("this is printed by default");
//! ```
//!
//! ```bash
//! $ ./example
//! {"@timestamp":"2021-11-26T15:25:22.321002600Z","log.level":"ERROR","message":"this is printed by default","ecs.version":"1.12.1","log.origin":{"file":{"line":13,"name":"example.rs"},"rust":{"target":"example::tests","module_path":"example::tests","file_path":"tests/example.rs"}}}
//! ```
//!
//! ```bash
//! $ RUST_LOG=debug ./example
//! {"@timestamp":"2021-11-26T15:26:13.524069Z","log.level":"DEBUG","message":"this is a debug message, which is NOT printed by default","ecs.version":"1.12.1","log.origin":{"file":{"line":9,"name":"example.rs"},"rust":{"target":"example::tests","module_path":"example::tests","file_path":"tests/example.rs"}}}
//! {"@timestamp":"2021-11-26T15:26:13.524193100Z","log.level":"ERROR","message":"this is printed by default","ecs.version":"1.12.1","log.origin":{"file":{"line":13,"name":"example.rs"},"rust":{"target":"example::tests","module_path":"example::tests","file_path":"tests/example.rs"}}}
//! ```
//!
//! More filtering config examples are available at [`env_logger`]'s documentation.
//!
//! ### Extra Fields
//!
//! You can add extra fields to the log output by using the [`extra_fields`] module.
//!
//! ```
//! use ecs_logger::extra_fields;
//! use serde::Serialize;
//!
//! #[derive(Serialize)]
//! struct MyExtraFields {
//!   my_field: String,
//! }
//!
//! ecs_logger::init();
//!
//! extra_fields::set_extra_fields(MyExtraFields {
//!   my_field: "my_value".to_string(),
//! }).unwrap();
//!
//! log::error!("Hello {}!", "world");
//! log::info!("Goodbye {}!", "world");
//!
//! extra_fields::clear_extra_fields();
//! ```
//!
//! ### Custom logging
//!
//! You need to add [`env_logger`] to your `Cargo.toml` for the following examples.
//!
//! ```toml
//! [dependencies]
//! log = "0.4"
//! env_logger = "0.9"
//! ecs-logger = "1"
//! ```
//!
//! #### Write to stdout
//!
//! ```
//! use log::info;
//!
//! // Initialize custom logger
//! env_logger::builder()
//!     .format(ecs_logger::format) // Configure ECS logger
//!     .target(env_logger::Target::Stdout) // Write to stdout
//!     .init();
//!
//! info!("Hello {}!", "world");
//! ```
//!
//! #### Configure log filters
//!
//! ```
//! use log::info;
//!
//! // Initialize custom logger
//! env_logger::builder()
//!     .parse_filters("info,my_app=debug") // Set filters
//!     .format(ecs_logger::format) // Configure ECS logger
//!     .init();
//!
//! info!("Hello {}!", "world");
//! ```
//!
//! ## Default log fields
//!
//! ```json
//! {
//!     "@timestamp": "2021-11-26T15:25:22.321002600Z",
//!     "log.level": "ERROR",
//!     "message": "this is printed by default",
//!     "ecs.version": "1.12.1",
//!     "log.origin": {
//!         "file": {
//!             "line": 13,
//!             "name": "example.rs"
//!         },
//!         "rust": {
//!             "target": "example::tests",
//!             "module_path": "example::tests",
//!             "file_path": "tests/example.rs"
//!         }
//!     }
//! }
//! ```

pub mod ecs;
pub mod extra_fields;
mod timestamp;

use ecs::Event;
use extra_fields::merge_extra_fields;
use std::borrow::BorrowMut;

/// Initializes the global logger with an instance of [`env_logger::Logger`] with ECS-Logging formatting.
///
/// This should be called early in the execution of a Rust program. Any log events that occur before initialization will be ignored.
///
/// # Panics
///
/// This function will panic if it is called more than once, or if another library has already initialized a global logger.
///
/// # Example
///
/// ```
/// use log::error;
///
/// error!("this is NOT logged");
///
/// ecs_logger::init();
///
/// error!("this is logged");
/// ```
pub fn init() {
    try_init().expect("ecs_logger::init should not be called after logger initialized");
}

/// Attempts to initialize the global logger with an instance of [`env_logger::Logger`] with ECS-Logging formatting.
///
/// This should be called early in the execution of a Rust program. Any log events that occur before initialization will be ignored.
///
/// # Errors
///
/// This function returns [`log::SetLoggerError`] if it is called more than once, or if another library has already initialized a global logger.
///
/// # Example
///
/// ```
/// use log::error;
///
/// error!("this is NOT logged");
///
/// assert!(ecs_logger::try_init().is_ok());
///
/// error!("this is logged");
///
/// // try_init should not be called more than once
/// assert!(ecs_logger::try_init().is_err());
/// ```
pub fn try_init() -> Result<(), log::SetLoggerError> {
    env_logger::builder().format(format).try_init()
}

/// Writes an ECS log line to the `buf`.
///
/// You may pass this format function to [`env_logger::Builder::format`] when building a custom logger.
///
/// # Example
///
/// ```
/// use log::info;
///
/// // Initialize custom logger
/// env_logger::builder()
///     .parse_filters("info,my_app=debug") // Set filters
///     .format(ecs_logger::format) // Configure ECS logger
///     .target(env_logger::Target::Stdout) // Write to stdout
///     .init();
///
/// info!("Hello {}!", "world");
/// ```
pub fn format(buf: &mut impl std::io::Write, record: &log::Record) -> std::io::Result<()> {
    let event = Event::new(timestamp::get_timestamp(), record);

    let event_json_value =
        serde_json::to_value(event).expect("Event should be converted into JSON");
    let event_json_map = match event_json_value {
        serde_json::Value::Object(m) => m,
        _ => unreachable!("Event should be converted into a JSON object"),
    };

    let merged_json_map = merge_extra_fields(event_json_map);

    serde_json::to_writer(buf.borrow_mut(), &merged_json_map)?;
    writeln!(buf)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_init() {
        init();
        assert!(try_init().is_err());
    }

    #[test]
    fn test_format() {
        extra_fields::clear_extra_fields();

        let mut buf = Vec::new();
        let record = create_example_record();
        format(&mut buf, &record).unwrap();

        let log_line = String::from_utf8(buf).unwrap();
        assert_eq!(
            log_line,
            json!({
                "@timestamp": timestamp::MOCK_TIMESTAMP,
                "log.level": "ERROR",
                "message": "hello world",
                "ecs.version": "1.12.1",
                "log.origin": {
                    "file": {
                        "line": 13,
                        "name": "example.rs"
                    },
                    "rust": {
                        "target": "example",
                        "module_path": "example::tests",
                        "file_path": "tests/example.rs"
                    }
                }
            })
            .to_string()
                + "\n"
        );
    }

    #[test]
    fn test_format_with_extra_fields() {
        extra_fields::set_extra_fields(json!({
            "a": 1,
            "b": {
                "c": 2,
            },
        }))
        .unwrap();

        let mut buf = Vec::new();
        let record = create_example_record();
        format(&mut buf, &record).unwrap();

        let log_line = String::from_utf8(buf).unwrap();
        assert_eq!(
            log_line,
            json!({
                "@timestamp": timestamp::MOCK_TIMESTAMP,
                "log.level": "ERROR",
                "message": "hello world",
                "ecs.version": "1.12.1",
                "log.origin": {
                    "file": {
                        "line": 13,
                        "name": "example.rs"
                    },
                    "rust": {
                        "target": "example",
                        "module_path": "example::tests",
                        "file_path": "tests/example.rs"
                    }
                },
                "a": 1,
                "b": {
                    "c": 2,
                },
            })
            .to_string()
                + "\n"
        );
    }

    fn create_example_record<'a>() -> log::Record<'a> {
        log::Record::builder()
            .args(format_args!("hello world"))
            .level(log::Level::Error)
            .target("example")
            .file(Some("tests/example.rs"))
            .line(Some(13))
            .module_path(Some("example::tests"))
            .build()
    }
}
