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
pub use generated::UiEvent;
pub use generated::uievent;
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

const SERVER_PATH: &str = "/run/user/1000/nvim-server.s";
pub fn main() {
    let rt = tokio::runtime::LocalRuntime::new().unwrap();
    let rt = Rc::new(rt);
    let _guard = rt.enter();
    rt.block_on(main_sync(rt.clone()));
}

pub async fn main_sync(rt: Rc<LocalRuntime>) {
    let writer = std::os::unix::net::UnixStream::connect(SERVER_PATH).unwrap();
    let reader = writer.try_clone().unwrap();
    manager::start(TestH, &rt, reader, writer).await;
}

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
        use nvimapi::NvimapiNr;
        // nvim.ui_attach(64, 64, [();0]).await.unwrap();
        nvim.noret().ui_attach(64, 64, Pairs::new().with("rgb", true)).unwrap();
        // let atach = nvim.ui_attach(64, 64, Pairs::new().with("rgb", true));
        // let atach = nvim.ui_attach(64, 64, Pairs::new().with("rgb", true));
        // let pa = Box::pin(atach);
        // PollOnce{inner: pa}.await;
        // nvim.ui_attach(64, 64, Pairs::new().with("rgb", 42)).await.unwrap();
        debug!("attached");
        // let w: Value = nvim.call_fn_wv("nvim_strwidth".into(), "fsajll".into()).await.unwrap();
        let w = nvim.noret().strwidth("hello").unwrap();
        let w = nvim.noret().strwidth("hello").unwrap();
        let w = nvim.strwidth("hellooooooo").await.unwrap();
        debug!("w: {w}");
    }
}
struct PollOnce<F: Future> {
    inner: F,
}
// impl<F: std::future::Future> Future for PollOnce<F> {
//     type Output = Option<F::Output>;

//     fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
//         let this = unsafe{self.get_unchecked_mut()};
//         let inner = unsafe { std::pin::Pin::new_unchecked(&mut this.inner) };
//         match Future::poll(inner, cx) {
//             core::task::Poll::Ready(o) => return core::task::Poll::Ready(Some(o)),
//             core::task::Poll::Pending => return core::task::Poll::Ready(None),
//         };
//     }
// }

impl<F> Future for PollOnce<F>
where
    F: Future + Unpin,
{
    type Output = core::task::Poll<F::Output>;

    fn poll(mut self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Self::Output> {
        core::task::Poll::Ready(core::pin::Pin::new(&mut self.inner).poll(cx))
    }
}
