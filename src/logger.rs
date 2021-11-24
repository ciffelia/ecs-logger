use crate::ecs::Event;
use env_logger::filter::Filter;
use log::{Log, Metadata, Record, SetLoggerError};
use std::ops::DerefMut;
use std::sync::Mutex;

pub struct Logger {
    filter: Filter,
    writer: Box<Mutex<dyn std::io::Write + Send>>,
}

impl Logger {
    pub fn try_init(self) -> Result<(), SetLoggerError> {
        log::set_max_level(self.filter.filter());
        log::set_boxed_logger(Box::new(self))
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.filter.enabled(metadata)
    }

    fn log(&self, record: &Record) {
        // Check if the record is matched by the logger before logging
        if !self.filter.matches(record) {
            return;
        }

        let event = Event::from_log_record(record);

        let mut writer = self.writer.lock().expect("Unexpected mutex error");
        serde_json::to_writer(writer.deref_mut(), &event).expect("Unexpected serialization error");
        writeln!(writer).expect("Unexpected writer error");
    }

    fn flush(&self) {}
}

pub struct Builder<'a> {
    filter: &'a str,
    writer: Box<dyn std::io::Write + Send>,
}

impl<'a> Default for Builder<'a> {
    fn default() -> Self {
        Self {
            filter: "error",
            writer: Box::new(std::io::stderr()),
        }
    }
}

impl<'a> Builder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn filter(mut self, filter: &'a str) -> Self {
        self.filter = filter;
        self
    }

    pub fn writer(mut self, writer: Box<dyn std::io::Write + Send>) -> Self {
        self.writer = writer;
        self
    }

    pub fn writer_stdout(self) -> Self {
        self.writer(Box::new(std::io::stdout()))
    }

    pub fn writer_stderr(self) -> Self {
        self.writer(Box::new(std::io::stderr()))
    }

    pub fn build(self) -> Logger {
        let mut filter_builder = env_logger::filter::Builder::new();
        filter_builder.parse(self.filter);

        Logger {
            filter: filter_builder.build(),
            writer: Box::new(Mutex::new(self.writer)),
        }
    }
}
