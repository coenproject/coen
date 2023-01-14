use std::path::PathBuf;

use super::Coen;

impl Coen {
    pub(crate) fn get_info_file_path(&self) -> PathBuf {
        let mut info_file_path = PathBuf::new();
        info_file_path.push(self.project_path.clone());
        info_file_path.push("info.txt");

        info_file_path
    }

    pub(crate) fn get_coen_file_path(&self) -> PathBuf {
        let mut coen_file_path = PathBuf::new();
        coen_file_path.push(self.project_path.clone());
        coen_file_path.push("main.coen");

        coen_file_path
    }
}
