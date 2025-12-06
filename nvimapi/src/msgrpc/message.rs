use serde::Deserialize;
mod request;
pub use request::Request;
mod response;
pub use response::Response;

use crate::{contseq::ContSeq, msgrpc::{NOTIFICATION_CODE, REQUEST_CODE, RESPONSE_CODE}, nvimapi::notification::Notification};

#[derive(Debug)]
pub enum Message {
    Request(Request),
    Response(Response),
    Notification(Notification),
}

impl<'de> Deserialize<'de> for Message {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        return deserializer.deserialize_seq(MVisitor);


        use serde::de::Visitor;
        struct MVisitor;
        impl<'de> Visitor<'de> for MVisitor {
            type Value = Message;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("expecting seq")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                use serde::de::Error as DError;
                let msg = "missing item. expecting 2 elements.";
                let Some(type_) = seq.next_element::<u8>()? else {return Err(DError::custom(msg))};
                match type_ {
                    NOTIFICATION_CODE => {
                        let inner = Notification::deserialize(ContSeq::new(seq))?;
                        return Ok(Message::Notification(inner));
                    },
                    RESPONSE_CODE => {
                        let inner = Response::deserialize(ContSeq::new(seq))?;
                        return Ok(Message::Response(inner));

                    },
                    REQUEST_CODE => {
                        let inner = Request::deserialize(ContSeq::new(seq))?;
                        return Ok(Message::Request(inner));
                    },
                    c => {unreachable!("msgrpc doesn't have type {c}")}
                }
            }

        }
    }
}
