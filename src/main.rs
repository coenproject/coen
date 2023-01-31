mod args;

use args::{
    CoenArgs,
    OperationType::{Build, New},
};
use clap::Parser;

fn main() {
    let args = CoenArgs::parse();

    match args.operation {
        New(file) => {
            println!("Creating new project: {file:?}");
        }
        Build(file) => {
            println!("Building current project: {file:?}");
        }
    }
}
