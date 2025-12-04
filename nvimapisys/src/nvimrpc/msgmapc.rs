use rmpv::Value;
use crate::nvimrpc::valueseq::{SerialSeq, ValueSeq};
const REQUEST: u8 = 0;
const RESPONSE: u8 = 1;
const NOTIFY: u8 = 2;
pub fn create_request_value(msg_id: u32, fn_name: String, args: impl ValueSeq) -> Value {
    Value::Array(vec![
        Value::from(REQUEST),
        Value::from(msg_id),
        Value::from(fn_name),
        ValueSeq::to_value(args),
    ])
}
pub fn create_request_ser<S: SerialSeq>(msg_id: u32, fn_name: &str, args: S)
    -> (u8, u32, &str, S)
{
    (
        REQUEST,
        msg_id,
        fn_name,
        args,
    )
}

