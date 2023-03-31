#[cfg(test)]
mod tests {
    use log::{debug, error, info, log, trace, warn};
    use once_cell::sync::Lazy;
    use regex::Regex;
    use std::sync::Mutex;

    /// Collect log into a global sink.
    ///
    /// <[`log`] macro> -->> <[`env_logger`]> -->> <[`sink::Writer`]> --(mpsc channel)>> <[`sink::Sink`]>
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

        env_logger::builder()
            .parse_filters("trace")
            .format(ecs_logger::format)
            .target(env_logger::Target::Pipe(Box::new(writer)))
            .init();

        Mutex::new(sink)
    });

    #[test]
    fn test_logs() {
        let sink = SINK.lock().unwrap();

        error!("error {}!", 123);
        warn!("foo");
        info!("{}", "456");
        debug!("bar {}", "abc");
        trace!("baz {}", false);

        let output = sink.read();
        let re = Regex::new(r#"^\{"@timestamp":"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d+Z","log\.level":"ERROR","message":"error 123!","ecs\.version":"1\.12\.1","log\.origin":\{"file":\{"line":\d+,"name":"log\.rs"},"rust":\{"target":"log::tests","module_path":"log::tests","file_path":"tests(?:/|\\\\)log\.rs"}}}
\{"@timestamp":"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d+Z","log\.level":"WARN","message":"foo","ecs\.version":"1\.12\.1","log\.origin":\{"file":\{"line":\d+,"name":"log\.rs"},"rust":\{"target":"log::tests","module_path":"log::tests","file_path":"tests(?:/|\\\\)log\.rs"}}}
\{"@timestamp":"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d+Z","log\.level":"INFO","message":"456","ecs\.version":"1\.12\.1","log\.origin":\{"file":\{"line":\d+,"name":"log\.rs"},"rust":\{"target":"log::tests","module_path":"log::tests","file_path":"tests(?:/|\\\\)log\.rs"}}}
\{"@timestamp":"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d+Z","log\.level":"DEBUG","message":"bar abc","ecs\.version":"1\.12\.1","log\.origin":\{"file":\{"line":\d+,"name":"log\.rs"},"rust":\{"target":"log::tests","module_path":"log::tests","file_path":"tests(?:/|\\\\)log\.rs"}}}
\{"@timestamp":"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d+Z","log\.level":"TRACE","message":"baz false","ecs\.version":"1\.12\.1","log\.origin":\{"file":\{"line":\d+,"name":"log\.rs"},"rust":\{"target":"log::tests","module_path":"log::tests","file_path":"tests(?:/|\\\\)log\.rs"}}}
$"#).unwrap();
        assert!(re.is_match(&output));
    }

    #[test]
    fn test_target() {
        let sink = SINK.lock().unwrap();

        log!(target: "example_target", log::Level::Info, "log with {:?}!", "custom target".to_string());

        let output = sink.read();
        let re = Regex::new(r#"^\{"@timestamp":"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d+Z","log\.level":"INFO","message":"log with \\"custom target\\"!","ecs\.version":"1\.12\.1","log\.origin":\{"file":\{"line":\d+,"name":"log\.rs"},"rust":\{"target":"example_target","module_path":"log::tests","file_path":"tests(?:/|\\\\)log\.rs"}}}
$"#).unwrap();
        assert!(re.is_match(&output));
    }
}
