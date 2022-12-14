use serde_json::Value;

use crate::error::{Error, ErrorKind};

pub fn validate_string(attribute_value: &Value) -> Result<(), Error> {
    match attribute_value.as_str() {
        Some(_) => Ok(()),
        None => Err(Error::new(
            ErrorKind::InvalidValue,
            "failed to convert attribute value to string".to_string(),
        )),
    }
}

pub fn validate_integer(attribute_value: &Value) -> Result<(), Error> {
    match attribute_value.as_i64() {
        Some(_) => Ok(()),
        None => Err(Error::new(
            ErrorKind::InvalidValue,
            "failed to convert attribute value to integer".to_string(),
        )),
    }
}

pub fn validate_decimal(attribute_value: &Value) -> Result<(), Error> {
    match attribute_value.as_f64() {
        Some(_) => Ok(()),
        None => Err(Error::new(
            ErrorKind::InvalidValue,
            "failed to convert attribute value to decimal".to_string(),
        )),
    }
}

pub fn validate_boolean(attribute_value: &Value) -> Result<(), Error> {
    match attribute_value.as_bool() {
        Some(_) => Ok(()),
        None => Err(Error::new(
            ErrorKind::InvalidValue,
            "failed to convert attribute value to boolean".to_string(),
        )),
    }
}
