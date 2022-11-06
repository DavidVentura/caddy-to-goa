use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Deserialize, Debug)]
struct Headers {
    #[serde(rename = "User-Agent")]
    user_agent: Option<Vec<String>>,
    #[serde(rename = "Referer")]
    referer: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
struct Request {
    headers: Headers,
    // remote_addr: String,
}
#[derive(Deserialize, Debug)]
struct Entry {
    request: Request,
    common_log: String,
    duration: f32,
    ts: f32,
}

#[derive(Serialize, Debug)]
struct GoaRecord<'a> {
    common_log: &'a str,
    referer: &'a str,
    user_agent: &'a str,
    duration: f32,
}

fn unwrap_first(data: &'_ Option<Vec<String>>) -> Option<&'_ str> {
    if data.is_none() {
        return None;
    }
    if let Some(d) = data {
        let first = d.get(0);
        if first.is_none() {
            return None;
        }
        return Some(first.unwrap());
    }
    return None;
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file = File::open(args[1].clone())?;
    let reader = BufReader::new(file);
    let default: String = "".to_owned();
    let start = (Utc::now() - Duration::days(60)).timestamp() as f32;

    let mut wtr = csv::Writer::from_writer(std::io::stdout());
    for line in reader.lines() {
        let l: String = line.unwrap();
        let parsed = serde_json::from_str::<Entry>(l.as_str());
        if let Ok(v) = parsed {
            if v.ts < start {
                continue;
            }
            let r = GoaRecord {
                common_log: &v.common_log,
                referer: unwrap_first(&v.request.headers.referer).unwrap_or(&default),
                user_agent: unwrap_first(&v.request.headers.user_agent).unwrap_or(&default),
                duration: v.duration,
            };
            wtr.serialize(r)?;
        } else {
            println!("{}", l);
            println!("{:?}", parsed);
        }
    }

    Ok(())
}
