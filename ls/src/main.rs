use std::fs;
use structopt::StructOpt;


#[derive(StructOpt)]
struct Cli {
    pattern: String,

    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,

    #[structopt(name = "hoge")]
    hoge: Vec<String>,
}

fn main() {
    let args = Cli::from_args();
    println!("Hello, ls program! args.pattern:{} args.path:{}i args.hoge:{:?}", args.pattern, args.path.to_str().unwrap(), args.hoge);


    let mut files: Vec<String> = Vec::new();

    let paths = fs::read_dir("./").unwrap();
    for path in paths {
        files.push( path.unwrap().path().display().to_string() );
    }

    files.sort();
    println!("{:?}", files);
}
