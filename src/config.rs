use std::{fs, path::PathBuf};

pub const CONFIG_PATH: &str = "$HOME/.coen/";

pub struct Wrap {
    path: PathBuf,
}

impl Wrap {
    pub fn get_contents(self) -> String {
        let contents = fs::read_to_string(self.path).unwrap();

        contents
    }
}

pub struct Info {
    path: PathBuf,
}

impl Info {
    pub fn get_contents(self, name: &str) -> String {
        let contents = fs::read_to_string(self.path).unwrap();
        let contents = contents.replace("$PROJECT_NAME", name);

        contents
    }
}

pub struct Main {
    path: PathBuf,
}

impl Main {
    pub fn get_contents(self) -> String {
        let contents = fs::read_to_string(self.path).unwrap();

        contents
    }
}

pub struct TemplateGenerator {
    template_name: String,
    wrap: Wrap,
    info: Info,
    main: Main,
}

impl TemplateGenerator {
    pub fn new(template_name: Option<String>) -> Self {
        let template_name = template_name.unwrap_or("default".to_owned());

        let template_path: PathBuf = [CONFIG_PATH.to_owned(), template_name.clone()]
            .iter()
            .collect();

        let wrap_path: PathBuf = [template_path.clone(), "wrap.coen".into()].iter().collect();
        let wrap = Wrap { path: wrap_path };

        let info_path: PathBuf = [template_path.clone(), "info.coen".into()].iter().collect();
        let info = Info { path: info_path };

        let main_path: PathBuf = [template_path.clone(), "main.coen".into()].iter().collect();
        let main = Main { path: main_path };

        Self {
            template_name,
            wrap,
            info,
            main,
        }
    }

    pub fn get_wrap() {
        //
    }
}
