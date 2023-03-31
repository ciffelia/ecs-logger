use serde_json::{Map, Value};
use std::sync::RwLock;
use thiserror::Error;

type JsonMap = Map<String, Value>;

static EXTRA_FIELDS: RwLock<Option<JsonMap>> = RwLock::new(None);

#[derive(Error, Debug)]
pub enum SetExtraFieldsError {
    #[error("the data cannot be converted into JSON")]
    InvalidJson(#[from] serde_json::Error),
    #[error("the data cannot be converted into a JSON object")]
    NotObject,
}

pub fn set_extra_fields(extra_fields: impl serde::Serialize) -> Result<(), SetExtraFieldsError> {
    let v = serde_json::to_value(extra_fields)?;
    let json_map = match v {
        Value::Object(m) => Some(m),
        _ => return Err(SetExtraFieldsError::NotObject),
    };

    {
        let mut w = EXTRA_FIELDS.write().unwrap();
        *w = json_map;
    }

    Ok(())
}

pub fn clear_extra_fields() {
    let mut w = EXTRA_FIELDS.write().unwrap();
    *w = None;
}

/// Deep merge extra fields into `json_map`
pub(crate) fn merge_extra_fields(mut json_map: JsonMap) -> JsonMap {
    let r = EXTRA_FIELDS.read().unwrap();
    if let Some(extra_fields) = &*r {
        extend_json_map(&mut json_map, extra_fields);
    }

    json_map
}

/// Deep merge `b` into `a`
fn extend_json_map(a: &mut JsonMap, b: &JsonMap) {
    for (k, v) in b {
        match (a.get_mut(k), v) {
            (Some(Value::Object(a)), Value::Object(b)) => extend_json_map(a, b),
            (Some(a), b) => {
                *a = b.clone();
            }
            (None, b) => {
                a.insert(k.clone(), b.clone());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::collections::BTreeMap;

    #[test]
    fn test_set_extra_fields_ok() {
        set_extra_fields(json!({
            "a": 1,
            "b": {
                "c": 2,
            },
        }))
        .unwrap();

        let r = EXTRA_FIELDS.read().unwrap();
        assert!(r.is_some());
        assert_eq!(
            serde_json::to_string(r.as_ref().unwrap()).unwrap(),
            json!({
                "a": 1,
                "b": {
                    "c": 2,
                },
            })
            .to_string()
        );
    }

    #[test]
    fn test_set_extra_fields_err() {
        let mut map = BTreeMap::new();
        map.insert(vec![32, 64], "x86");
        assert!(matches!(
            set_extra_fields(map),
            Err(SetExtraFieldsError::InvalidJson(_))
        ));

        assert!(matches!(
            set_extra_fields(1),
            Err(SetExtraFieldsError::NotObject)
        ));
    }

    #[test]
    fn test_clear_extra_fields() {
        set_extra_fields(json!({
            "a": 1,
            "b": {
                "c": 2,
            },
        }))
        .unwrap();

        clear_extra_fields();

        let r = EXTRA_FIELDS.read().unwrap();
        assert!(r.is_none());
    }

    #[test]
    fn test_merge_extra_fields() {
        set_extra_fields(json!({
            "b": {
                "d": 3,
            },
            "e": 4,
        }))
        .unwrap();

        let mut a = json!({
            "a": 1,
            "b": {
                "c": 2,
            },
        });
        let a_with_extra_fields = merge_extra_fields(a.as_object_mut().unwrap().clone());

        assert_eq!(
            serde_json::to_string(&Value::Object(a_with_extra_fields)).unwrap(),
            json!({
                "a": 1,
                "b": {
                    "c": 2,
                    "d": 3,
                },
                "e": 4,
            })
            .to_string()
        );
    }

    #[test]
    fn test_extend_json_map() {
        let mut a = json!({
            "a": 1,
            "b": {
                "c": 2,
            },
        });
        let b = json!({
            "b": {
                "d": 3,
            },
            "e": 4,
        });

        let a = a.as_object_mut().unwrap();
        let b = b.as_object().unwrap();

        extend_json_map(a, b);

        assert_eq!(
            serde_json::to_string(a).unwrap(),
            json!({
                "a": 1,
                "b": {
                    "c": 2,
                    "d": 3,
                },
                "e": 4,
            })
            .to_string()
        );
    }
}
