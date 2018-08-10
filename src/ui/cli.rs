use clap::{App, Arg};
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
}
