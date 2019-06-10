use std::error::Error;
use std::io::BufRead;

pub type Result<T> = std::result::Result<Option<T>, Box<dyn Error>>;

pub fn read_to_string_until<R: BufRead>(bytes: &mut R, sep: u8) -> Result<String> {
    let mut vec = Vec::new();

    if bytes.read_until(sep, &mut vec)? == 0 {
        return Ok(None);
    }

    let mut string = String::from_utf8(vec)?;
    string.truncate(string.len() - 1);

    Ok(Some(string.trim().to_string()))
}
