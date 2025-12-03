use std::os::unix::net::UnixStream;
use nvimapi;
use rmpv::Value;
use tokio::runtime::LocalRuntime;
mod handler;

const SERVER: &str = "/run/user/1000/nvim-server.s";
pub fn main() {
    let rt = LocalRuntime::new().unwrap();
    let mut stream = UnixStream::connect(SERVER).unwrap();
    // let mut nvimapi = NvimRpc::new(stream);
    // nvimapi.nvim_ui_attach().unwrap();
    // nvimapi.call_fn("nvim_get_commands", &()).unwrap();
    let m = Value::Map(vec![]);
    let to_send = rmpv::Value::Array(vec![
        Value::from(0),
        Value::from(1),
        Value::from("nvim_get_commands"),
        Value::Array(vec![
            m
        ]),
    ]);
    rmpv::encode::write_value(&mut stream, &to_send).unwrap();
    loop {
        let value = rmpv::decode::read_value(&mut stream).unwrap();
        // let value = nvimapi.read_temp();
        // let value = nvimapi.read_temp2();
        println!("{value:?}");
    }
}
async fn main_async() {

}
