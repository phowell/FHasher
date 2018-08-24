use hasher::{hash_file, Algorithm};
use serde_json;
use std::{
    collections::HashMap,
    error::Error,
    fs::{write, File},
    io::BufReader,
    path::Path,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Log {
    #[serde(flatten)]
    pub meta: Metadata,

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

    pub fn save(&self, path: &str) -> Result<(), Box<Error>> {
        let pickle = serde_json::to_string_pretty(self)?;
        write(path, pickle)?;
        Ok(())
    }

    pub fn add(&mut self, p: &Path, alg: Algorithm) -> Result<(), Box<Error>> {
        let hash = hash_file(alg, p);
        let mut map = HashMap::new();
        map.insert(alg.to_string(), hash);
        let mut entry = Entry {
            path: p.display().to_string(),
            hashes: map,
        };
        self.hashes.push(entry);
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
