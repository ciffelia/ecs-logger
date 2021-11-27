//! A logger compatible with [Elastic Common Schema (ECS) Logging](https://www.elastic.co/guide/en/ecs-logging/overview/current/intro.html).
//!
//! ## Features
//!
//! - Configurable via the `RUST_LOG` environment variable.
//!   - Uses [env_logger] under the hood.
//!   - **By default, all logging is disabled except for the `error` level.**
//! - By default logs are written to stderr.
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
//! ## Log fields
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

use crate::ecs::Event;
use std::borrow::BorrowMut;

pub mod ecs;

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
    let event = Event::new(chrono::Utc::now(), record);

    serde_json::to_writer(buf.borrow_mut(), &event)?;
    writeln!(buf)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        init();
        assert!(try_init().is_err());
    }
}
