mod args;
mod coen_builder;

use std::{error::Error, fs};

use crate::coen_builder::CoenBuilder;

use args::CoenArgs;

use clap::Parser;

fn main() -> Result<(), Box<dyn Error>> {
    let args = CoenArgs::parse();

    let root_path = fs::canonicalize(args.root)?;

    let builder = CoenBuilder::new(root_path)?;

    println!("{builder:?}");

    Ok(())
}
