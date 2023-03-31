#[cfg(not(test))]
pub fn get_timestamp() -> chrono::DateTime<chrono::Utc> {
    chrono::Utc::now()
}

#[cfg(test)]
pub static MOCK_TIMESTAMP: &str = "2000-01-23T01:23:45.678901200Z";

#[cfg(test)]
pub fn get_timestamp() -> chrono::DateTime<chrono::Utc> {
    chrono::DateTime::parse_from_rfc3339(MOCK_TIMESTAMP)
        .unwrap()
        .with_timezone(&chrono::Utc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_timestamp() {
        assert_eq!(
            get_timestamp().to_rfc3339(),
            "2000-01-23T01:23:45.678901200+00:00"
        );
    }
}
