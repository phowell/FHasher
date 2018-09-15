use std::fmt;
use std::fmt::{Display, Formatter};
use std::path::Path;

use md5::{Digest, Md5};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Algorithm {
    Md5,
    Sha1,
    Sha256,
}

impl Algorithm {
    pub fn from_string(alg: &str) -> Algorithm {
        match alg {
            "md5" => Algorithm::Md5,
            "sha1" => Algorithm::Sha1,
            "sha256" => Algorithm::Sha256,
            _ => panic!("That's not an algorithm"),
        }
    }
}

impl Display for Algorithm {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let printable = match *self {
            Algorithm::Md5 => "Md5",
            Algorithm::Sha1 => "Sha1",
            Algorithm::Sha256 => "Sha256",
        };

        write!(f, "{}", printable)
    }
}

pub fn hash_file(alg: Algorithm, fp: &Path) -> String {
    match alg {
        Algorithm::Md5 => md5(fp),
        Algorithm::Sha1 => sha1(),
        Algorithm::Sha256 => sha256(),
    }
}

fn md5(fp: &Path) -> String {
    let hasher = Md5::default();

    //hasher.result_str()
    "MD5GoesHere".to_string()
}
fn sha1() -> String {
    "SHA1HashGoesHere".to_string()
}
fn sha256() -> String {
    "SHA256HashGoesHere".to_string()
}
