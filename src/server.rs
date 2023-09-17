use std::collections::HashSet;

use uuid::Uuid;

use crate::{
    error::Error,
    request::{Request, RequestPayload},
    response::{Response, ResponseBody, ResponsePayload},
};

pub struct Server {
    node_id: String,
    msg_id: usize,
    messages_seen: Option<HashSet<usize>>,
}

impl Server {
    fn new(node_id: impl Into<String>) -> Self {
        Self {
            node_id: node_id.into(),
            msg_id: 0,
            messages_seen: None,
        }
    }

    pub fn handle_initial_request(&mut self, request: Request) -> color_eyre::Result<Response> {
        match request.body.payload {
            RequestPayload::Init {
                node_id: _,
                node_ids: _,
            } => {
                let response_payload = ResponsePayload::Init;
                let response_body =
                    ResponseBody::new(self.msg_id, request.body.msg_id, response_payload);
                let response = Response::new(&self.node_id, request.src, response_body);
                self.msg_id += 1;
                Ok(response)
            }
            _ => {
                let error = Error::Initialisation;
                Err(error)?
            }
        }
    }

    pub fn handle_request(&mut self, request: Request) -> color_eyre::Result<Response> {
        let response_payload = match request.body.payload {
            RequestPayload::Init { .. } => {
                let error = Error::AlreadyInitialised;
                Err(error)?
            }
            RequestPayload::Echo { echo } => ResponsePayload::Echo { echo },
            RequestPayload::Generate => {
                let id = Uuid::new_v4();
                ResponsePayload::Generate { id }
            }
            RequestPayload::Broadcast { message } => {
                if self.messages_seen.is_none() {
                    let messages_seen = HashSet::new();
                    self.messages_seen = Some(messages_seen);
                };
                self.messages_seen
                    .as_mut()
                    .map(|messages_seen| messages_seen.insert(message));
                ResponsePayload::Broadcast
            }
            RequestPayload::Read => {
                let messages = self.messages_seen.clone();
                ResponsePayload::Read { messages }
            }
            RequestPayload::Topology { .. } => ResponsePayload::Topology,
        };
        let response_body = ResponseBody::new(self.msg_id, request.body.msg_id, response_payload);
        let response = Response::new(&self.node_id, request.src, response_body);
        self.msg_id += 1;
        Ok(response)
    }
}

impl TryFrom<&Request> for Server {
    type Error = Error;

    fn try_from(request: &Request) -> Result<Self, Self::Error> {
        match &request.body.payload {
            RequestPayload::Init {
                node_id,
                node_ids: _,
            } => {
                let server = Server::new(node_id);
                Ok(server)
            }
            _ => {
                let error = Error::Initialisation;
                Err(error)?
            }
        }
    }
}
