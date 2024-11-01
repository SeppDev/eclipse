use crate::types::Value;

pub fn extract_value(value: Value) -> String {
    return match value {
        Value::Integer(minus, value) => {
            if minus {
                format!("-{}", value)
            } else {
                format!("{}", value)
            }
        }
        Value::Float(value) => format!("{}", value),
        Value::Boolean(value) => {
            if value {
                String::from("1")
            } else {
                String::from("0")
            }
        }
        Value::String(_string) => todo!("String value"),
    };
}
