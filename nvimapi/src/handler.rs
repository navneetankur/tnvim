use crate::{Nvimapi, msgrpc::Request, nvimapi::notification::Notification};

pub trait Handler {
    async fn notify(&self, nvim: &Nvimapi, notification: Notification);
    async fn request(&self, nvim: &Nvimapi, request: Box<Request>);
    async fn init(&self, nvim: &Nvimapi);
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
