use crate::{args::NewArgs, coen::config::TemplateGenerator};
use std::{fs, path::PathBuf};

use super::Coen;

impl Coen {
    pub fn create(new_args: NewArgs) {
        let project_path = new_args.path;
        let project_name = project_path.file_name().unwrap().to_str().unwrap();

        println!("Creating new project: {}", project_path.display());

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

        println!("{}", wrap_path.display());

        if !project_path.exists() {
            fs::create_dir(&project_path).unwrap();

            if !src_path.exists() {
                fs::create_dir(&src_path).unwrap();
            }
        }

        if !new_args.reference {
            fs::write(&wrap_path, template.get_wrap().get_contents()).unwrap();
        }

        let target_wrap_path = if new_args.reference {
            wrap_path.to_str().unwrap()
        } else {
            "wrap.coen"
        };

        fs::write(
            &info_path,
            template
                .get_info()
                .get_contents(project_name, target_wrap_path),
        )
        .unwrap();

        fs::write(&main_path, template.get_main().get_contents()).unwrap();
    }
}
