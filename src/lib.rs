use regex::Regex;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

use std::collections::HashMap;

pub fn run(path: String) -> Result<(), Box<dyn Error>> {
    for (path, metadata) in index_path(&path) {
        println!("== {}", path.to_str().unwrap());
        for (k, v) in metadata {
            println!("{}: {}", k, v);
        }
    }

    Ok(())
}

pub fn analyze_table(contents: &str) -> HashMap<String, String> {
    let re = Regex::new(r"^#(\+|-)(?P<key>[-[:lower:]]+):\s*(?P<value>.+)$").unwrap();

    let mut metadata = HashMap::new();

    for line in contents.lines() {
        if let Some(caps) = re.captures(line) {
            let k = caps.name("key").unwrap().as_str().to_string();
            let v = caps.name("value").unwrap().as_str().to_string();
            metadata.insert(k, v);
        }
    }
    metadata
}

pub fn index_path(path: &str) -> HashMap<PathBuf, HashMap<String, String>> {
    let mut metadata_index = HashMap::new();

    for entry in fs::read_dir(path).unwrap() {
        let path = entry.unwrap().path();
        if path.is_file() {
            // ignore files that aren't utf-8
            if let Ok(content) = fs::read_to_string(&path) {
                let metadata = analyze_table(&content);
                if !metadata.is_empty() {
                    metadata_index.insert(path, metadata);
                }
            }
        }
    }
    metadata_index
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashmap;

    #[test]
    fn plain() {
        let contents = "\
#+foo: bar
#+hehe:hoho
#+foo-bar: yes
#+foo_bar: no
#-contracted: full";

        let map = hashmap! {
            String::from("foo") => String::from("bar"),
            String::from("hehe") => String::from("hoho"),
            String::from("foo-bar") => String::from("yes"),
            String::from("contracted") => String::from("full"),
        };
        assert_eq!(map, analyze_table(contents));
    }

    #[test]
    fn faulty() {
        let contents = "\
#+: no
#+foo-bar:";

        let map = hashmap! {};
        assert_eq!(map, analyze_table(contents));
    }
}
