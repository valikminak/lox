use crate::ast::AST;

pub struct Output {}

#[derive(Debug)]
pub struct Error {}
pub fn evaluate(_ast: AST) -> Result<Output, Error> {
    println!("Evaluating");
    Ok(Output {})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_alive() {
        assert_eq!(true, true);
    }
}
