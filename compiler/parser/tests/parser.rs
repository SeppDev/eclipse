#[cfg(test)]
mod tests {
    use common::position::LocatedAt;
    use lexer::tokenize;
    use parser::parse;
    use syntax::ast::{
        Node,
        RawNode::{self, *},
        RawType::*,
        Type,
    };
    use syntax::operators::{
        ArithmeticOperator::{self, *},
        CompareOperator::{self, *},
        EqualsOperation::{self, *},
        Operator,
    };

    macro_rules! parser_test {
        ($name:ident, $input:expr, $expected:expr) => {
            #[test]
            fn $name() {
                test_parser_eq($input, $expected);
            }
        };
    }

    fn test_init(input: &'static str) -> Vec<Node> {
        // let project_path = PathBuf::from("project");
        // let mut resolver = MockResolver::new();

        // let mut main_path = project_path.join("main");
        // main_path.set_extension(FILE_EXTENSION);
        // resolver.insert(main_path, input);

        // let mut compiler = CompilerCtx::builder()
        //     .project_path(project_path.into())
        //     .resolver(resolver)
        //     .build();

        let tokens = tokenize(input).unwrap();
        let nodes = parse(tokens).unwrap();
        nodes
    }

    fn test_parser_eq(input: &'static str, expected: RawNode) {
        let expected = expected.into();
        let nodes = test_init(input);
        let node = nodes.first().unwrap();

        assert!(
            node == &expected,
            "INPUT: {input}\n----------------------------\nEXPECTED: {expected:#?}\n----------------------------\nRESULT: {node:#?}\n----------------------------\nNODES: {nodes:#?}",
        );
    }

    fn identifier(string: impl ToString) -> RawNode {
        Identifier(string.to_string())
    }
    fn integer(string: impl ToString) -> RawNode {
        Integer(string.to_string())
    }

    fn arithmetic(left: RawNode, right: RawNode, operator: ArithmeticOperator) -> RawNode {
        RawNode::Operation {
            left: Box::new(left.into()),
            right: Box::new(right.into()),
            operator: Operator::Arithmetic(operator),
        }
    }

    fn compare(left: RawNode, right: RawNode, operator: CompareOperator) -> RawNode {
        RawNode::Operation {
            left: Box::new(left.into()),
            right: Box::new(right.into()),
            operator: Operator::Compare(operator),
        }
    }

    fn field(node: RawNode, field: &'static str) -> RawNode {
        Field(
            Box::new(node.into()),
            Box::new(LocatedAt::value(RawNode::Identifier(field.to_string()))),
        )
    }

    fn block(nodes: Vec<RawNode>) -> RawNode {
        Block(nodes.into_iter().map(|n| n.into()).collect())
    }
    fn tuple(nodes: Vec<RawNode>) -> RawNode {
        RawNode::Tuple(nodes.into_iter().map(|n| n.into()).collect())
    }
    fn wrapped(node: RawNode) -> RawNode {
        RawNode::Wrapped(Some(node.into()))
    }

    fn declare(
        mutable: bool,
        name: impl ToString,
        data_type: Option<Type>,
        value: RawNode,
    ) -> RawNode {
        Declare {
            mutable: mutable.then_some(LocatedAt::from(())),
            name: name.to_string().into(),
            data_type,
            node: value.into(),
        }
    }
    fn set_path(path: impl ToString, operation: EqualsOperation, value: RawNode) -> RawNode {
        SetPath {
            path: path.to_string().into(),
            operation,
            value: value.into(),
        }
    }

    parser_test!(only_block, "{}", block(Vec::new()));

    parser_test!(
        variable_declaration,
        "var x = 5",
        declare(false, "x", None, integer("5"))
    );
    parser_test!(
        variable_declaration_with_type,
        "var x: i32 = 5",
        declare(false, "x", Some(Int(32).into()), integer("5"))
    );
    parser_test!(set_value, "x = 2", set_path("x", Equals, integer("2")));
    parser_test!(
        plus_equals,
        "x += 2",
        set_path("x", PlusEquals, integer("2"))
    );

    parser_test!(add, "1 + 2", arithmetic(integer("1"), integer("2"), Plus));

    parser_test!(
        order_of_operations,
        "1 * 2 + 3",
        arithmetic(
            arithmetic(integer("1"), integer("2"), Multiply),
            integer("3"),
            Plus
        )
    );

    parser_test!(one_field, "a.b", field(identifier("a"), "b"));
    parser_test!(
        field_with_order,
        "a.b * c.d + e.f",
        arithmetic(
            arithmetic(
                field(Identifier("a".into()), "b"),
                field(Identifier("c".into()), "d"),
                Multiply
            ),
            field(Identifier("e".into()), "f"),
            Plus
        )
    );
    parser_test!(
        add_field,
        "a.b + c.d",
        arithmetic(
            field(identifier("a"), "b"),
            field(identifier("c"), "d"),
            Plus
        )
    );

    parser_test!(
        fields,
        "a.b.c.d",
        field(field(field(identifier("a"), "b"), "c"), "d")
    );

    parser_test!(
        identifier_add,
        "a + 2",
        arithmetic(identifier("a"), integer("2"), Plus)
    );

    parser_test!(
        compare_integer,
        "1 == 2",
        compare(integer("1"), integer("2"), Compare)
    );

    parser_test!(
        keyword_expression,
        "{continue return 1 + b}",
        block(vec![
            Continue(None),
            Return(Some(arithmetic(integer("1"), identifier("b"), Plus).into()))
        ])
    );
    parser_test!(
        integers_tuple,
        "(1,2,3)",
        tuple(vec![integer("1"), integer("2"), integer("3")])
    );
    parser_test!(integer_wrapped, "(1)", wrapped(integer("1")));
}
