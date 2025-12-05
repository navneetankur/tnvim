use core::ops::ControlFlow;
use std::{collections::VecDeque, os::unix::net::UnixStream};
use log::{debug, warn};
use tokio::sync::mpsc;
use crate::{MsgToReader, PendingRequest, handler::MsgForHandler, msgrpc::Message};


pub fn readloop(
    mut reader: UnixStream,
    rx: std::sync::mpsc::Receiver<MsgToReader>,
    tx: mpsc::Sender<MsgForHandler>,
) {
    let mut pending_requests = VecDeque::<PendingRequest>::new();
    'outer: loop {
        if ControlFlow::Break(()) == check_messages_from_handler(&rx, &mut pending_requests) {
            break 'outer;
        }
        let message: Message = rmp_serde::decode::from_read(&mut reader).unwrap();
        match message {
            Message::Request(request) => {
                let msg = MsgForHandler::Request(Box::new(request));
                tx.blocking_send(msg).unwrap();
            },
            Message::Response(response) => {
                if ControlFlow::Break(()) == check_messages_from_handler(&rx, &mut pending_requests) {
                    break 'outer;
                }
                let msgid = response.msgid;
                // request is supposed to be in order?. I think.
                // So no need to check whole queue first one should be it.
                let corres_request = pending_requests.pop_front().expect("How did i got a response when there is no request?");
                assert_eq!(msgid, corres_request.msg_id, "is response coming out of order. Should i check whole queue?");
                if response.error.is_nil() {
                    corres_request.sender.send(response.result).unwrap();
                } else {
                    corres_request.sender.send(response.error).unwrap();
                }
            },
            Message::Notification(notify) => {
                tx.blocking_send(MsgForHandler::Notification(notify)).unwrap();
            },
        }
    }
}

fn check_messages_from_handler(rx: &std::sync::mpsc::Receiver<MsgToReader>, pending_requests: &mut VecDeque<PendingRequest>) -> ControlFlow<()> {
    loop { // check for internal messages. Probably from handler.
        match rx.try_recv() {
            Ok(msgfh) => {
                match msgfh {
                    MsgToReader::PendingRequest(pending_request) => {
                        pending_requests.push_back(pending_request);
                    },
                    MsgToReader::End => {
                        return ControlFlow::Break(());
                    },
                }
            },
            Err(e) => {
                match e {
                    std::sync::mpsc::TryRecvError::Empty => break,
                    std::sync::mpsc::TryRecvError::Disconnected => {
                        warn!("channel to handler gone.");
                        return ControlFlow::Break(());
                    },
                }
            },
        }
    }
    return ControlFlow::Continue(());
}
