use crate::compiler::{
    errors::CompileResult,
    lexer::{Token, Tokens},
    nodes::ast::LocatedPath,
    path::Path,
};

impl Tokens {
    pub fn parse_tree_path(&mut self) -> CompileResult<()> {
        todo!();
    }
    pub fn parse_path(&mut self) -> CompileResult<LocatedPath> {
        self.start_next();
        let root = self.parse_identifier()?;

        let mut path = Path::from(root.raw);
        while self
            .peek_expect_tokens(vec![Token::DoubleColon], true)
            .is_some()
        {
            let identifier = self.parse_identifier()?;
            path.push(identifier.raw)
        }

        return Ok(self.create_located(path));
    }
    pub fn parse_path_current(&mut self, root: String) -> CompileResult<LocatedPath> {
        self.start_current()?;

        let mut path = Path::from(root);
        while self
            .peek_expect_tokens(vec![Token::DoubleColon], true)
            .is_some()
        {
            let identifier = self.parse_identifier()?;
            path.push(identifier.raw)
        }

        return Ok(self.create_located(path));
    }
}
