mod args;
mod coen;

use crate::coen::Coen;

use args::{
    CoenArgs,
    OperationType::{Build, New},
};

use clap::Parser;

fn main() {
    let args = CoenArgs::parse();

    match args.operation {
        New(new_args) => Coen::create(new_args),
        Build(file) => println!("Building current project: {file:?}"),
    }
}
