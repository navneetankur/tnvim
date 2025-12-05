use std::os::unix::net::UnixStream;
use tokio::sync::mpsc;
use crate::{MsgToReader, PendingRequest};

pub(crate) struct ReadLoop {
    // reader: UnixStream,
    // rx: mpsc::Receiver<MsgToReader>,
    // msg_id: u32,
    pub(crate) pending_requests: Vec<PendingRequest>,
}

impl ReadLoop {
    pub fn start(&mut self, mut reader: UnixStream, rx: mpsc::Receiver<MsgToReader>) {
        loop {
            let value = rmpv::decode::read_value(&mut reader).unwrap();
            println!("{value}");
        }
    }
}
