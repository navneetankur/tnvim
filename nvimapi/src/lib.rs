#![feature(anonymous_lifetime_in_impl_trait)]
#![feature(type_alias_impl_trait)]
#![feature(trim_prefix_suffix)]
#![feature(iter_array_chunks)]
// mod out;
mod manualser;
pub use manualser::{color::Color};
mod contseq;
mod pairs;
pub use pairs::Pairs;
use rmpv::Value;
mod generated;
mod nvimapi;
pub use generated::UiEvent;
pub use generated::uievent;
pub use generated::UiOptions;
pub use nvimapi::{Nvimapi, NvimapiNr, notification::Notification,};
mod handler;
pub use handler::Handler;
pub mod manager;
mod readloop;
mod msgrpc;
pub use msgrpc::Request;
mod valueseq;
pub use nvimapi::Nvimrpc;
pub mod error;
pub use nvimapi::TryFromValue;

// const SERVER_PATH: &str = "/run/user/1000/nvim-server.s";
// pub fn main() {
//     let rt = tokio::runtime::LocalRuntime::new().unwrap();
//     let rt = Rc::new(rt);
//     let _guard = rt.enter();
//     rt.block_on(main_sync(rt.clone()));
// }

// pub async fn main_sync(rt: Rc<LocalRuntime>) {
//     let writer = std::os::unix::net::UnixStream::connect(SERVER_PATH).unwrap();
//     let reader = writer.try_clone().unwrap();
//     manager::start_async(TestH, &rt, reader, writer).await;
// }

pub enum MsgToReader {
    PendingRequest(PendingRequest),
    Other,
}
impl MsgToReader {
    pub fn pending_request(self) -> PendingRequest {
        let MsgToReader::PendingRequest(this) = self else {unimplemented!()};
        return this;
    }
}
impl MsgToReader {
    pub fn new(msg_id: u32, sender: tokio::sync::oneshot::Sender<Result<Value, Value>>) -> Self {
        Self::PendingRequest(PendingRequest { msg_id, sender })
    }
}
pub struct PendingRequest {
    msg_id: u32,
    sender: tokio::sync::oneshot::Sender<core::result::Result<Value, Value>>,
}
