use crate::types::{InputLogEvent, LogStream};
use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct Context {
    pub groups: HashMap<String, Group>,
}

#[derive(Debug, Clone, Default)]
pub struct Group {
    pub name: String,
    pub streams: Vec<Stream>,
}

#[derive(Debug, Clone, Default)]
pub struct Stream {
    pub name: String,
    pub logs: Vec<InputLogEvent>,
}

impl From<Stream> for LogStream {
    fn from(stream: Stream) -> Self {
        LogStream {
            log_stream_name: Some(stream.name),
            ..Default::default()
        }
    }
}
