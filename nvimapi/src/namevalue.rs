use rmpv::Value;
use serde::{Deserialize, Deserializer, de::{DeserializeSeed, EnumAccess, VariantAccess, Visitor}};
use rmpv::ext::Error as RError;
fn error<T>(s: impl Into<String>) -> Result<T, RError> {
    Err(rmpv::ext::Error::Syntax(s.into()))
}
#[derive(Deserialize)]
pub struct NameValue<'s> {
    name: &'s str,
    value: Value,
}
struct NameDe<'s>(&'s str);
impl<'de> Deserializer<'de> for NameDe<'de> {
    type Error = RError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        visitor.visit_str(self.0)
    }
    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}


impl<'s> NameValue<'s> {
    pub fn new(name: &'s str, value: Value) -> Self {
        Self { name, value }
    }
}
impl<'de> Deserializer<'de> for NameValue<'de> {
    type Error = RError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        visitor.visit_enum(self)
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

impl<'de> EnumAccess<'de> for NameValue<'de> {
    type Error = RError;

    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>
    {
        let v = seed.deserialize(NameDe(self.name))?;
        return Ok((v, self));
    }
}

impl<'de> VariantAccess<'de> for NameValue<'de> {
    type Error = RError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        if Value::Nil == self.value {
            Ok(())
        } else {
            return error("not unit variant");
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>
    {
        seed.deserialize(self.value)
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        Deserializer::deserialize_tuple(self.value, len, visitor)
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        Deserializer::deserialize_struct(self.value, "", fields, visitor)
    }
}
