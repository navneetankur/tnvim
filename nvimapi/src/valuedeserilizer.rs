use rmpv::Value;
use serde::{Deserializer, de::{DeserializeSeed, MapAccess, SeqAccess}};

pub struct ValueDeserilizer(pub Value);

impl<'de> Deserializer<'de> for ValueDeserilizer {
    type Error = crate::error::Error;

    fn deserialize_any<V>(self, vis: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>
    {
        match self.0 {
            Value::Nil => vis.visit_unit::<Self::Error>(),
            Value::Boolean(val) => vis.visit_bool(val),
            Value::Integer(integer) => {
                if let Some(int) = integer.as_u64() {
                    vis.visit_u64(int)
                } else {
                    vis.visit_i64(integer.as_i64().unwrap())
                }
            },
            Value::F32(v) => vis.visit_f32(v),
            Value::F64(v) => vis.visit_f64(v),
            Value::String(utf8_string) => {
                let Some(string) = utf8_string.into_str() else {return crate::error::from_cow("non utf8 string")};
                vis.visit_string(string)
            },
            Value::Binary(items) => vis.visit_byte_buf(items),
            Value::Array(values) => vis.visit_seq(ValueSeqAccessor::new(values)),
            Value::Map(items) => vis.visit_map(ValueMapAccessor::new(items)),
            Value::Ext(t, _items) => return crate::error::from_cow(format!("not implemented extension: {t}")),
        }
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

struct ValueMapAccessor {
    // inner: Vec<(Value, Value)>,
    inner: std::vec::IntoIter<(Value, Value)>,
    value: Option<Value>,
}

impl ValueMapAccessor {
    fn new(v: Vec<(Value, Value)>,) -> Self {
        Self { inner: v.into_iter(), value: None }
    }
}
use crate::error;
impl<'de> MapAccess<'de> for ValueMapAccessor {
    type Error = crate::error::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>
    {
        let Some((key, value)) = self.inner.next() else { return Ok(None) };
        self.value = Some(value);
        return seed.deserialize(ValueDeserilizer(key)).map(Some);
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>
    {
        if let Some(value) = self.value.take() {
            return seed.deserialize(ValueDeserilizer(value));
        }
        return error::from_cow("Get key before value");
    }
    fn size_hint(&self) -> Option<usize> {
        self.inner.size_hint().1
    }
}

struct ValueSeqAccessor {
    inner: std::vec::IntoIter<Value>,
}

impl ValueSeqAccessor {
    fn new(v: Vec<Value>) -> Self {
        Self { inner: v.into_iter() }
    }
}
impl<'de> SeqAccess<'de> for ValueSeqAccessor {
    type Error = error::Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>
    {
        let Some(value) = self.inner.next() else { return Ok(None) };
        return seed.deserialize(ValueDeserilizer(value)).map(Some);
    }

    fn size_hint(&self) -> Option<usize> {
        self.inner.size_hint().1
    }
}

