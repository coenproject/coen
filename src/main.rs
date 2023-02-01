mod args;
mod config;

use std::{fs, path::PathBuf};

use config::TemplateGenerator;

use args::{
    CoenArgs,
    OperationType::{Build, New},
};
use clap::Parser;

fn main() {
    let args = CoenArgs::parse();

    match args.operation {
        New(new_args) => {
            println!("Creating new project: {0}", new_args.path.display());

            let template = TemplateGenerator::new(new_args.template);

            let project_path = new_args.path;
            let src_path: PathBuf = [project_path.clone(), PathBuf::from("src")]
                .iter()
                .collect();

            if !project_path.exists() {
                fs::create_dir(&project_path).unwrap();

                if !src_path.exists() {
                    fs::create_dir(&src_path).unwrap();
                }
            }

            //
        }
        Build(file) => {
            println!("Building current project: {file:?}");
        }
    }
}
