pub struct Source {
    pub contents: String,
}

#[derive(Debug)]
pub struct Error {
    msg: String,
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error { msg: format!("{}", e) }
    }
}

pub fn read_source(filename: &str) -> Result<Source, Error> {
    println!("Reading source");
    let contents = std::fs::read_to_string(filename)?;
    Ok(Source { contents })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_alive() {
        assert_eq!(true, true);
    }
}
