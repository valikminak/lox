pub type Source = ();

pub type Error = ();

pub fn read_source(filename: &str) -> Result<Source, Error> {
    println!("Reading source");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_alive() {
        assert_eq!(true, true);
    }
}
