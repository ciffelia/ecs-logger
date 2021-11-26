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

pub mod ecs;
pub mod logger;

pub fn init() {
    try_init().expect("ecs_logger::init should not be called after logger initialized");
}

pub fn try_init() -> Result<(), log::SetLoggerError> {
    let mut builder = logger::Builder::new();

    let filter = std::env::var("RUST_LOG");
    if let Ok(f) = &filter {
        builder = builder.filter(f);
    }

    let logger = builder.build();
    logger.try_init()
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
