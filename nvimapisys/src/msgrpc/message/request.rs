use rmpv::Value;
use serde::Deserialize;

#[derive(Debug)]
pub struct Request {
    pub msgid: u32,
    pub method: String,
    pub params: Vec<Value>,
}
impl<'de> Deserialize<'de> for Request {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        return deserializer.deserialize_seq(RVisitor);


        struct RVisitor;
        impl<'de> serde::de::Visitor<'de> for RVisitor {
            type Value = Request;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("expecting seq")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                use serde::de::Error as DError;
                let msg = "expected 3 elements";
                let Some(msgid) = seq.next_element::<u32>()? else {
                    return Err(DError::custom(msg));
                };
                let Some(method) = seq.next_element::<String>()? else {
                    return Err(DError::custom(msg));
                };
                let Some(params) = seq.next_element::<Value>()? else {
                    return Err(DError::custom(msg));
                };
                let Value::Array(params) = params else {
                    return Err(DError::custom("expected params to be array"));
                };
                return Ok(Request {
                    msgid,
                    method,
                    params,
                });
            }

        }

    }
}
