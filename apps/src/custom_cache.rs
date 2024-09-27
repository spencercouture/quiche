#![allow(unused)]
use crate::custom_cache;
use crate::http_record;
use serde::{Deserialize, Serialize};
use serde_json::to_writer;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
#[allow(unused_imports)]
use std::io;
use std::io::prelude::*;
use std::io::Result;

#[derive(Clone, Hash, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct CacheKey {
    pub method: String,
    pub keyuri: String,
    pub host: String,
    pub https: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CacheEntry {
    pub request_uri: String,
    pub response_status: i64,
    pub response_headers: Vec<(String, String)>,
    pub response_body: Vec<u8>,
    pub request_headers: HashMap<String, (String, String)>,
}

// unchunks the HTTP response
fn unchunk(body: &[u8]) -> Vec<u8> {
    let mut offset = 0;
    let length = body.len();
    let mut out: Vec<u8> = Vec::new();
    // let mut out = String::new();
    let mut tmp: Vec<char> = Vec::new();
    let mut overall_length = 0;

    loop {
        if offset + 1 > length {
            break;
        }

        let e = body[offset] as char;

        offset += 1;
        tmp.push(e);

        if tmp.len() > 8 {
            return body.to_vec();
            // return body.to_vec().iter().map(|&c| c as char).collect();
        }

        // &tmp[(tmp.len() - 2)..] == "\r\n"
        if tmp.len() >= 2
            && tmp[tmp.len() - 2] == '\r'
            && tmp[tmp.len() - 1] == '\n'
        {
            let chunk_size: String =
                tmp[..tmp.len() - 2].to_vec().into_iter().collect();
            println!("interpreting chunk size: {}", chunk_size);
            match i64::from_str_radix(&chunk_size, 16) {
                Ok(cl) => {
                    let chunk_length = cl as usize;
                    tmp.clear();
                    overall_length += chunk_length;

                    println!("found chunk of length: {}", chunk_length);

                    if overall_length > 10 * 1024 * 1024 {
                        println!("{}", overall_length);
                        println!("chunks use more than 10MB?!");
                        return body.to_vec();
                        // return body.to_vec().iter().map(|&c| c as char).collect();
                    }

                    if offset + chunk_length > length {
                        println!("unexpected end of chunk");
                        return body.to_vec();
                        // return body.to_vec().iter().map(|&c| c as char).collect();
                    }

                    // let chunk = String::from_utf8(body[offset..offset + chunk_length].to_vec()).unwrap();
                    // let chunk: String = body[offset..(offset + chunk_length)].to_vec().iter().map(|&c| c as char).collect();
                    let chunk = body[offset..(offset + chunk_length)].to_vec();
                    offset += chunk_length;
                    out.extend_from_slice(&chunk);

                    if offset + 2 > length {
                        println!("missing \\r\\n after chunk");
                        return body.to_vec();
                        // return body.to_vec().iter().map(|&c| c as char).collect();
                    }

                    let tail =
                        String::from_utf8(body[offset..offset + 2].to_vec())
                            .unwrap();
                    offset += 2;

                    if tail != "\r\n" {
                        println!("chunk does not end with \\r\\n");
                        return body.to_vec();
                        // return body.to_vec().iter().map(|&c| c as char).collect();
                    }
                },
                Err(err) => {
                    println!("HIT AN ERROR!");
                    // println!("{}", String::from_utf8_lossy(body));
                    println!("{}", err);
                    return body.to_vec();
                    // return body.to_vec().iter().map(|&c| c as char).collect();
                },
            }
        }
    }

    out
}

pub fn get_cache() -> HashMap<CacheKey, Vec<CacheEntry>> {
    // create cache...
    let mut cache: HashMap<CacheKey, Vec<CacheEntry>> = HashMap::new();
    let mut is_chunked = false;

    let pb_files = fs::read_dir("/protobuf_files").unwrap();
    'foreach_file: for pb_file in pb_files {
        let path = pb_file.unwrap().path();

        if path.extension() != Some(std::ffi::OsStr::new("save")) {
            continue;
        }

        let mut f = File::open(path.to_owned()).unwrap();
        let mut buffer = Vec::new();
        // read the whole file
        f.read_to_end(&mut buffer).unwrap();

        // println!("reading in {}", path.display());

        // read in RequestResponse
        let rr: http_record::RequestResponse =
            protobuf::Message::parse_from_bytes(&buffer).unwrap();

        // set https var
        // let https = (rr.scheme == http_record::request_response::Scheme::HTTPS);
        let https = matches!(
            rr.scheme.unwrap().unwrap(),
            http_record::request_response::Scheme::HTTPS
        );

        // build map of requests String: (String, String)
        let mut request_headers: HashMap<String, (String, String)> =
            HashMap::new();
        for rheader in &rr.request.header {
            let key = String::from_utf8(rheader.key.as_deref().unwrap().to_vec())
                .unwrap();
            let key_stripped = key.trim().to_lowercase();
            let val =
                String::from_utf8(rheader.value.as_deref().unwrap().to_vec())
                    .unwrap();
            request_headers.insert(key_stripped, (key, val));
        }

        // build vec of responses (String, String)
        let mut response_headers = Vec::new();
        for rheader in &rr.response.header {
            let key = String::from_utf8(rheader.key.as_deref().unwrap().to_vec())
                .unwrap();
            let key_stripped = key.trim().to_lowercase();
            let val =
                String::from_utf8(rheader.value.as_deref().unwrap().to_vec())
                    .unwrap();

            println!("parsing response header: '{}':'{}'", key, val);
            if val.trim().to_lowercase() == "gzip" {
                // println!("found gzip response, skipping cache insertion");
                // is_chunked = false;
                // continue 'foreach_file;
            }

            // remove transfer-encoding to disable chunking
            if ![
                "expires",
                "date",
                "last-modified",
                "link",
                "alt-svc",
                "connection",
                "transfer-encoding",
            ]
            .iter()
            .any(|&s| s == key_stripped)
            {
                response_headers.push((key, val));
            } else if key_stripped == "transfer-encoding"
                && val.contains("chunked")
            {
                is_chunked = true;
                println!("found chunked header");
            }
        }

        // let smc = rr.response.body.as_deref().unwrap().to_vec();
        // if smc.len() > 0 {
        //     println!("smc: {}", smc[1]);
        // }
        //    let utf8_bytes = utf8_u32.to_be_bytes();
        // let s = std::str::from_utf8(&utf8_bytes);

        // read in response body, response.FirstLine and request.FirstLine
        // let response_body: String = String::from_utf8(rr.response.body.as_deref().unwrap().to_vec()).unwrap();
        // let response_body: String = rr.response.body.as_deref().unwrap().to_vec().iter().map(|&c| c as char).collect();
        // let response_body: String =     if !is_chunked { rr.response.body.as_deref().unwrap().to_vec().iter().map(|&c| c as char).collect() }
        // else { unchunk(rr.response.body.as_deref().unwrap()) };
        // let response_body: Vec<u8> = if !is_chunked { rr.response.body.as_deref().unwrap().to_vec() } else { unchunk(rr.response.body.as_deref().unwrap()) };

        let response_body: Vec<u8> = if !is_chunked {
            rr.response.body.as_deref().unwrap().to_vec()
        } else {
            unchunk(rr.response.body.as_deref().unwrap())
        };

        let response_line = String::from_utf8(
            rr.response.first_line.as_deref().unwrap().to_vec(),
        )
        .unwrap();
        let request_line =
            String::from_utf8(rr.request.first_line.as_deref().unwrap().to_vec())
                .unwrap();

        // split up request line, make sure it has 3 parts?
        // get method and uri
        let reqls: Vec<&str> = request_line.split(" ").collect();
        if reqls.len() != 3 {
            panic!("request line has more than 3 entries");
        }
        let req_method = reqls[0];
        let req_uri = reqls[1];

        // get request_keyuri and host
        let req_keyuri = req_uri.split("?").collect::<Vec<&str>>()[0];
        let (_, req_host) = request_headers.get("host").unwrap();

        // split up response line, make sure it has at least 2 parts
        // get method and uri
        let resps: Vec<&str> = response_line.split(" ").collect();
        if reqls.len() < 2 {
            panic!("response line has less than 2 entries");
        }
        // get response code
        let resps_code: i64 = resps[1].parse().unwrap();

        println!(
            "Cache key: [{}] [{}] [{}] [{}]",
            req_method, req_keyuri, req_host, https
        );

        // build cache key and cache entry
        let cache_key = CacheKey {
            method: req_method.to_owned(),
            keyuri: req_keyuri.to_owned(),
            host: req_host.to_owned(),
            https,
        };
        let cache_entry = CacheEntry {
            request_uri: req_uri.to_owned(),
            response_status: resps_code,
            response_headers,
            response_body,
            request_headers: request_headers.to_owned(),
        };

        // DEBUG
        println!("inserting into cache: ");
        println!("\t{:?}", &cache_key);
        //println!("\t\t->{:?}", &cache_entry);

        // insert into cache
        let cache_entries =
            cache.entry(cache_key).or_insert(Vec::<CacheEntry>::new());

        // if we have a cache_entries length of > 1, print it out for debugging
        if cache_entries.len() > 1 {
            println!("adding entry number {}", cache_entries.len());
        }

        cache_entries.push(cache_entry);
    }

    cache
}

// function to serialize the cache so we can modify the priorities
// pub fn serialize_cache(
//     json_location: String,
//     cache: &HashMap<custom_cache::CacheKey, Vec<custom_cache::CacheEntry>>,
// ) -> Result<()> {
//     let file: File = File::create(json_location)?;
//     to_writer(file, &cache)?;
//     Ok(())
// }

// fn main() -> io::Result<()> {
//     let mut cache: HashMap<CacheKey, Vec<CacheEntry>> = get_cache();

//     // // test cache access
//     let cache_key = CacheKey{
//         method: "GET".to_string(),
//         keyuri: "/".to_string(),
//         host: "www.unh.edu".to_string(),
//         https: false
//     };

//     let entries = cache.get(&cache_key).unwrap();
//     println!("{} entries found.", entries.len());

//     let res = &entries[0];
//     println!("res: {:?}", res);

//     // struct CacheKey {
//     //     method: String,
//     //     keyuri: String,
//     //     host: String,
//     //     https: bool,
//     // }

//     // // build cache key and cache entry
//     // let cache_key2 = CacheKey{
//     //     method: req_method.to_owned(),
//     //     keyuri: req_keyuri.to_owned(),
//     //     host: req_host.to_owned(),
//     //     https: https
//     // };
//     // match cache.get(&cache_key2) {
//     //     Some(entries) => println!("found entries: {}", entries.len()),
//     //     None => println!("nothing")
//     // }

//     Ok(())
// }
