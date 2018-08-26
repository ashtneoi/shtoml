extern crate toml;

use std::env;
use std::fs;
use std::process::exit;

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
        "[{}]\n{} = 0\n", key_str, SENTINEL_KEY);
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
            _ => return None,
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

    let mut v: &Value = &parsed;
    for key_part in key {
        let t = match v {
            &Value::Table(ref x) => x,
            _ => {
                eprintln!("error: value is not a table"); // TODO: better error
                exit(1);
            },
        };
        v = t.get(&key_part).unwrap_or_else(|| {
            eprintln!("error: can't find key"); // TODO: better error
            exit(1);
        });
    }

    match v {
        Value::String(x) => println!("{}", x),
        Value::Integer(x) => println!("{}", x),
        Value::Float(x) => println!("{}", x),
        Value::Boolean(x) => println!("{}", x),
        Value::Datetime(x) => println!("{}", x),
        _ => {
            eprintln!("error: value is not a scalar");
            exit(1);
        },
    };
}
