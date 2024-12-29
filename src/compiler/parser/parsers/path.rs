use crate::compiler::{
    errors::CompileResult,
    lexer::{Token, Tokens},
    path::Path,
};

impl Tokens {
    pub fn parse_path(&mut self, root: &String) -> CompileResult<Path> {
        let mut path = Path::from(root);
        loop {
            if !self
                .peek_expect_tokens(vec![Token::DoubleColon], true)
                .is_some()
            {
                break;
            }
            
            let identifier = self.parse_identifier()?;
            path.push(identifier)
        }

        return Ok(path);
    }
}
