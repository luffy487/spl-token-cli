use serde_json;
use std::fs;
use std::io;

pub fn read_keys_from_file() -> Result<Vec<u8>, io::Error> {
    let file = fs::File::open("config.json")?;
    let keys: Vec<u8> = serde_json::from_reader(file).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("JSON parsing error: {}", e),
        )
    })?;
    Ok(keys)
}
