use log::debug;
use nvim_rs::{Handler, Neovim};
use rmpv::Value;

#[derive(Clone)]
pub struct RequestHandler {

}
impl Handler for RequestHandler {
    #[doc = " The type where we write our responses to requests. Handling of incoming"]
    #[doc = " requests/notifications is done on the io loop, which passes the parsed"]
    #[doc = " messages to the handler."]
    type Writer = nvim_rs::compat::tokio::Compat<tokio::net::unix::OwnedWriteHalf>;

    async fn handle_request(&self,name:String,_args:Vec<Value> ,_neovim:Neovim<Self::Writer> ,) ->  Result<Value,Value>
    {
        todo!("{name}")
    }

    async fn handle_notify(
        &self,
        name: String,
        args: Vec<Value>,
        _neovim: Neovim<<Self as Handler>::Writer>,
      ) {
        debug!("name: {name}");
        for arg in args {
            debug!("arg: {arg}");
        }

      }



}
