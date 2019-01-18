use std::fmt;
use std::fmt::{Display, Formatter};
use std::io;
use std::io::Read;
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
            "Md5" => Algorithm::Md5,
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
    let res = match alg {
        Algorithm::Md5 => md5(fp),
        Algorithm::Sha1 => sha1(fp),
        Algorithm::Sha256 => sha256(fp),
    };

    match res {
        Ok(n) => n,
        _ => "Oops".to_string(),
    }
}

fn md5(fp: &Path) -> io::Result<String> {
    let mut hasher = Md5::default();
    let mut reader = std::fs::File::open(fp)?;

    let mut buffer = [0u8; 1024];
    loop {
        let bytes_read = reader.read(&mut buffer)?;
        hasher.input(&buffer[..bytes_read]);
        if bytes_read == 0 {
            break;
        }
    }

    let res = format!("{:x}", hasher.result());

    Ok(res)
}
fn sha1(_fp: &Path) -> io::Result<String> {
    Ok("SHA1HashGoesHere".to_string())
}
fn sha256(_fp: &Path) -> io::Result<String> {
    Ok("SHA256HashGoesHere".to_string())
}
