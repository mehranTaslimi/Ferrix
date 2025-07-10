use std::path::Path;

use fs2::available_space;

impl super::File {
    pub fn check_disk_space(file_path: &str, total_bytes: u64) -> Result<bool, String> {
        let path = Path::new(file_path)
            .parent()
            .ok_or("failed to get parent of the file path".to_string())?;
        let space = available_space(path).map_err(|e| e.to_string())?;

        return Ok(space > total_bytes);
    }
}
