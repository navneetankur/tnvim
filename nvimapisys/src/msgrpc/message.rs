use rmpv::Value;
use serde::Deserialize;
mod request;
use request::Request;
mod response;
use response::Response;
mod notification;
use notification::Notification;

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
        return todo!();


        use serde::de::Visitor;
        struct MVisitor;
        impl<'de> Visitor<'de> for MVisitor {
            type Value = Message;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("expecting seq")
            }

            fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                return todo!();
            }

        }
    }
}
