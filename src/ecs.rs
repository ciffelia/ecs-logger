use chrono::{DateTime, Utc};
use serde::Serialize;
use std::path::Path;

const ECS_VERSION: &str = "1.12.1";

// The event follows ECS Logging spec: https://github.com/elastic/ecs-logging/tree/master/spec
#[derive(Debug, Clone, Serialize)]
pub struct Event<'a> {
    #[serde(rename = "@timestamp")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "log.level")]
    pub log_level: &'static str,

    pub message: String,

    #[serde(rename = "ecs.version")]
    pub ecs_version: &'static str,

    #[serde(rename = "log.origin")]
    pub log_origin: LogOrigin<'a>,
}

#[derive(Debug, Clone, Serialize)]
pub struct LogOrigin<'a> {
    pub file: LogOriginFile<'a>,
    pub rust: LogOriginRust<'a>,
}

#[derive(Debug, Clone, Serialize)]
pub struct LogOriginFile<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize)]
pub struct LogOriginRust<'a> {
    pub target: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_path: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<&'a str>,
}

impl<'a> Event<'a> {
    pub fn from_log_record(record: &'a log::Record<'a>) -> Self {
        let file_path = record.file().map(|f| Path::new(f));

        Event {
            timestamp: Utc::now(),
            log_level: record.level().as_str(),
            message: record.args().to_string(),
            ecs_version: ECS_VERSION,
            log_origin: LogOrigin {
                file: LogOriginFile {
                    line: record.line(),
                    name: file_path
                        .and_then(|p| p.file_name())
                        .and_then(|os_str| os_str.to_str()),
                },
                rust: LogOriginRust {
                    target: record.target(),
                    module_path: record.module_path(),
                    file_path: record.file(),
                },
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_from_log_record() {
        let record = log::Record::builder()
            .args(format_args!("Error!"))
            .level(log::Level::Error)
            .target("myApp")
            .file(Some("src/server.rs"))
            .line(Some(144))
            .module_path(Some("my_app::server"))
            .build();

        let event = Event::from_log_record(&record);

        assert_eq!(event.log_level, "ERROR");
        assert_eq!(event.message, "Error!");
        assert_eq!(event.ecs_version, "1.12.1");
        assert_eq!(event.log_origin.file.line, Some(144));
        assert_eq!(event.log_origin.file.name, Some("server.rs"));
        assert_eq!(event.log_origin.rust.target, "myApp");
        assert_eq!(event.log_origin.rust.module_path, Some("my_app::server"));
        assert_eq!(event.log_origin.rust.file_path, Some("src/server.rs"));
    }

    #[test]
    fn test_serialize() {
        let event = Event {
            timestamp: Utc.timestamp(1637775501, 98765),
            log_level: "TRACE",
            message: "tracing msg".to_string(),
            ecs_version: "1.12.1",
            log_origin: LogOrigin {
                file: LogOriginFile {
                    line: Some(1234),
                    name: Some("file.rs"),
                },
                rust: LogOriginRust {
                    target: "myCustomTarget123",
                    module_path: Some("my_app::path::to::your::file"),
                    file_path: Some("src/path/to/your/file.rs"),
                },
            },
        };

        assert_eq!(
            serde_json::to_string(&event).expect("Failed to serialize ECS event"),
            r#"{"@timestamp":"2021-11-24T17:38:21.000098765Z","log.level":"TRACE","message":"tracing msg","ecs.version":"1.12.1","log.origin":{"file":{"line":1234,"name":"file.rs"},"rust":{"target":"myCustomTarget123","module_path":"my_app::path::to::your::file","file_path":"src/path/to/your/file.rs"}}}"#
        );
    }

    #[test]
    fn test_serialize_with_none() {
        let event = Event {
            timestamp: Utc.timestamp(1637775501, 98765),
            log_level: "TRACE",
            message: "tracing msg".to_string(),
            ecs_version: "1.12.1",
            log_origin: LogOrigin {
                file: LogOriginFile {
                    line: None,
                    name: None,
                },
                rust: LogOriginRust {
                    target: "myCustomTarget123",
                    module_path: None,
                    file_path: None,
                },
            },
        };

        assert_eq!(
            serde_json::to_string(&event).expect("Failed to serialize ECS event"),
            r#"{"@timestamp":"2021-11-24T17:38:21.000098765Z","log.level":"TRACE","message":"tracing msg","ecs.version":"1.12.1","log.origin":{"file":{},"rust":{"target":"myCustomTarget123"}}}"#
        );
    }
}
