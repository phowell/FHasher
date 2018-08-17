use hasher::Algorithm;
use serde_json;
use std::{collections::HashMap, error::Error, fs::File, io::BufReader, path::Path};

#[derive(Serialize, Deserialize, Debug)]
pub struct Log {
    #[serde(flatten)]
    pub meta: Metadata,

    #[serde(flatten)]
    pub hashes: Vec<Entry>,
}

impl Log {
    fn new() -> Log {
        Log {
            meta: Metadata {
                creation_date: "dategoeshere".to_string(),
            },
            hashes: Vec::new(),
        }
    }

    pub fn open(path: &str) -> Result<Log, Box<Error>> {
        if Path::new(path).exists() {
            let file = File::open(path)?;
            let buf = BufReader::new(file);
            let log: Log = serde_json::from_reader(buf).unwrap();

            Ok(log)
        } else {
            Ok(Log::new())
        }
    }

    fn add(&self, path: &Path, alg: Algorithm, hash: &String) -> Result<(), serde_json::Error> {
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub creation_date: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    pub path: String,
    pub hashes: HashMap<String, String>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

}
