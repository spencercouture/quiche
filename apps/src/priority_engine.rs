use crate::custom_cache;
use crate::custom_cache::CacheKey;
use crate::priority_engine;
use serde::{Deserialize, Serialize};
use serde_json::to_writer;
use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Result};
use std::io::{ErrorKind, Write};
use std::path::Path;

// this "class" holds a few important fields:
//  - cache: a hashmap of cache-keys to a list of entries (our usual protobuf cache)
//  - map: a map of cache-keys to priority values
//  - logger: a class object-like structure that holds a list of messages and a file name
//  - read_from_input: a bool that decides whether or not to read from our input file ("apply_map")
pub struct PriorityContext {
    pub cache: HashMap<custom_cache::CacheKey, Vec<custom_cache::CacheEntry>>,

    pub map: HashMap<custom_cache::CacheKey, (u8, bool)>,

    pub logger: priority_engine::PriorityLogger,

    pub read_from_input: bool,
}
impl PriorityContext {
    pub fn new() -> Self {
        Self {
            cache: custom_cache::get_cache(),
            logger: priority_engine::PriorityLogger::new(),
            map: HashMap::new(),
            read_from_input: false,
        }
    }

    pub fn load_priorities(&mut self, file_path: String) -> Result<String> {
        if file_path.is_empty() {
            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "no priority input file supplied"));
        }

        let path = Path::new(&file_path);
        let file = File::open(&path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }

            if let Ok(tup) = serde_json::from_str::<(CacheKey, u8, bool)>(&line) {
                info!("read in object: {:?}", tup);
                let (ck, u, i) = tup;
                self.map.insert(ck, (u, i));
            } else {
                continue
            }
        }

        self.read_from_input = true;

        Ok("".to_string())
    }

    pub fn map_priority(
        &mut self, priority: &quiche::h3::Priority,
        cache_key: custom_cache::CacheKey,
    ) -> Result<quiche::h3::Priority> {
        // check if we supplied an input... if we did, read from it
        if self.read_from_input {
            if let Some(tup) = self.map.get(&cache_key) {
                info!("found prio map match: {:?}", tup);
                let (u, i) = tup;
                return Ok(quiche::h3::Priority::new(u.clone(), i.clone()));
            } else {
                let err = std::io::Error::new(
                    ErrorKind::NotFound,
                    "no match in map...",
                );
                return Err(err);
            }
        }

        info!("inserting {:?}, {:?}", cache_key, priority);
        self.map.insert(cache_key.clone(), priority.get_fields());

        Ok(quiche::h3::Priority::default())
    }

    pub fn write_files(&mut self) -> Result<()> {
        let path = Path::new("priorities-output.jsonl");
        let mut file = File::create(&path)?;
        for (cache_key, (u, i)) in &self.map {
            #[derive(Serialize)]
            struct Entry<'a> {
                cache_key: &'a custom_cache::CacheKey,
                u: u8,
                i: bool,
            }
            let entry = Entry { cache_key, u: *u, i: *i };
            let json_line = serde_json::to_string(&entry)?;
            writeln!(file, "{}", json_line)?;
        }
        self.logger.write_to_json()
    }

}

#[derive(Serialize, Deserialize, Debug)]
pub struct PriorityLogMsg {
    pub stream_id: u64,
    pub urgency: u8,
    pub incremental: bool,
    pub headers: Vec<(String, String)>,
    pub content_type: String,
    pub cache_key: custom_cache::CacheKey,
}

// gets the hashmap of CacheKey -> Priority
// pub fn read_priority_map(
//     priorities_input: String,
// ) -> HashMap<custom_cache::CacheKey, (u8, bool)> {
//     // open the file for reading
//     let file =
//         File::open(priorities_input).expect("ERROR READING PRIORITIES INPUT");
//     let reader = BufReader::new(file);
//
//     let map: HashMap<custom_cache::CacheKey, (u8, bool)> =
//         serde_json::from_reader(reader).expect("ERROR READING PRIORITIES INPUT");
//
//     info!("(smc) map: {:?}", map);
//     map
// }

// This is a sort of class:
//  this struct holds a vector of log messages. it exposes a few public functions:
//   new: creates a new object (instantiates an empty vector)
//   add_msg: creates a new PriorityLogMsg object and pushes it to the vector
//   write_to_json: writes the objects to a JSON file
pub struct PriorityLogger {
    msgs: Vec<PriorityLogMsg>
}
impl PriorityLogger {
    pub fn new() -> Self {
        Self {
            msgs: vec![]
        }
    }

    // Adds a message to the msgs vector
    // all it does it create a PriorityLogMsg object from the args, and pushes it
    pub fn add_msg(
        &mut self, stream_id: u64, priority: &quiche::h3::Priority,
        headers: Vec<(String, String)>, content_type: String,
        cache_key: custom_cache::CacheKey,
    ) {
        let (urg, inc) = priority.get_fields();
        let log_msg = PriorityLogMsg {
            stream_id,
            urgency: urg,
            incremental: inc,
            headers,
            content_type,
            cache_key,
        };
        info!("adding log message: {:?}", log_msg);
        self.msgs.push(log_msg);
    }

    // writes the vector to a JSON file
    pub fn write_to_json(&self) -> Result<()> {
        let path = Path::new("priorities-log.jsonl");
        let mut file = File::create(&path)?;
        for log_msg in &self.msgs {
            // (ck, (u, i))
            let json_line = serde_json::to_string(&log_msg)?;
            writeln!(file, "{}", json_line)?;
        }
        // let file = File::create(&self.json_location)?;
        // to_writer(file, &self.msgs)?;
        Ok(())
    }
}
