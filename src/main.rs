mod args;
mod config;

use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

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
            let project_path = new_args.path;
            let project_name = project_path.file_name().unwrap().to_str().unwrap();
            println!("Creating new project: {0}", project_path.display());

            let template = TemplateGenerator::new(new_args.template);

            let src_path: PathBuf = [project_path.clone(), PathBuf::from("src")]
                .iter()
                .collect();
            let wrap_path: PathBuf = [src_path.clone(), PathBuf::from("wrap.coen")]
                .iter()
                .collect();
            let main_path: PathBuf = [src_path.clone(), PathBuf::from("main.coen")]
                .iter()
                .collect();
            let info_path: PathBuf = [src_path.clone(), PathBuf::from("info.coen")]
                .iter()
                .collect();

            println!("{}", project_path.display());

            if !project_path.exists() {
                fs::create_dir(&project_path).unwrap();

                if !src_path.exists() {
                    fs::create_dir(&src_path).unwrap();
                }
            }

            fs::write(wrap_path, template.get_wrap().get_contents()).unwrap();
            fs::write(info_path, template.get_info().get_contents(project_name)).unwrap();
            fs::write(main_path, template.get_main().get_contents()).unwrap();
        }
        Build(file) => {
            println!("Building current project: {file:?}");
        }
    }
}
