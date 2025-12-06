#![feature(anonymous_lifetime_in_impl_trait)]
#![feature(type_alias_impl_trait)]
#![feature(trim_prefix_suffix)]
// mod out;
mod contseq;
mod pairs;
use log::debug;
pub use pairs::Pairs;
use tokio::runtime::LocalRuntime;
use std::rc::Rc;
use rmpv::Value;
mod generated;
mod nvimapi;
mod handler;
mod manager;
mod readloop;
mod msgrpc;
mod valueseq;
pub use nvimapi::Nvimrpc;
pub mod error;
pub use nvimapi::TryFromValue;

use crate::handler::Handler;
use crate::nvimapi::Nvimapi;

const SERVER_PATH: &str = "/run/user/1000/nvim-server.s";
pub fn main() {
    let rt = tokio::runtime::LocalRuntime::new().unwrap();
    let rt = Rc::new(rt);
    rt.block_on(main_sync(rt.clone()));
}

pub async fn main_sync(rt: Rc<LocalRuntime>) {
    let writer = std::os::unix::net::UnixStream::connect(SERVER_PATH).unwrap();
    let reader = writer.try_clone().unwrap();
    manager::start(TestH, rt, reader, writer).await;
}

pub enum MsgToReader {
    PendingRequest(PendingRequest),
    Other,
}
impl MsgToReader {
    pub fn new(msg_id: u32, sender: tokio::sync::oneshot::Sender<Result<Value, Value>>) -> Self {
        Self::PendingRequest(PendingRequest { msg_id, sender })
    }
}
struct PendingRequest {
    msg_id: u32,
    sender: tokio::sync::oneshot::Sender<core::result::Result<Value, Value>>,
}

struct TestH;
impl Handler for TestH {
    async fn notify(&self, _nvim: &impl Nvimapi, notification: nvimapi::notification::Notification) {
        match notification {
            nvimapi::notification::Notification::Redraw(ui_events) => {
                for event in ui_events {
                    println!("got {}", event.name());
                }
            },
            nvimapi::notification::Notification::Unknown(_) => todo!(),
        }
    }

    async fn request(&self, _nvim: &impl Nvimapi, request: Box<msgrpc::Request>) {
        println!("request: {request:?}");
    }

    async fn init(&self, nvim: &impl Nvimapi) {
        nvim.ui_attach(64, 64, [();0]).await.unwrap();
        debug!("attached");
        let w = nvim.strwidth("hillo").await.unwrap();
        debug!("w: {w}");
    }
}
