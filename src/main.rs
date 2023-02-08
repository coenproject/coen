mod args;
mod coen_builder;

use std::{error::Error, fs};

use crate::coen_builder::CoenBuilder;

use args::CoenArgs;

use clap::Parser;

fn main() -> Result<(), Box<dyn Error>> {
    let args = CoenArgs::parse();

    let tgt = args.target.clone().unwrap();

    let mut builder = CoenBuilder::new(args)?;
    builder.build()?;

    let content = builder.get_content();

    fs::write(tgt, &content)?;

    Ok(())
}
