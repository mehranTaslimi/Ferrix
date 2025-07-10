use std::fs;

impl super::File {
    pub fn remove_file(file_path: &str) -> Result<(), std::io::Error> {
        if std::path::Path::new(file_path).exists() {
            return fs::remove_file(file_path);
        }

        Ok(())
    }
}
