#[cfg(test)]
mod tests {
    use log::{debug, error};

    #[test]
    fn main() {
        ecs_logger::init();

        debug!(
            "this is a debug {}, which is NOT printed by default",
            "message"
        );
        error!("this is printed by default");
    }
}
