use std::collections::BTreeMap;

use serde::{
    Serialize, Serializer,
    ser::{SerializeMap, SerializeSeq},
};
use serde_json::{Map, Number, Value};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("PHP cache value is incompatible")]
pub struct PhpCacheError;

#[derive(Clone, Debug, PartialEq)]
enum PhpValue {
    Null,
    Bool(bool),
    Integer(i64),
    Unsigned(u64),
    Float(f64),
    String(String),
    Sequence(Vec<PhpValue>),
    Map(BTreeMap<String, PhpValue>),
}

pub fn decode(bytes: &[u8]) -> Result<Value, PhpCacheError> {
    let mut parser = Parser { bytes, offset: 0 };
    let value = parser.value()?;
    if parser.offset != bytes.len() {
        return Err(PhpCacheError);
    }
    Ok(value)
}

pub fn encode(value: &Value) -> Result<Vec<u8>, PhpCacheError> {
    let value = PhpValue::from_json(value)?;
    php_serde::to_vec(&value).map_err(|_| PhpCacheError)
}

impl PhpValue {
    fn from_json(value: &Value) -> Result<Self, PhpCacheError> {
        Ok(match value {
            Value::Null => Self::Null,
            Value::Bool(value) => Self::Bool(*value),
            Value::Number(value) => {
                if let Some(value) = value.as_i64() {
                    Self::Integer(value)
                } else if let Some(value) = value.as_u64() {
                    Self::Unsigned(value)
                } else {
                    Self::Float(value.as_f64().ok_or(PhpCacheError)?)
                }
            }
            Value::String(value) => Self::String(value.clone()),
            Value::Array(values) => Self::Sequence(
                values
                    .iter()
                    .map(Self::from_json)
                    .collect::<Result<_, _>>()?,
            ),
            Value::Object(values) => Self::Map(
                values
                    .iter()
                    .map(|(key, value)| Ok((key.clone(), Self::from_json(value)?)))
                    .collect::<Result<_, PhpCacheError>>()?,
            ),
        })
    }
}

impl Serialize for PhpValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Null => serializer.serialize_none(),
            Self::Bool(value) => serializer.serialize_bool(*value),
            Self::Integer(value) => serializer.serialize_i64(*value),
            Self::Unsigned(value) => serializer.serialize_u64(*value),
            Self::Float(value) => serializer.serialize_f64(*value),
            Self::String(value) => serializer.serialize_str(value),
            Self::Sequence(values) => {
                let mut sequence = serializer.serialize_seq(Some(values.len()))?;
                for value in values {
                    sequence.serialize_element(value)?;
                }
                sequence.end()
            }
            Self::Map(values) => {
                let mut map = serializer.serialize_map(Some(values.len()))?;
                for (key, value) in values {
                    map.serialize_entry(key, value)?;
                }
                map.end()
            }
        }
    }
}

struct Parser<'a> {
    bytes: &'a [u8],
    offset: usize,
}

enum ArrayKey {
    Integer(i64),
    String(String),
}

impl Parser<'_> {
    fn value(&mut self) -> Result<Value, PhpCacheError> {
        match self.take()? {
            b'N' => {
                self.expect(b';')?;
                Ok(Value::Null)
            }
            b'b' => {
                self.expect(b':')?;
                let value = self.take_until(b';')?;
                match value {
                    b"0" => Ok(Value::Bool(false)),
                    b"1" => Ok(Value::Bool(true)),
                    _ => Err(PhpCacheError),
                }
            }
            b'i' => {
                self.expect(b':')?;
                let value = self.integer_until(b';')?;
                Ok(Value::Number(value.into()))
            }
            b'd' => {
                self.expect(b':')?;
                let bytes = self.take_until(b';')?;
                let value = std::str::from_utf8(bytes)
                    .map_err(|_| PhpCacheError)?
                    .parse::<f64>()
                    .map_err(|_| PhpCacheError)?;
                Ok(Value::Number(Number::from_f64(value).ok_or(PhpCacheError)?))
            }
            b's' => self.string().map(Value::String),
            b'a' => self.array(),
            _ => Err(PhpCacheError),
        }
    }

    fn key(&mut self) -> Result<ArrayKey, PhpCacheError> {
        match self.take()? {
            b'i' => {
                self.expect(b':')?;
                Ok(ArrayKey::Integer(self.integer_until(b';')?))
            }
            b's' => Ok(ArrayKey::String(self.string()?)),
            _ => Err(PhpCacheError),
        }
    }

    fn string(&mut self) -> Result<String, PhpCacheError> {
        self.expect(b':')?;
        let length = self.usize_until(b':')?;
        self.expect(b'"')?;
        let end = self.offset.checked_add(length).ok_or(PhpCacheError)?;
        let bytes = self.bytes.get(self.offset..end).ok_or(PhpCacheError)?;
        self.offset = end;
        self.expect(b'"')?;
        self.expect(b';')?;
        String::from_utf8(bytes.to_vec()).map_err(|_| PhpCacheError)
    }

    fn array(&mut self) -> Result<Value, PhpCacheError> {
        self.expect(b':')?;
        let length = self.usize_until(b':')?;
        self.expect(b'{')?;
        let mut entries = Vec::with_capacity(length);
        for _ in 0..length {
            entries.push((self.key()?, self.value()?));
        }
        self.expect(b'}')?;

        let sequential = entries.iter().enumerate().all(
            |(index, (key, _))| matches!(key, ArrayKey::Integer(value) if *value == index as i64),
        );
        if sequential {
            Ok(Value::Array(
                entries.into_iter().map(|(_, value)| value).collect(),
            ))
        } else {
            let mut object = Map::new();
            for (key, value) in entries {
                let key = match key {
                    ArrayKey::Integer(value) => value.to_string(),
                    ArrayKey::String(value) => value,
                };
                object.insert(key, value);
            }
            Ok(Value::Object(object))
        }
    }

    fn integer_until(&mut self, delimiter: u8) -> Result<i64, PhpCacheError> {
        let bytes = self.take_until(delimiter)?;
        std::str::from_utf8(bytes)
            .map_err(|_| PhpCacheError)?
            .parse()
            .map_err(|_| PhpCacheError)
    }

    fn usize_until(&mut self, delimiter: u8) -> Result<usize, PhpCacheError> {
        let bytes = self.take_until(delimiter)?;
        std::str::from_utf8(bytes)
            .map_err(|_| PhpCacheError)?
            .parse()
            .map_err(|_| PhpCacheError)
    }

    fn take_until(&mut self, delimiter: u8) -> Result<&[u8], PhpCacheError> {
        let start = self.offset;
        let relative = self
            .bytes
            .get(start..)
            .ok_or(PhpCacheError)?
            .iter()
            .position(|byte| *byte == delimiter)
            .ok_or(PhpCacheError)?;
        let end = start + relative;
        self.offset = end + 1;
        Ok(&self.bytes[start..end])
    }

    fn take(&mut self) -> Result<u8, PhpCacheError> {
        let value = *self.bytes.get(self.offset).ok_or(PhpCacheError)?;
        self.offset += 1;
        Ok(value)
    }

    fn expect(&mut self, expected: u8) -> Result<(), PhpCacheError> {
        if self.take()? == expected {
            Ok(())
        } else {
            Err(PhpCacheError)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn strings_are_not_exposed_as_byte_arrays() {
        let packed = b"a:2:{s:12:\"access_token\";s:5:\"token\";s:10:\"expires_in\";i:7200;}";
        let decoded = decode(packed).unwrap();
        assert_eq!(decoded, json!({"access_token":"token","expires_in":7200}));
    }

    #[test]
    fn unicode_round_trip_preserves_values() {
        let original = json!({"ticket":"票据","nested":{"ok":true},"values":[1,"二"]});
        assert_eq!(decode(&encode(&original).unwrap()).unwrap(), original);
    }
}
