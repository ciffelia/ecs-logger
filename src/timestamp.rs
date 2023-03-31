#[cfg(not(test))]
pub fn get_timestamp() -> chrono::DateTime<chrono::Utc> {
    chrono::Utc::now()
}

#[cfg(test)]
pub static TEST_TIMESTAMP: &str = "2023-03-31T09:25:06.576136800Z";

#[cfg(test)]
pub fn get_timestamp() -> chrono::DateTime<chrono::Utc> {
    chrono::DateTime::parse_from_rfc3339(TEST_TIMESTAMP)
        .unwrap()
        .with_timezone(&chrono::Utc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_timestamp() {
        assert_eq!(
            serde_json::to_string(&get_timestamp()).unwrap(),
            r#""2023-03-31T09:25:06.576136800Z""#
        );
    }
}
