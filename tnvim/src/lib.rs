use std::os::unix::net::UnixStream;
use log::debug;
use nvim_rs::UiAttachOptions;
use nvimapi;
use rmpv::Value;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, runtime::LocalRuntime};
mod handler;
use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};

const SERVER: &str = "/run/user/1000/nvim-server.s";
pub fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
    rt.block_on(main_async());
}
async fn main_async() {
    let mut stream = UnixStream::connect(SERVER).unwrap();
    let handler = handler::RequestHandler{};
    // let (nvim, io_handler) = nvim_rs::Neovim::new(r, w, handler);
    debug!("attaching");
    let attach = Value::Array(vec![
        Value::from(0), // request
        Value::from(1), //msg_id
        Value::from("nvim_ui_attach"), //fn_ name
        Value::Array(vec![
            Value::from(64), //width
            Value::from(64), //height
            Value::Map(
                Vec::new(),
            ),
        ])
    ]);
    rmpv::encode::write_value(&mut stream, &attach).unwrap();
    loop {
        let val = rmpv::decode::read_value(&mut stream).unwrap();
        println!("{val:?}");
    }

    // nvim.ui_attach(65, 65, &UiAttachOptions::new()).await.unwrap();
    // debug!("attached");

    // Any error should probably be logged, as stderr is not visible to users.
    // io_handler.await.unwrap();
}
