use crate::compiler::{
    lexer::{Token, Tokens},
    path::Path,
};

pub fn parse_path(tokens: &mut Tokens, root: &String) -> Path {
    let mut path = Path::from(root);
    loop {
        if !tokens.peek_expect_token(Token::DoubleColon, true) {
            break;
        }
        let info = tokens.expect_tokens(vec![Token::Identifier(String::new())], false);
        match info.token {
            Token::Identifier(name) => path.add(name),
            _ => panic!()
        }
    }

    path
}
