extern crate toml;

use std::env;
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

fn main() {
    let arg_strings: Vec<_> = env::args().collect();
    let args = arg_strings.as_strs();
    if args.len() != 3 {
        exit_with_usage();
    }

    let filename = args[1];
    let key_str = format!(
        "[{}]\n{} = 0", args[2], SENTINEL_KEY);

    let fake_table = match key_str.parse::<Value>().unwrap() {
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

    println!("{:?}", keys);
}
