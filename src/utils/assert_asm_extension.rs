use std::path::PathBuf;

pub fn assert_asm_extension(path: String) -> PathBuf {
    let path = PathBuf::from(path);
    if path.extension().and_then(|ext| ext.to_str()) != Some("s") {
        path.with_extension("s")
    } else {
        path
    }
}
