use std::fmt;
use std::fmt::{Display, Formatter};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Algorithm {
    Md5,
    Sha1,
    Sha256,
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
    "fakehash".to_string()
}

fn md5() -> String {
    "MD5HashGoesHere".to_string()
}
fn sha1() -> String {
    "SHA1HashGoesHere".to_string()
}
fn sha256() -> String {
    "SHA256HashGoesHere".to_string()
}
