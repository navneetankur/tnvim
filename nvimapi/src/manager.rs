use std::{os::unix::net::UnixStream, rc::Rc};

use tokio::{runtime::LocalRuntime, sync::mpsc};
use crate::{MsgToReader, Nvimapi, handler::{Handler, MsgForHandler}, msgrpc::Request, nvimapi::notification::Notification, readloop};

pub async fn loopy<H>(
    mut rx: mpsc::Receiver<MsgForHandler>,
    nvim: Rc<Nvimapi>,
    handler: H,
    rt: Rc<LocalRuntime>,
)
where 
    H: Handler + 'static
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
}

pub async fn start<H>(
    handler: H,
    rt: Rc<LocalRuntime>,
    reader: UnixStream,
    writer: UnixStream,
)
where 
    H: Handler + 'static
{
    let (tx_to_handler, rx_for_handler) = mpsc::channel::<MsgForHandler>(10);
    let (tx_to_reader, rx_for_reader) = std::sync::mpsc::sync_channel::<MsgToReader>(10);
    std::thread::spawn(|| {
        readloop::readloop(reader, rx_for_reader, tx_to_handler);
    });
    let nvim = Nvimapi {
        tx_to_reader,
        msgid: Default::default(),
        write: core::cell::RefCell::new(writer),
    };
    loopy(rx_for_handler, Rc::new(nvim), handler, rt).await;
}
async fn send_request_to_handler(nvim: Rc<Nvimapi>,handler: Rc<impl Handler>, request: Box<Request>) {
    handler.request(&nvim, request).await
}
async fn send_notification_to_handler(nvim: Rc<Nvimapi>, handler: Rc<impl Handler>, notification: Notification) {
    handler.notify(&nvim, notification).await
}
async fn init_handler(nvim: Rc<Nvimapi>, handler: Rc<impl Handler>,) {
    handler.init(&nvim).await
}
