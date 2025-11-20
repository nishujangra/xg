use serde::Serialize;
use std::fs::File;
use std::io::Write;

pub fn write_json_file<T: Serialize>(path: &str, data: &T) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(data)?;

    let mut file = File::create(path)?;
    file.write_all(json.as_bytes())?;
 
    Ok(())
}