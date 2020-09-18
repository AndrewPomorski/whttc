use structopt::StructOpt;
extern crate clap;
use clap::{Arg, App, SubCommand};

#[derive(StructOpt)]
#[derive(Debug)]
struct Cli {
    pattern: String,
}

fn main() {
    let args = Cli::from_args();
    let matches = App::new("whttc HTTP Codes detailer")
                        .version("0.1.0")
                        .author("Andrew Pomorski @andrewpomorski")
                        .arg(Arg::with_name("code")
                            .short("c")
                            .help("pass http code")
                            .takes_value(true))
                        .get_matches();
    
    println!("ARGUMENTS {:?}", args);
}
