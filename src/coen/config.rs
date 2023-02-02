use std::{fs, path::PathBuf};

pub(crate) const CONFIG_PATH: &str = "/home/madhavan/.coen";

#[derive(Debug, Clone)]
pub(crate) struct Wrap {
    path: PathBuf,
}

impl Wrap {
    pub(crate) fn get_contents(self) -> String {
        let contents = fs::read_to_string(self.path).unwrap();

        contents
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Info {
    path: PathBuf,
}

impl Info {
    pub(crate) fn get_contents(self, name: &str, wrap: &str) -> String {
        let contents = fs::read_to_string(self.path).unwrap();
        let contents = contents.replace("$PROJECT_NAME", name);
        let contents = contents.replace("$WRAP_PATH", wrap);

        contents
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Main {
    path: PathBuf,
}

impl Main {
    pub(crate) fn get_contents(self) -> String {
        let contents = fs::read_to_string(self.path).unwrap();

        contents
    }
}

#[derive(Debug, Clone)]
pub(crate) struct TemplateGenerator {
    template_name: String,
    wrap: Wrap,
    info: Info,
    main: Main,
}

impl TemplateGenerator {
    pub(crate) fn new(template_name: Option<String>) -> Self {
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

    pub(crate) fn get_wrap(&self) -> Wrap {
        self.wrap.clone()
    }

    pub(crate) fn get_info(&self) -> Info {
        self.info.clone()
    }

    pub(crate) fn get_main(&self) -> Main {
        self.main.clone()
    }
}
