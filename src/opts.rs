use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Opts {
    #[clap(short, long, value_parser, default_value = "3000")]
    pub port: u16,
}
