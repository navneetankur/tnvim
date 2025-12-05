use rmpv::Value;

pub(crate) struct Request {
    msgid: u32,
    method: String,
    params: Vec<Value>,
}
