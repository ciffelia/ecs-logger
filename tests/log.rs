#[cfg(test)]
mod tests {
    use ecs_logger::logger::Builder;
    use log::{debug, error, info, log, trace, warn};
    use once_cell::sync::Lazy;
    use std::sync::Mutex;

    /// Collect log into a global sink.
    ///
    /// <[`log`] macro> -->> <[`ecs_logger`](crate)> -->> <[`sink::Writer`]> --(mpsc channel)>> <[`sink::Sink`]>
    mod sink {
        use std::sync::mpsc::{channel, Receiver, Sender};

        /// Create and initialize a [`Sink`] and a [`Writer`]
        pub fn create() -> (Sink, Writer) {
            let (sender, receiver) = channel();
            (Sink { receiver }, Writer { sender })
        }

        pub struct Sink {
            receiver: Receiver<u8>,
        }

        impl Sink {
            pub fn read(&self) -> String {
                String::from_utf8(self.receiver.try_iter().collect::<Vec<u8>>()).unwrap()
            }
        }

        /// This struct is used as an adaptor, it implements io::Write and forwards the buffer to a [`Sender`](std::sync::mpsc::Sender)
        pub struct Writer {
            sender: Sender<u8>,
        }

        impl std::io::Write for Writer {
            // On write we forward each u8 of the buffer to the sender and return the length of the buffer
            fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
                for chr in buf {
                    self.sender.send(*chr).unwrap();
                }
                Ok(buf.len())
            }

            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }
    }

    /// A global [`sink::Sink`] to collect logs
    static SINK: Lazy<Mutex<sink::Sink>> = Lazy::new(|| {
        let (sink, writer) = sink::create();

        let logger = Builder::new()
            .filter("trace")
            .writer(Box::new(writer))
            .build();
        logger.try_init().expect("Failed to initialize logger");

        Mutex::new(sink)
    });

    #[test]
    fn test_logs() {
        let sink = SINK.lock().unwrap();

        error!("error log! {}!", 123);
        warn!("warn log! {}!", "456");
        info!("info log! {}!", 789);
        debug!("debug log! {}!", "abc");
        trace!("trace log! {}!", "def");

        let output = sink.read();
        assert!(output.ends_with('\n'));

        let lines: Vec<&str> = output.lines().collect();
        assert!(lines[0].starts_with(r#"{"@timestamp":""#));
        assert!(lines[0].ends_with(r#"Z","log.level":"ERROR","message":"error log! 123!","ecs.version":"1.12.1","log.origin":{"file":{"line":67,"name":"log.rs"},"rust":{"target":"log::tests","module_path":"log::tests","file_path":"tests/log.rs"}}}"#));
        assert!(lines[1].starts_with(r#"{"@timestamp":""#));
        assert!(lines[1].ends_with(r#"Z","log.level":"WARN","message":"warn log! 456!","ecs.version":"1.12.1","log.origin":{"file":{"line":68,"name":"log.rs"},"rust":{"target":"log::tests","module_path":"log::tests","file_path":"tests/log.rs"}}}"#));
        assert!(lines[2].starts_with(r#"{"@timestamp":""#));
        assert!(lines[2].ends_with(r#"Z","log.level":"INFO","message":"info log! 789!","ecs.version":"1.12.1","log.origin":{"file":{"line":69,"name":"log.rs"},"rust":{"target":"log::tests","module_path":"log::tests","file_path":"tests/log.rs"}}}"#));
        assert!(lines[3].starts_with(r#"{"@timestamp":""#));
        assert!(lines[3].ends_with(r#"Z","log.level":"DEBUG","message":"debug log! abc!","ecs.version":"1.12.1","log.origin":{"file":{"line":70,"name":"log.rs"},"rust":{"target":"log::tests","module_path":"log::tests","file_path":"tests/log.rs"}}}"#));
        assert!(lines[4].starts_with(r#"{"@timestamp":""#));
        assert!(lines[4].ends_with(r#"Z","log.level":"TRACE","message":"trace log! def!","ecs.version":"1.12.1","log.origin":{"file":{"line":71,"name":"log.rs"},"rust":{"target":"log::tests","module_path":"log::tests","file_path":"tests/log.rs"}}}"#));
    }

    #[test]
    fn test_target() {
        let sink = SINK.lock().unwrap();

        log!(target: "example_target", log::Level::Info, "log with {:?}!", "custom target".to_string());

        let output = sink.read();
        assert!(output.ends_with('\n'));

        let lines: Vec<&str> = output.lines().collect();
        assert!(lines[0].starts_with(r#"{"@timestamp":""#));
        assert!(lines[0].ends_with(r#"Z","log.level":"INFO","message":"log with \"custom target\"!","ecs.version":"1.12.1","log.origin":{"file":{"line":93,"name":"log.rs"},"rust":{"target":"example_target","module_path":"log::tests","file_path":"tests/log.rs"}}}"#));
    }
}
