use hasher::algorithm;

#[derive(Serialize, Deserialize, Debug)]
pub struct log {
    #[serde(flatten)]
    pub meta: metadata,

    #[serde(flatten)]
    pub hashes: Vec<entry>,
}

impl log {
    fn new() -> log {
        log {
            meta: metadata {
                creation_date: "dategoeshere".to_str(),
            },
            hashes: Vec::new(),
        }
    }

    fn add(&self, path: &String, alg: algorithm, hash: &String) -> Result {
        Ok()
    }
}

pub struct metadata {
    pub creation_date: String,
}

pub struct entry {
    pub path: String,
    pub hashes: Vec,
}
