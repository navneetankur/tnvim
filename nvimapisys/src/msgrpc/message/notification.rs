use rmpv::Value;
use serde::Deserialize;

pub(crate) struct Notification {
    method: String,
    params: Vec<Value>,
}


impl<'de> Deserialize<'de> for Notification {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        return deserializer.deserialize_seq(NVisitor);


        struct NVisitor;
        impl<'de> serde::de::Visitor<'de> for NVisitor {
            type Value = Notification;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("expecting seq")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                use serde::de::Error as DError;
                let msg = "missing elements, expected 2 elements";
                let Some(method) = seq.next_element::<String>()? else {
                    return Err(DError::custom(msg));
                };
                let Some(params) = seq.next_element::<Vec<Value>>()? else {
                    return Err(DError::custom(msg));
                };
                return Ok(Notification {
                    method,
                    params,
                });
            }

        }

    }
}
