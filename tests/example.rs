#[cfg(test)]
mod tests {
    use log::{debug, error};
    use serde::Serialize;

    #[derive(Serialize)]
    struct ExtraData {
        #[serde(rename = "organization.name")]
        organization_name: String,
        service: Service,
    }

    #[derive(Serialize)]
    struct Service {
        name: String,
        version: String,
    }

    #[test]
    fn main() {
        ecs_logger::init();

        ecs_logger::set_extra_fields(ExtraData {
            organization_name: "Example Company".to_string(),
            service: Service {
                name: "example-service".to_string(),
                version: "1.2.3".to_string(),
            },
        })
        .unwrap();

        debug!(
            "this is a debug {}, which is NOT printed by default",
            "message"
        );
        error!("this is printed by default");
    }
}
