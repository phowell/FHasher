use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub enum Algorithm {
    Md5,
    Sha1,
    Sha256,
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
