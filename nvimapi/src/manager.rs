use std::{io::{Read, Write}, rc::Rc};
use log::debug;
use tokio::{runtime::LocalRuntime, sync::mpsc};
use crate::{MsgToReader, Nvimapi, Nvimrpc, handler::{Handler, MsgForHandler}, msgrpc::Request, nvimapi::notification::Notification, readloop};
use core::ops::Deref;

async fn loopy<H, W>(
    mut rx: mpsc::Receiver<MsgForHandler>,
    nvim: Rc<Nvimrpc<W>>,
    handler: H,
    rt: Rc<LocalRuntime>,
)
where 
    W: Write + 'static,
    H: Handler + 'static,
{
    use MsgForHandler as Mfh;
    let handler = Rc::new(handler);
    rt.spawn_local(init_handler(nvim.clone(), handler.clone()));
    while let Some(msg) = rx.recv().await {
        match msg {
            Mfh::Request(request) => {
                rt.spawn_local(send_request_to_handler(nvim.clone(), handler.clone(), request));
            },
            Mfh::Notification(notification) => {
                rt.spawn_local(send_notification_to_handler(nvim.clone(), handler.clone(), notification));
            },
        }
    }
    debug!("channel closed");
}

pub async fn start_async<H, W>(
    handler: H,
    rt: Rc<LocalRuntime>,
    reader: impl Read + Send + 'static,
    writer: W,
)
where 
    W: Write + 'static,
    H: Handler + 'static,
{
    start(handler, rt, reader, writer).0.await
}
async fn send_request_to_handler<W: Write>(nvim: Rc<Nvimrpc<W>>,handler: Rc<impl Handler>, request: Box<Request>) {
    handler.request(nvim.deref(), request).await
}
async fn send_notification_to_handler<W: Write>(nvim: Rc<Nvimrpc<W>>, handler: Rc<impl Handler>, notification: Notification) {
    handler.notify(nvim.deref(), notification).await
}
async fn init_handler<W:Write>(nvim: Rc<Nvimrpc<W>>, handler: Rc<impl Handler>,) {
    handler.init(nvim.deref()).await
}
pub fn start<H, W>(
    handler: H,
    rt: Rc::<LocalRuntime>,
    reader: impl Read + Send + 'static,
    writer: W,
) -> (impl Future<Output = ()>, Rc<impl Nvimapi>)
where 
    W: Write + 'static,
    H: Handler + 'static,
{
    let (tx_to_handler, rx_for_handler) = mpsc::channel::<MsgForHandler>(10);
    let (tx_to_reader, rx_for_reader) = mpsc::channel::<MsgToReader>(10);
    std::thread::spawn(|| {
        readloop::readloop(reader, rx_for_reader, tx_to_handler);
    });
    let nvim = Nvimrpc {
        tx_to_reader,
        msgid: Default::default(),
        write: core::cell::RefCell::new(writer),
    };
    let nvim = Rc::new(nvim);
    return (loopy(rx_for_handler, nvim.clone(), handler, rt), nvim.clone(),);
}
