#[cfg(test)]
mod parser {
    use crate::{
        common::position::LocatedAt,
        compiler::{
            nodes::{
                ast::{
                    Node,
                    RawNode::{self, *},
                    RawType::{self, *},
                    Type,
                },
                shared::{
                    ArithmethicOperator::{self, *},
                    CompareOperator::{self, *},
                    EqualsOperation::{self, *},
                },
            },
            CompilerCtx, Path,
        },
        FILE_EXTENSION,
    };

    macro_rules! parser_test {
        ($name:ident, $input:expr, $expected:expr) => {
            #[test]
            fn $name() {
                test_parser_eq($input, $expected);
            }
        };
    }

    fn test_init(input: &str) -> Node {
        let mut compiler = CompilerCtx::test();

        let mut main_path = Path::new().join("src").join("main");
        main_path.set_extension(FILE_EXTENSION);

        compiler.files.cache(compiler.resolve_path(main_path), input);
        let mut nodes = compiler.parse().unwrap();
        assert!(nodes.len() == 1, "Expected only 1 node and got more");
        nodes.pop().unwrap()
    }

    fn test_parser_eq(input: &str, expected: RawNode) {
        let expected: Node = expected.into();
        let node = test_init(input);

        assert!(
            node == expected,
            "INPUT: {}\n{}\n-------------------------------------\n{}",
            input,
            expected,
            node
        );
    }

    fn identifier(string: impl ToString) -> RawNode {
        Identifier(string.to_string())
    }
    fn integer(string: impl ToString) -> RawNode {
        Integer(string.to_string())
    }

    fn arithmetic(left: RawNode, right: RawNode, operator: ArithmethicOperator) -> RawNode {
        RawNode::ArithmethicOperation {
            left: Box::new(left.into()),
            right: Box::new(right.into()),
            operator,
        }
    }

    fn compare(left: RawNode, right: RawNode, operator: CompareOperator) -> RawNode {
        CompareOperation {
            left: Box::new(left.into()),
            right: Box::new(right.into()),
            operator,
        }
    }

    fn field(node: RawNode, field: &'static str) -> RawNode {
        Field(Box::new(node.into()), field.to_string().into())
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
    fn r#struct(name: impl ToString, fields: Vec<(impl ToString, RawType)>) -> RawNode {
        Struct {
            name: name.to_string().into(),
            fields: fields
                .into_iter()
                .map(|(name, data_type)| (name.to_string().into(), data_type.into()))
                .collect(),
        }
    }
    fn r#enum(name: impl ToString, fields: Vec<impl ToString>) -> RawNode {
        Enum {
            name: name.to_string().into(),
            items: fields
                .into_iter()
                .map(|name| name.to_string().into())
                .collect(),
        }
    }

    fn r#while(condition: RawNode, body: RawNode) -> RawNode {
        let condition = condition.into();
        let body = body.into();
        While { condition, body }
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

    parser_test!(simple_loop, "loop {}", Loop(Block(vec![]).into()));

    parser_test!(
        simple_while,
        "while true {}",
        r#while(Bool(true), Block(vec![]))
    );

    parser_test!(
        person_struct,
        "struct Person { name String, age i16 }",
        r#struct("Person", vec![("name", RawType::String), ("age", Int(32))])
    );

    parser_test!(
        simple_enum,
        "enum Fruits { Apple, Pear, Orange, Banana }",
        r#enum("Person", vec!["Apple", "Pear", "Orange", "Banana"])
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
