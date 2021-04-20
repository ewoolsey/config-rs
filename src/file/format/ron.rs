use std::collections::HashMap;
use std::error::Error;

use ron;

use crate::value::{Value, ValueKind};

pub fn parse(
    uri: Option<&String>,
    text: &str,
) -> Result<HashMap<String, Value>, Box<dyn Error + Send + Sync>> {
    let value = from_ron_value(uri, ron::from_str(text)?)?;
    match value.kind {
        ValueKind::Table(map) => Ok(map),

        _ => Ok(HashMap::new()),
    }
}

fn from_ron_value(
    uri: Option<&String>,
    value: ron::Value,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let kind = match value {
        ron::Value::Option(value) => match value {
            Some(value) => from_ron_value(uri, *value)?.kind,
            None => ValueKind::Nil,
        },

        ron::Value::Unit => ValueKind::Nil,

        ron::Value::Bool(value) => ValueKind::Boolean(value),

        ron::Value::Number(value) => match value {
            ron::Number::Float(value) => ValueKind::Float(value.get()),
            ron::Number::Integer(value) => ValueKind::Integer(value),
        },

        ron::Value::Char(value) => ValueKind::String(value.to_string()),

        ron::Value::String(value) => ValueKind::String(value),

        ron::Value::Seq(values) => {
            let array = values
                .into_iter()
                .map(|value| from_ron_value(uri, value))
                .collect::<Result<Vec<_>, _>>()?;

            ValueKind::Array(array)
        }

        ron::Value::Map(values) => {
            let map = values
                .iter()
                .map(|(key, value)| -> Result<_, Box<dyn Error + Send + Sync>> {
                    let key = key.clone().into_rust::<String>()?;
                    let value = from_ron_value(uri, value.clone())?;

                    Ok((key, value))
                })
                .collect::<Result<HashMap<_, _>, _>>()?;

            ValueKind::Table(map)
        }
    };

    Ok(Value::new(uri, kind))
}
