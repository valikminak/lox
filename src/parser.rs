use crate::tokenize::Tokens;

pub struct AST {}

#[derive(Debug)]
pub struct Error {} 

// pub type Error = ();  // roughly playing the same role as Python’s None

pub fn parse(_tokens: Tokens) -> Result<AST, Error> {
    println!("Parsing");
    Ok(AST {})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_alive(){
        assert_eq!(true, true);
    }
}