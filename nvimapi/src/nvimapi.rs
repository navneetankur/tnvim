use crate::{MsgToReader, msgrpc::{self, RESPONSE_CODE}, nvimapi::notification::Notification, valueseq};
use core::{cell::{Cell, RefCell}, ops::DerefMut};
use std::os::unix::net::UnixStream;
use rmpv::Value;
use serde::Deserialize;
use tokio::sync::{mpsc, oneshot};
use crate::{error, nvimapi::valueseq::{SerialSeq, ValueSeq}};
pub use crate::generated::{UiEvent, UiOptions};
pub mod notification;
pub const BUFFER_ID:  i8 = 0;
pub const WINDOW_ID:  i8 = 1;
pub const TABPAGE_ID: i8 = 2;
// will keep a writer to encode with.
// it will send its message(request) id to main loop.
// and a channel rx. Where it will get redraw, request or notify messages.
// then it will call appropriate handler method.
pub struct Nvimapi
{
    // (msgid, oneshot returner.)
    pub(crate) tx: mpsc::Sender<MsgToReader>,
    pub(crate) msgid: Cell<u32>,
    pub(crate) write: RefCell<UnixStream>,
}
pub struct ApiAndHandler<H: Handler> {
    api: Nvimapi,
    handler: H,
    // rx: mpsc::Receiver<MsgFromNvim>,
}
pub trait Handler {
    async fn notify(&self, notification: Notification);
    async fn request(&self);
}


impl Nvimapi
{
    pub fn send_response(&self, msgid: i32, error: impl serde::Serialize, result: impl serde::Serialize) -> error::Result<()> {
        let mut w = self.write.borrow_mut();
        rmp_serde::encode::write_named(w.deref_mut(), &(
            RESPONSE_CODE,
            msgid,
            error,
            result,
        ))?;
        drop(w);
        return Ok(());
    }
    pub fn send_response_wv(&self, msgid: i32, error: Value, result: Value) -> error::Result<()> {
        let mut w = self.write.borrow_mut();
        rmpv::encode::write_value(w.deref_mut(), &Value::Array(vec![
            Value::from(RESPONSE_CODE),
            Value::from(msgid),
            error,
            result,
        ]))?;
        drop(w);
        return Ok(());
    }
    pub async fn call_fn_wv<R>(&self, fn_name: String, args: impl ValueSeq) -> error::Result<R>
    where 
        R: TryFromValue
    {
        let msg_id = self.get_next_msg_id();
        let request = msgrpc::create_request_value(msg_id, fn_name, args);
        let mut w = self.write.borrow_mut();
        rmpv::encode::write_value(w.deref_mut(), &request)?;
        let (sender, rx) = oneshot::channel::<Value>();
        let msg = MsgToReader::new(msg_id, sender);
        // self.tx.send(msg).await?;
        // let rv = rx.await?;
        // return R::try_from_value(rv);
        return error::with_msg("return value not implemented yet.");
    }

    pub async fn call_fn<D,S>(&self, fn_name: &str, args: S) -> error::Result<D>
    where 
        D: Deserialize<'static>,
        S: SerialSeq,
    {
        let msg_id = self.get_next_msg_id();
        let request = msgrpc::create_request_ser(msg_id, fn_name, args);
        let mut w = self.write.borrow_mut();
        rmp_serde::encode::write_named(w.deref_mut(), &request)?;
        let (sender, rx) = oneshot::channel::<Value>();
        let msg = MsgToReader::new(msg_id, sender);
        // self.tx.send(msg).await?;
        // let rv = rx.await?;
        // return Ok(D::deserialize(rv)?);
        return error::with_msg("return value not implemented yet.");
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
    String, Vec<(Value, Value)>, Vec<u8>, bool, f32, f64, i64, u64,
);
impl TryFromValue for Value {
    fn try_from_value(value: Value) -> error::Result<Self> {
        Ok(value)
    }
}
// specialize of Value and (Value,Value)
// when specialization arrives
impl<V: TryFromValue> TryFromValue for Vec<V> {
    fn try_from_value(value: Value) -> error::Result<Self> where Self: Sized {
        let Value::Array(array) = value else {return error::with_msg("expected array.")};
        let mut rv = Vec::with_capacity(array.len());
        for value in array {
            let v = V::try_from_value(value)?;
            rv.push(v);
        }
        return Ok(rv);
    }
}

// impl TryFromValue for Vec<String> {
//     fn try_from_value(value: Value) -> error::Result<Self> {
//         let Value::Array(array) = value else {return error::with_msg("expected array.")};
//         let mut rv = Vec::with_capacity(array.len());
//         for value in array {
//             let Value::String(value) = value else {return error::with_msg("expected String")};
//             let Some(value) = value.into_str() else { return error::with_msg("string not utf8"); };
//             rv.push(value);
//         }
//         return Ok(rv);
//     }
// }
// impl TryFromValue for Vec<i64> {
//     fn try_from_value(value: Value) -> error::Result<Self> {
//         let Value::Array(array) = value else {return error::with_msg("expected array.")};
//         let mut rv = Vec::with_capacity(array.len());
//         for value in array {
//             let Ok(value) = i64::try_from(value) else {return error::with_msg("expected i64")};
//             rv.push(value);
//         }
//         return Ok(rv);
//     }
// }
// impl TryFromValue for Vec<Vec<(Value, Value)>> {
//     fn try_from_value(value: Value) -> error::Result<Self> where Self: Sized {
//         let Value::Array(array) = value else { return error::with_msg("expected array.") };
//         let mut rv = Vec::with_capacity(array.len());
//         for map in array {
//             let Value::Map(map) = map else {return error::with_msg("expected map.")};
//             rv.push(map);
//         }
//         return Ok(rv);
//     }
// }
impl TryFromValue for () {
    fn try_from_value(_: Value) -> error::Result<Self> where Self: Sized {
        Ok(())
    }
}
