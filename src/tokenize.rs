use crate::reader::Source;
pub type Tokens = ();
pub type Error = ();

pub fn tokenize(source: Source) -> Result<Tokens, Error> {
    println!("Tokenizing");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_alive(){
        assert_eq!(true, true);
    }
}