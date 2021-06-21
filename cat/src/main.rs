
use std::io;
use std::io::Read;
use std::fs::File;
use std::path::Path;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(name = "file")]
    files: Vec<String>,
}

fn cat(path: &Path) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn main() {
    let opt = Opt::from_args();
    println!("Hello, world!i file:{:?}", opt.files);

    for file_path in opt.files {
        println!("{}", file_path);

        match cat(&Path::new(&file_path)) {
            Err(why) => println!("! {:?}", why.kind()),
            Ok(s) => println!("> {}", s),
        }
    }
}

