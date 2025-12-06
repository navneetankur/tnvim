use core::ops::ControlFlow;
use std::{collections::VecDeque, io::Read};
use log::{debug, warn};
use tokio::sync::mpsc;
use crate::{MsgToReader, PendingRequest, handler::MsgForHandler, msgrpc::Message};


pub fn readloop<R: Read>(
    mut reader: R,
    mut rx: mpsc::Receiver<MsgToReader>,
    tx: mpsc::Sender<MsgForHandler>,
) {
    let mut unprocessed_request = Option::<PendingRequest>::None;
    'outer: loop {
        let message: Message = rmp_serde::decode::from_read(&mut reader).unwrap();
        match message {
            Message::Request(request) => {
                let msg = MsgForHandler::Request(Box::new(request));
                tx.blocking_send(msg).unwrap();
            },
            Message::Response(response) => {
                let msgid = response.msgid;
                let corres_request = 
                    if let Some(unprocessed_request) = unprocessed_request.take() {
                        unprocessed_request
                    } else {
                        rx.try_recv().unwrap().pending_request()
                    };
                if msgid != corres_request.msg_id {
                    debug!("response for msgid: {msgid}, with no receiver");
                    unprocessed_request = Some(corres_request);
                }
                else if let Err(_) = corres_request.sender.send(response.result) {
                    warn!("return value channel dropped for msg id: {}", corres_request.msg_id);
                }
            },
            Message::Notification(notify) => {
                if let Err(e) = tx.try_send(MsgForHandler::Notification(notify)) {
                    match e {
                        mpsc::error::TrySendError::Full(e) => {
                            let MsgForHandler::Notification(notify) = e else {unreachable!()};
                            warn!("channel to ui full, dropped notification {}", notify.name());
                        },
                        mpsc::error::TrySendError::Closed(_) => {
                            warn!("channel gone");
                            break 'outer;
                        },
                    }
                }
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
                    MsgToReader::Other => {unimplemented!()},
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
