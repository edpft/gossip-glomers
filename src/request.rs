use std::io;

use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
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
}

impl RequestBody {
    pub fn kind(&self) -> String {
        let str = match self {
            RequestBody::Echo { .. } => "echo",
            RequestBody::Init { .. } => "init",
        };
        str.to_string()
    }
}
