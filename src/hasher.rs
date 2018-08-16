pub enum algorithm {
    md5,
    sha1,
    sha256,
}

pub fn hash_file(alg: algorithm, fn: os::path) -> String{
    
}

fn md5 () -> String {
    "MD5HashGoesHere".to_str()
}
fn sha1 () -> String {
    "SHA1HashGoesHere".to_str()
}
fn sha256 () -> String {
    "SHA256HashGoesHere".to_str()
}
