use std::{collections::HashMap, io};

use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Request {
    pub src: String,
    pub dest: String,
    pub body: RequestBody,
}

impl Request {
    pub fn from_stdin() -> color_eyre::Result<Self> {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        let request: Self = serde_json::from_str(&buffer)?;
        Ok(request)
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum RequestBody {
    Init {
        msg_id: usize,
        node_id: String,
        node_ids: Vec<String>,
    },
    Echo {
        msg_id: usize,
        echo: String,
    },
    Generate {
        msg_id: usize,
    },
    Broadcast {
        msg_id: usize,
        message: usize,
    },
    Read {
        msg_id: usize,
    },
    Topology {
        msg_id: usize,
        topology: HashMap<String, Vec<String>>,
    },
}

impl RequestBody {
    pub fn kind(&self) -> String {
        let str = match self {
            RequestBody::Echo { .. } => "echo",
            RequestBody::Init { .. } => "init",
            RequestBody::Generate { .. } => "generate",
            RequestBody::Broadcast { .. } => "broadcast",
            RequestBody::Read { .. } => "read",
            RequestBody::Topology { .. } => "topology",
        };
        str.to_string()
    }
}
