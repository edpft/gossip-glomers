use std::fmt;

use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Response {
    src: String,
    dest: String,
    body: ResponseBody,
}

impl Response {
    pub fn new(src: impl Into<String>, dest: impl Into<String>, body: ResponseBody) -> Self {
        Self {
            src: src.into(),
            dest: dest.into(),
            body,
        }
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // fmt::Error does not support transmitting any information about an error other than that the error occurred.
        let string = serde_json::to_string(self).map_err(|_| fmt::Error)?;
        write!(f, "{}", string)
    }
}

#[derive(Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum ResponseBody {
    InitOk {
        in_reply_to: usize,
    },
    EchoOk {
        msg_id: usize,
        in_reply_to: usize,
        echo: String,
    },
    GenerateOk {
        in_reply_to: usize,
        id: Uuid,
    },
}
