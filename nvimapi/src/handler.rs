use std::{io::Write, os::unix::net::UnixStream};

use crate::{Nvimrpc, msgrpc::Request, nvimapi::{Nvimapi, notification::Notification}};

pub trait Handler {
    async fn notify(&self, nvim: &impl Nvimapi, notification: Notification);
    async fn request(&self, nvim: &impl Nvimapi, request: Box<Request>);
    async fn init(&self, nvim: &impl Nvimapi);
}

pub enum MsgForHandler {
    Request(Box<Request>),
    Notification(Notification),
}

#[cfg(test)]
mod tests {
    use core::mem::size_of;

    use crate::handler::MsgForHandler;
    #[test]
    fn size_of_message() {
        assert!(size_of::<MsgForHandler>() <= 32);
    }

}
