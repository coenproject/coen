mod args;
mod coen_builder;

use std::error::Error;

use crate::coen_builder::CoenBuilder;

use args::CoenArgs;

use clap::Parser;

fn main() -> Result<(), Box<dyn Error>> {
    let args = CoenArgs::parse();

    let mut builder = CoenBuilder::new(args.root, args.silent)?;
    builder.build()?;

    builder.write(args.target)?;

    Ok(())
}
