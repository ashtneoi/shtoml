extern crate toml;

use std::env;
use std::fs;
use std::io;
use std::process::exit;
use std::str::FromStr;

use toml::Value;

const SENTINEL_KEY: &str = "shtoml-0ed5f89f718b";

pub trait SliceAsStrs {
    fn as_strs(&self) -> Vec<&str>;
}

impl SliceAsStrs for [String] {
    fn as_strs(&self) -> Vec<&str> {
        self.iter().map(|s| &s[..]).collect()
    }
}

fn exit_with_usage() -> ! {
    eprintln!("Usage: shtoml FILE KEY");
    exit(2);
}

fn parse_key(key_str: &str) -> Option<Vec<String>> {
    let fake_table_string = format!(
        "[{}]\n{} = 0", key_str, SENTINEL_KEY);
    let fake_table = match fake_table_string.parse::<Value>().ok()? {
        Value::Table(x) => x,
        _ => panic!(),
    };

    let mut keys: Vec<String> = Vec::new();
    let mut t = &fake_table;
    loop {
        let (key, val) = t.iter().next().unwrap();
        if key == SENTINEL_KEY {
            break;
        }
        keys.push(key.to_string());
        t = match val {
            Value::Table(x) => &x,
            _ => panic!(),
        };
    }

    Some(keys)
}

fn main() {
    let arg_strings: Vec<_> = env::args().collect();
    let args = arg_strings.as_strs();
    if args.len() != 3 {
        exit_with_usage();
    }

    let filename = args[1];
    let key = parse_key(args[2]).unwrap_or_else(|| {
        eprintln!("error: invalid key");
        exit(2);
    });

    let file = match fs::read(filename) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("error: can't read file ({})", e);
            exit(2);
        },
    };
    let parsed: Value = match toml::de::from_slice(&file) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("error: can't parse file ({})", e);
            exit(1);
        },
    };

    println!("{:?}", parsed);
}
