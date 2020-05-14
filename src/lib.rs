use regex::Regex;
use std::error::Error;
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub struct EntityAttributeValue {
    entity: PathBuf,
    attribute: String,
    value: String,
}

pub fn run(path: String) -> Result<(), Box<dyn Error>> {
    for eav in index_path(&path)? {
        let EntityAttributeValue {
            entity,
            attribute,
            value,
        } = eav;
        println!("{}, {}: {}", entity.to_str().unwrap(), attribute, value);
    }

    Ok(())
}

pub fn analyze_table(entity: PathBuf, contents: &str) -> Vec<EntityAttributeValue> {
    let re = Regex::new(r"^#(\+|-)(?P<key>[-[:lower:]]+):\s*(?P<value>.+)$").unwrap();

    let mut metadata = Vec::new();

    for line in contents.lines() {
        if let Some(caps) = re.captures(line) {
            let entity = entity.clone();
            let attribute = caps.name("key").unwrap().as_str().to_string();
            let value = caps.name("value").unwrap().as_str().to_string();
            metadata.push(EntityAttributeValue {
                entity,
                attribute,
                value,
            })
        }
    }
    metadata
}

pub fn index_path(path: &str) -> io::Result<Vec<EntityAttributeValue>> {
    let mut metadata = Vec::new();

    for entry in fs::read_dir(path)? {
        let path = entry?.path();
        if path.is_file() {
            // ignore files that aren't utf-8
            if let Ok(content) = fs::read_to_string(&path) {
                metadata.extend(analyze_table(path, &content));
            }
        }
    }
    Ok(metadata)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain() {
        let path = PathBuf::from("/foo/bar");
        let contents = "\
#+foo: bar
#+hehe:hoho
#+foo-bar: yes
#+foo_bar: no
#-contracted: full";

        let index = vec![
            EntityAttributeValue {
                entity: path.clone(),
                attribute: String::from("foo"),
                value: String::from("bar"),
            },
            EntityAttributeValue {
                entity: path.clone(),
                attribute: String::from("hehe"),
                value: String::from("hoho"),
            },
            EntityAttributeValue {
                entity: path.clone(),
                attribute: String::from("foo-bar"),
                value: String::from("yes"),
            },
            EntityAttributeValue {
                entity: path.clone(),
                attribute: String::from("contracted"),
                value: String::from("full"),
            },
        ];
        assert_eq!(index, analyze_table(path, contents));
    }

    #[test]
    fn faulty() {
        let path = PathBuf::from("/foo/bar");
        let contents = "\
#+: no
#+foo-bar:";

        let index: Vec<EntityAttributeValue> = vec![];
        assert_eq!(index, analyze_table(path, contents));
    }
}
