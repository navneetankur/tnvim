use std::os::unix::net::UnixStream;
use log::debug;
use nvim_rs::UiAttachOptions;
use nvimapi;
use rmpv::Value;
use tokio::runtime::LocalRuntime;
mod handler;
use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};

const SERVER: &str = "/run/user/1000/nvim-server.s";
pub fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
    rt.block_on(main_async());
}
async fn main_async() {
    let stream = tokio::net::UnixStream::connect(SERVER).await.unwrap();
    let (r,w) = stream.into_split();
    let handler = handler::RequestHandler{};
    let w = w.compat_write();
    let r = r.compat();
    let (nvim, io_handler) = nvim_rs::Neovim::new(r, w, handler);
    debug!("attaching");
    nvim.ui_attach(65, 65, &UiAttachOptions::new()).await.unwrap();
    debug!("attached");

    // Any error should probably be logged, as stderr is not visible to users.
    io_handler.await.unwrap();
}
