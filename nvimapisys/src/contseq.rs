use serde::{Deserializer, de::Visitor};

pub(crate) struct ContSeq<'de, A,>
where 
    A: serde::de::SeqAccess<'de>,
{
    seq: A,
    _pd: core::marker::PhantomData<&'de ()>
}

impl<'de, A> ContSeq<'de, A>
where 
    A: serde::de::SeqAccess<'de>,
{
    pub(crate) fn new(seq: A) -> Self {
        Self { seq, _pd: core::marker::PhantomData }
    }
}
impl<'de, A> Deserializer<'de> for ContSeq<'de, A>
where 
    A: serde::de::SeqAccess<'de>,
{
    type Error = A::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        visitor.visit_seq(self.seq)
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}
