use std::io;

use clap::Parser;
use vas::config::Config;

fn main() -> io::Result<()> {
    let config = Config::parse();
    vas::run(config)
}
