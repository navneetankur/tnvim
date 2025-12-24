use core::ops::Deref;
use std::rc::Rc;

use crate::{msgrpc::Request, nvimapi::{Nvimapi, notification::Notification}};
#[allow(async_fn_in_trait)]
pub trait Handler {
    async fn notify(&self, nvim: &impl Nvimapi, notification: Notification);
    async fn request(&self, nvim: &impl Nvimapi, request: Box<Request>);
    async fn init(&self, nvim: &impl Nvimapi);
}

impl<H: Handler> Handler for Rc<H> {
    async fn notify(&self, nvim: &impl Nvimapi, notification: Notification) {
        self.deref().notify(nvim, notification).await
    }
    async fn request(&self, nvim: &impl Nvimapi, request: Box<Request>) {
        self.deref().request(nvim, request).await
    }
    async fn init(&self, nvim: &impl Nvimapi) {
        self.deref().init(nvim).await
    }
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
