#[cfg(test)]
mod tests {
    use lexer::{
        token::{
            Token,
            TokenKind::{self, *},
        },
        tokenize,
    };

    fn test_lexer(input: &str, expected: Vec<TokenKind>) {
        let tokens = tokenize(input).unwrap();
        token_stream_eq(tokens, expected);
    }

    fn token_stream_eq(stream: Vec<Token>, expected: Vec<TokenKind>) {
        let mut stream_kinds: Vec<TokenKind> = stream.into_iter().map(|t| t.kind).collect();
        let _ = stream_kinds.pop();

        assert!(
            stream_kinds.len() == expected.len(),
            "{} expected/{} stream -> ({:#?}, {:#?})",
            expected.len(),
            stream_kinds.len(),
            expected,
            stream_kinds
        );

        for (a, b) in stream_kinds.iter().zip(expected.iter()) {
            assert!(a == b, "{a:?} != {b:?}")
        }
    }

    macro_rules! lexer_test {
        ($name:ident, $input:expr, [ $($expected:expr),* $(,)? ]) => {
            #[test]
            fn $name() {
                test_lexer($input, vec![$($expected),*]);
            }
        };
    }

    lexer_test!(
        main_function,
        "func main() {}",
        [
            Function,
            Identifier,
            OpenParen,
            CloseParen,
            OpenCurlyBracket,
            CloseCurlyBracket
        ]
    );

    lexer_test!(only_block, "{}", [OpenCurlyBracket, CloseCurlyBracket]);
    lexer_test!(
        block_comment,
        "{} // this is a comment",
        [OpenCurlyBracket, CloseCurlyBracket]
    );

    lexer_test!(
        add_one,
        "func add_one(x i32) { return x + 1 }",
        [
            Function,
            Identifier,
            OpenParen,
            Identifier,
            Identifier,
            CloseParen,
            OpenCurlyBracket,
            Return,
            Identifier,
            Plus,
            Integer,
            CloseCurlyBracket
        ]
    );

    lexer_test!(
        parens,
        "(1 + 2)",
        [OpenParen, Integer, Plus, Integer, CloseParen]
    );

    lexer_test!(add, "1 + 2", [Integer, Plus, Integer]);
    lexer_test!(subtract, "1 - 2", [Integer, Minus, Integer]);
    lexer_test!(string_literal, "\"hello\"", [Text]);
    lexer_test!(compare, "a == b", [Identifier, Compare, Identifier]);
    lexer_test!(
        variable_decleration,
        "var x = 1234",
        [Var, Identifier, Equals, Integer]
    );
    lexer_test!(not_a_float, "1.b", [Integer, Dot, Identifier]);
    lexer_test!(block, "{  }", [OpenCurlyBracket, CloseCurlyBracket]);
    lexer_test!(
        integer_after_string_literal,
        "\"hello\" 1234",
        [Text, Integer]
    );
}
