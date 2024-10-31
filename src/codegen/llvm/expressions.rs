use crate::Value;

pub fn value(value: Value) -> String {
    use crate::Value::{self};

    return match value {
        Value::Integer(_signed, value) => format!("{}", value),
        Value::Float(value) => format!("{}", value),
        Value::Boolean(value) => match value {
            true => String::from("1"),
            false => String::from("0"),
        },
        Value::String(string) => format!("{:?}", string),
    };
}
