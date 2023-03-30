use serde_json::{Map, Value};
use std::sync::RwLock;
use thiserror::Error;

pub type JsonMap = Map<String, Value>;

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
pub fn merge_extra_fields(mut json_map: JsonMap) -> JsonMap {
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
