use uuid::Uuid;

use crate::{
    error::Error,
    request::{Request, RequestBody},
    response::{Response, ResponseBody},
};

pub struct Server {
    node_id: String,
    msg_id: usize,
}

impl Server {
    fn new(node_id: impl Into<String>) -> Self {
        Self {
            node_id: node_id.into(),
            msg_id: 0,
        }
    }

    pub fn from_initial_request(initial_request: &Request) -> color_eyre::Result<Self> {
        match &initial_request.body {
            RequestBody::Init {
                msg_id: _,
                node_id,
                node_ids: _,
            } => {
                let server = Server::new(node_id);
                Ok(server)
            }
            request_body => {
                let request_kind = request_body.kind();
                let error = Error::Initialisation(request_kind);
                Err(error)?
            }
        }
    }

    pub fn handle_initial_request(&mut self, request: Request) -> color_eyre::Result<Response> {
        match request.body {
            RequestBody::Init {
                msg_id,
                node_id: _,
                node_ids: _,
            } => {
                let response_body = ResponseBody::Init {
                    in_reply_to: msg_id,
                };
                let response = Response::new(&self.node_id, request.src, response_body);
                self.msg_id += 1;
                Ok(response)
            }
            request_body => {
                let request_kind = request_body.kind();
                let error = Error::Initialisation(request_kind);
                Err(error)?
            }
        }
    }

    pub fn handle_request(&mut self, request: Request) -> color_eyre::Result<Response> {
        let response_body = match request.body {
            RequestBody::Init { .. } => {
                let error = Error::AlreadyInitialised;
                Err(error)?
            }
            RequestBody::Echo { msg_id, echo } => ResponseBody::Echo {
                msg_id: self.msg_id,
                in_reply_to: msg_id,
                echo,
            },
            RequestBody::Generate { msg_id } => {
                let id = Uuid::new_v4();
                ResponseBody::Generate {
                    in_reply_to: msg_id,
                    id,
                }
            }
        };
        let response = Response::new(&self.node_id, request.src, response_body);
        self.msg_id += 1;
        Ok(response)
    }
}
