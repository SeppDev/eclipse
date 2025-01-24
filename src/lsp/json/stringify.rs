use super::Value;

impl Value {
    pub fn stringify(self) -> String {
        match self {
            Self::Null => "null".to_string(),
            Self::Literal(literal) => literal,
            Self::String(string) => format!("\"{string}\""),
            Self::Bool(bool) => format!("{bool}"),
            Self::Number(number) => number.as_string(),
            Self::Array(array) => format!(
                "[{}]",
                array
                    .into_iter()
                    .map(|v| v.stringify())
                    .collect::<Vec<String>>()
                    .join(",")
            ),
            Self::Object(object) => format!(
                "{{{}}}",
                object
                    .into_iter()
                    .map(|(key, value)| format!("\"{key}\":{}", value.stringify()))
                    .collect::<Vec<String>>()
                    .join(",")
            ),
        }
    }
}
