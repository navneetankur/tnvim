use rmpv::Value;
use serde::{Deserialize, de::Error as DError};

pub(crate) struct Response {
    msgid: u32,
    error: Value,
    result: Value,
}

impl<'de> Deserialize<'de> for Response {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        return deserializer.deserialize_seq(RVisitor);


        struct RVisitor;
        impl<'de> serde::de::Visitor<'de> for RVisitor {
            type Value = Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("expecting seq")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let msg = "expected 3 elements";
                let Some(msgid) = seq.next_element::<u32>()? else {
                    return Err(DError::custom(msg));
                };
                let Some(error) = seq.next_element::<Value>()? else {
                    return Err(DError::custom(msg));
                };
                let Some(result) = seq.next_element::<Value>()? else {
                    return Err(DError::custom(msg));
                };
                return Ok(Response {
                    msgid,
                    error,
                    result,
                });
            }

        }

    }
}
