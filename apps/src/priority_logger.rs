use serde::{Deserialize, Serialize};
use serde_json::to_writer;
use std::fs::File;
use std::io::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct PriorityLogMsg {
    pub stream_id: u64,
    pub urgency: u8,
    pub incremental: bool,
    pub headers: Vec<(String, String)>,
    pub content_type: String,
}

// This is a sort of class:
//  this struct holds a vector of log messages. it exposes a few public functions:
//   new: creates a new object (instantiates an empty vector)
//   add_msg: creates a new PriorityLogMsg object and pushes it to the vector
//   write_to_json: writes the objects to a JSON file
pub struct PriorityLogger {
    msgs: Vec<PriorityLogMsg>,
    json_location: String,
}
impl PriorityLogger {
    pub fn new(json_location: String) -> Self {
        Self {
            msgs: vec![],
            json_location,
        }
    }

    // Adds a message to the msgs vector
    // all it does it create a PriorityLogMsg object from the args, and pushes it
    pub fn add_msg(
        &mut self, stream_id: u64, priority: &quiche::h3::Priority,
        headers: Vec<(String, String)>, content_type: String,
    ) {
        let (urg, inc) = priority.get_fields();
        let log_msg = PriorityLogMsg {
            stream_id,
            urgency: urg,
            incremental: inc,
            headers,
            content_type,
        };
        self.msgs.push(log_msg);
    }

    // writes the vector to a JSON file
    pub fn write_to_json(&self) -> Result<()> {
        let file = File::create(&self.json_location)?;
        to_writer(file, &self.msgs)?;
        Ok(())
    }
}
