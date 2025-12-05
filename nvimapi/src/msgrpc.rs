use rmpv::Value;
use crate::valueseq::{SerialSeq, ValueSeq};
const REQUEST_CODE: u8 = 0;
const RESPONSE_CODE: u8 = 1;
const NOTIFICATION_CODE: u8 = 2;
pub fn create_request_value(msg_id: u32, fn_name: String, args: impl ValueSeq) -> Value {
    Value::Array(vec![
        Value::from(REQUEST_CODE),
        Value::from(msg_id),
        Value::from(fn_name),
        ValueSeq::to_value(args),
    ])
}
pub fn create_request_ser<S: SerialSeq>(msg_id: u32, fn_name: &str, args: S)
    -> (u8, u32, &str, S)
{
    (
        REQUEST_CODE,
        msg_id,
        fn_name,
        args,
    )
}
mod message;
pub use message::Message;
pub use message::Request;
pub use message::Response;
