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
