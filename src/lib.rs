use std::fs;
use std::error::Error;
use regex::Regex;

use std::collections::HashMap;

pub fn run(filename: String) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;

    for (k, v) in search(&contents) {
        println!("{}: {}", k, v);
    }

    Ok(())
}

pub fn search(contents: &str) -> HashMap<&str, &str> {
    let re = Regex::new(r"^#(\+|-)(?P<key>[-[:lower:]]+):\s*(?P<value>.+)$").unwrap();

    let mut metadata = HashMap::new();

    for line in contents.lines() {
	if let Some(caps) = re.captures(line) {
	    let k = caps.name("key").unwrap().as_str();
	    let v = caps.name("value").unwrap().as_str();
	    metadata.insert(k, v);
	}
    }
    metadata
}

