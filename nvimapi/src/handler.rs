use std::{io::Write, os::unix::net::UnixStream};

use crate::{Nvimapi, msgrpc::Request, nvimapi::notification::Notification};

pub trait Handler {
    type Write: std::io::Write;
    async fn notify(&self, nvim: &Nvimapi<Self::Write>, notification: Notification);
    async fn request(&self, nvim: &Nvimapi<Self::Write>, request: Box<Request>);
    async fn init(&self, nvim: &Nvimapi<Self::Write>);
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
