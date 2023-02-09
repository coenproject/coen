mod args;
mod coen_builder;

use std::{error::Error, fs};

use crate::coen_builder::CoenBuilder;

use args::CoenArgs;

use clap::Parser;

fn main() -> Result<(), Box<dyn Error>> {
    let args = CoenArgs::parse();

    let mut builder = CoenBuilder::new(&args)?;
    builder.build()?;

    let target = match args.target.clone() {
        Some(target) => target,
        None => builder.get_target().unwrap(),
    };

    let content = builder.get_content();

    fs::write(target, &content)?;

    Ok(())
}
