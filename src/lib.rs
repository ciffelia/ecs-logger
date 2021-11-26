//! A logger compatible with [Elastic Common Schema (ECS) Logging](https://www.elastic.co/guide/en/ecs-logging/overview/current/intro.html).
//!
//! ## Features
//!
//! - Configurable via the `RUST_LOG` environment variable
//!   - Uses [env_logger] style formatting
//!   - **By default all logging is disabled except for the `error` level**
//! - By default logs are written to stderr
//!
//! ## Example
//!
//! ### Basic Logging
//!
//! ```
//! use log::{debug, error};
//!
//! ecs_logger::init();
//!
//! debug!("this is a debug {}, which is NOT printed by default", "message");
//! error!("this is printed by default");
//! ```

use crate::ecs::Event;
use std::borrow::BorrowMut;

pub mod ecs;

pub fn init() {
    try_init().expect("ecs_logger::init should not be called after logger initialized");
}

pub fn try_init() -> Result<(), log::SetLoggerError> {
    env_logger::builder().format(format).try_init()
}

pub fn format(buf: &mut impl std::io::Write, record: &log::Record) -> std::io::Result<()> {
    let event = Event::from_log_record(record);

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
