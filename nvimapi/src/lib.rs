#![feature(anonymous_lifetime_in_impl_trait)]
#![feature(type_alias_impl_trait)]
#![feature(trim_prefix_suffix)]
// mod out;
mod contseq;
mod pairs;
use log::debug;
pub use pairs::Pairs;
use core::ops::ControlFlow;
use std::io::{Write, stdout};
use rmpv::Value;
mod generated;
mod nvimapi;
mod manager;
mod readloop;
mod msgrpc;
mod valueseq;
pub use nvimapi::Nvimapi;
pub mod error;
pub use nvimapi::TryFromValue;

const SERVER_PATH: &str = "/run/user/1000/nvim-server.s";
pub fn main() {
    let rt = tokio::runtime::LocalRuntime::new().unwrap();
    rt.block_on(main_sync());
}

pub async fn main_sync() {
    let stream = std::os::unix::net::UnixStream::connect(SERVER_PATH).unwrap();
    let (tx, rx) = tokio::sync::mpsc::channel(10);
    let nvim = nvimapi::Nvimapi {
        tx: tx,
        msgid: core::cell::Cell::new(0),
        write: core::cell::RefCell::new(stream.try_clone().unwrap()),
    };
    debug!("attaching");
    nvim.ui_attach(64, 64, [();0]).await;
    debug!("attached");
    let mut readloop = readloop::ReadLoop {
        pending_requests: Default::default(),
    };
    readloop.start(stream, rx);

}

pub enum MsgToReader {
    PendingRequest(PendingRequest),
    End,
}
impl MsgToReader {
    pub fn new(msg_id: u32, sender: tokio::sync::oneshot::Sender<Value>) -> Self {
        Self::PendingRequest(PendingRequest { msg_id, sender })
    }
}
struct PendingRequest {
    msg_id: u32,
    sender: tokio::sync::oneshot::Sender<Value>,
}
