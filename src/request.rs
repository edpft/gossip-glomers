use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Request {
    pub src: String,
    pub dest: String,
    pub body: RequestBody,
}

impl Request {
    pub fn from_string(string: String) -> color_eyre::Result<Self> {
        let request: Self = serde_json::from_str(&string)?;
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
