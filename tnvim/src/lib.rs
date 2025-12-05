const SERVER: &str = "/run/user/1000/nvim-server.s";
pub fn main() {
    let rt = tokio::runtime::LocalRuntime::new().unwrap();
    rt.block_on(main_async());
}
async fn main_async() {
}
