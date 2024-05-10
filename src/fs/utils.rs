use std::path::PathBuf;

pub fn path_buf_to_string(path_buf: &PathBuf) -> String {
    path_buf.to_string_lossy().to_string()
}
