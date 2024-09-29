use crate::custom_cache;
use crate::custom_cache::CacheKey;
use crate::priority_engine;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Result};
use std::io::{ErrorKind, Write};
use std::path::Path;

pub struct PriorityContext {
    pub cache: HashMap<custom_cache::CacheKey, Vec<custom_cache::CacheEntry>>,

    pub map: HashMap<custom_cache::CacheKey, (u8, bool)>,

    pub logger: priority_engine::PriorityLogger,

    pub output_loc: String,
    pub read_from_input: bool,
}
impl PriorityContext {
    pub fn new(dir_path: String) -> Self {
        Self {
            cache: custom_cache::get_cache(),
            logger: priority_engine::PriorityLogger::new(dir_path),
            map: HashMap::new(),
            output_loc: "".to_string(),
            read_from_input: false,
        }
    }

    pub fn load_priorities(&mut self, file_path: String) -> Result<String> {
        if file_path.is_empty() {
            return Ok("".to_string());
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
                continue;
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

        self.map.insert(cache_key.clone(), priority.get_fields());
        info!("inserting {:?}, {:?}", cache_key, priority);

        // Open the file in append mode, create if it doesn't exist
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(self.output_loc.clone())?;

        let (u, i) = priority.get_fields();
        let logline = serde_json::to_string(&(cache_key, u, i)).unwrap();

        // Write a line to the file
        writeln!(file, "{}", logline)?;

        Ok(quiche::h3::Priority::default())
    }

    // pub fn serialize_map(&mut self) -> Result<()> {
    //     let file = File::create(self.output_loc.clone())?;
    //
    //     let map_list: Vec<CacheKey, (u8, bool)> =
    //         self.map.into_iter().collect().to_writer(file, &self.map)?;
    //
    //     Ok(())
    // }
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
pub fn read_priority_map(
    priorities_input: String,
) -> HashMap<custom_cache::CacheKey, (u8, bool)> {
    // open the file for reading
    let file =
        File::open(priorities_input).expect("ERROR READING PRIORITIES INPUT");
    let reader = BufReader::new(file);

    let map: HashMap<custom_cache::CacheKey, (u8, bool)> =
        serde_json::from_reader(reader).expect("ERROR READING PRIORITIES INPUT");

    info!("(smc) map: {:?}", map);
    map
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
        Ok(())
        //let file = File::create(&self.json_location)?;
        //to_writer(file, &self.msgs)?;
        //Ok(())
    }
}
