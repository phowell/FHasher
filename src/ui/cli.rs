use clap::{App, Arg};
use hasher::Algorithm;
use logfile;
use std::path::Path;

pub fn parse() {
    let matches = App::new("FHasher Portable Hashing Tool")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Simple hashing tool")
        .arg(
            Arg::with_name("Recursive")
                .short("r")
                .help("Hash all files and folders in current folder, recursively"),
        )
        .arg(
            Arg::with_name("Files")
                .short("f")
                .long("file")
                .takes_value(true)
                .multiple(true)
                .conflicts_with("Recursive")
                .help("Name files to be hashed and logged"),
        )
        .arg(
            Arg::with_name("Folder")
                .short("F")
                .long("folder")
                .takes_value(true)
                .multiple(true)
                .conflicts_with("Recursive")
                .help("Name folders to be hashed and logged"),
        )
        .arg(
            Arg::with_name("Algorithm")
                .short("a")
                .long("alg")
                .long("algorithm")
                .takes_value(true)
                .default_value("md5")
                .multiple(true)
                .help("Name which algorithms you wish to use"),
        )
        .arg(
            Arg::with_name("Hash Log")
                .short("l")
                .long("log")
                .takes_value(true)
                .default_value("fhasher.log")
                .help("Set an alternate file name for the hash log file. Default is fhasher.log"),
        )
        .get_matches();

    let logfile = matches.value_of("Hash Log").unwrap();
    let file = Path::new(matches.value_of("Files").unwrap());
    let mut log = logfile::Log::open(logfile).unwrap();
    match log.add(file, Algorithm::Md5) {
        Ok(()) => println!("Yay"),
        Err(e) => println!("BOO: {}", e),
    }

    match log.save(logfile) {
        Ok(()) => println!("File Saved"),
        Err(e) => println!("File Not Saved: {}", e),
    }

    println!("{}:", logfile);
    println!("{:?}", log);
}
