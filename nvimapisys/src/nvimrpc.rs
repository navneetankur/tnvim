mod msgmapc;
mod valueseq;
use core::{cell::{Cell, RefCell}, ops::DerefMut};
use std::os::unix::net::UnixStream;
use rmpv::Value;
use serde::Deserialize;
use tokio::sync::{mpsc, oneshot};
use crate::{error, nvimrpc::valueseq::{SerialSeq, ValueSeq}};
// will keep a writer to encode with.
// it will send its message(request) id to main loop.
// and a channel rx. Where it will get redraw, request or notify messages.
// then it will call appropriate handler method.
pub struct Nvimapi
{
    // (msgid, oneshot returner.)
    tx: mpsc::Sender<MsgToReader>,
    msgid: Cell<u32>,
    write: RefCell<UnixStream>,
}
pub struct ApiAndHandler {
    api: Nvimapi,
    handler: Box<dyn Handler>,
    rx: mpsc::Receiver<MsgFromNvim>,
}
pub enum MsgToReader {
    PendingRequest(PendingRequest),
    End,
}
impl MsgToReader {
    pub fn new(msg_id: u32, sender: oneshot::Sender<Value>) -> Self {
        Self::PendingRequest(PendingRequest { msg_id, sender })
    }
}
struct PendingRequest {
    msg_id: u32,
    sender: oneshot::Sender<Value>,
}
pub trait Handler {
    fn notify(&self);
    fn redraw(&self);
    fn request(&self);
}
enum MsgFromNvim {
    Notify(Notify),
    Request(Request),
    Redraw(Redraw),
}
struct Notify;
struct Request;
struct Redraw;

struct ReadLoop {
    reader: UnixStream,
    msg_id: u32,
    rx: mpsc::Receiver<MsgToReader>,
}

impl ReadLoop {

}

// impl Processor {
// }

impl Nvimapi
{
    pub async fn call_fn_wv<R>(&self, fn_name: String, args: impl ValueSeq) -> error::Result<R>
    where 
        R: TryFromValue
    {
        let msg_id = self.get_next_msg_id();
        let request = msgmapc::create_request_value(msg_id, fn_name, args);
        let mut w = self.write.borrow_mut();
        rmpv::encode::write_value(w.deref_mut(), &request).unwrap();
        let (sender, rx) = oneshot::channel::<Value>();
        let msg = MsgToReader::new(msg_id, sender);
        self.tx.send(msg).await?;
        let rv = rx.await?;
        return R::try_from_value(rv);
    }

    pub async fn call_fn<R,S>(&self, fn_name: &str, args: S) -> error::Result<R>
    where 
        R: Deserialize<'static>,
        S: SerialSeq,
    {
        let msg_id = self.get_next_msg_id();
        let request = msgmapc::create_request_ser(msg_id, fn_name, args);
        let mut w = self.write.borrow_mut();
        rmp_serde::encode::write_named(w.deref_mut(), &request)?;
        let (sender, rx) = oneshot::channel::<Value>();
        let msg = MsgToReader::new(msg_id, sender);
        self.tx.send(msg).await?;
        let rv = rx.await?;
        return Ok(R::deserialize(rv)?);
    }
    fn get_next_msg_id(&self) -> u32 {
        let msg_id = self.msgid.get();
        self.msgid.update(|m| m+1);
        return msg_id;
    }

}
pub trait TryFromValue {
    fn try_from_value(value: Value) -> error::Result<Self> where Self: Sized;
}
macro_rules! impl_from_value {
    ($($type: path),* $(,)?) => {
        $(
        impl TryFromValue for $type {
            fn try_from_value(value: Value) -> error::Result<Self> {
                let Ok(rv) = Self::try_from(value) else {
                    return error::with_msg(format!("Failed to convert Value instance to {}.", stringify!($type)));
                };
                return Ok(rv);
            }
        }
        )*
    };
}
impl_from_value!(
    String, Vec<(Value, Value)>, Vec<Value>, Vec<u8>, bool, f32, f64, i64, u64,
);
impl TryFromValue for Value {
    fn try_from_value(value: Value) -> error::Result<Self> {
        Ok(value)
    }
}

impl TryFromValue for Vec<String> {
    fn try_from_value(value: Value) -> error::Result<Self> {
        let Value::Array(array) = value else {return error::with_msg("expected array.")};
        let mut rv = Vec::with_capacity(array.len());
        for value in array {
            let Value::String(value) = value else {return error::with_msg("expected String")};
            let Some(value) = value.into_str() else { return error::with_msg("string not utf8"); };
            rv.push(value);
        }
        return Ok(rv);
    }
}
impl TryFromValue for Vec<i64> {
    fn try_from_value(value: Value) -> error::Result<Self> {
        let Value::Array(array) = value else {return error::with_msg("expected array.")};
        let mut rv = Vec::with_capacity(array.len());
        for value in array {
            let Ok(value) = i64::try_from(value) else {return error::with_msg("expected i64")};
            rv.push(value);
        }
        return Ok(rv);
    }
}
impl TryFromValue for Vec<Vec<(Value, Value)>> {
    fn try_from_value(value: Value) -> error::Result<Self> where Self: Sized {
        let Value::Array(array) = value else { return error::with_msg("expected array.") };
        let mut rv = Vec::with_capacity(array.len());
        for map in array {
            let Value::Map(map) = map else {return error::with_msg("expected map.")};
            rv.push(map);
        }
        return Ok(rv);
    }
}
impl TryFromValue for () {
    fn try_from_value(_: Value) -> error::Result<Self> where Self: Sized {
        Ok(())
    }
}
