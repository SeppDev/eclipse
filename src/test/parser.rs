#[cfg(test)]
mod parser {
    use std::path::PathBuf;

    use crate::compiler::{
        nodes::{
            ast::{
                Node,
                RawNode::{self, *},
            },
            shared::{
                ArithmethicOperator::{self, *},
                CompareOperator::{self, *},
            },
        },
        CompilerCtx,
    };

    macro_rules! parser_test {
        ($name:ident, $input:expr, $expected:expr) => {
            #[test]
            fn $name() {
                test_parser($input, $expected);
            }
        };
    }

    fn node_eq(a: &Node, b: &Node) -> bool {
        match (&a.raw, &b.raw) {
            (
                ArithmethicOperation {
                    left,
                    right,
                    operator,
                },
                ArithmethicOperation {
                    left: left_other,
                    right: right_other,
                    operator: operator_other,
                },
            ) => {
                node_eq(&left, &left_other)
                    && node_eq(&right, &right_other)
                    && operator == operator_other
            }
            (
                CompareOperation {
                    left,
                    right,
                    operator,
                },
                CompareOperation {
                    left: left_other,
                    right: right_other,
                    operator: operator_other,
                },
            ) => {
                node_eq(&left, &left_other)
                    && node_eq(&right, &right_other)
                    && operator == operator_other
            }
            (a, b) => a == b,
        }
    }

    fn test_parser(input: &str, expected: RawNode) {
        let compiler = CompilerCtx::test();
        let tokens = compiler.tokenize(input).unwrap();
        let mut parser = compiler.new_parser(tokens, PathBuf::default());
        let expected = expected.into();
        let node = parser.expect_node().unwrap();

        assert!(node_eq(&node, &expected), "{} != ( {} )", expected, node);
    }

    fn arithmetic(left: RawNode, right: RawNode, operator: ArithmethicOperator) -> RawNode {
        RawNode::ArithmethicOperation {
            left: Box::new(left.into()),
            right: Box::new(right.into()),
            operator,
        }
    }

    fn compare(left: RawNode, right: RawNode, operator: CompareOperator) -> RawNode {
        RawNode::CompareOperation {
            left: Box::new(left.into()),
            right: Box::new(right.into()),
            operator,
        }
    }

    parser_test!(
        add,
        "1 + 2",
        arithmetic(Integer("1".into()), Integer("2".into()), Plus)
    );

    parser_test!(
        order_of_operations,
        "1 * 2 + 3",
        arithmetic(
            arithmetic(Integer("1".into()), Integer("2".into()), Multiply),
            Integer("3".into()),
            Plus
        )
    );

    parser_test!(
        identifier_add,
        "a + 2",
        arithmetic(Identifier("a".into()), Integer("2".into()), Plus)
    );

    parser_test!(
        compare_integer,
        "1 == 2",
        compare(Integer("1".into()), Integer("2".into()), Compare)
    );

    parser_test!(identifier, "x", Identifier("x".into()));
}
