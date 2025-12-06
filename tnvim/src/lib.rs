use std::os::unix::net::UnixStream;
use tokio::runtime::LocalRuntime;

use crate::app::App;
mod app;
mod nvim;

const SERVER: &str = "/run/user/1000/nvim-server.s";
pub fn main() {
    let rt = LocalRuntime::new().unwrap();
    rt.block_on(main_async(&rt));
}
async fn main_async(rt: &LocalRuntime) {
    println!("hello world");
    let stream = UnixStream::connect(SERVER).unwrap();
    nvimapi::manager::start(App::new(), rt, stream.try_clone().unwrap(), stream).await;
}
