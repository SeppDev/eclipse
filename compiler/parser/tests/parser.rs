#[cfg(test)]
#[allow(unused)]
mod tests {
    use common::position::{LocatedAt, PositionRange};
    use diagnostics::DiagnosticResult;
    use lexer::tokenize;
    use parser::parse;
    use syntax::ast::{
        self, Node,
        RawNode::{self, *},
        RawParameter,
        RawType::{self, *},
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
                parser_eq($input, $expected);
            }
        };
    }
    macro_rules! parser_test_fail {
        ($name:ident, $input:expr) => {
            #[test]
            fn $name() {
                parser_neq($input);
            }
        };
    }

    fn init(input: &'static str) -> DiagnosticResult<Vec<Node>> {
        let tokens = tokenize(input)?;
        parse(tokens)
    }

    fn parser_eq(input: &'static str, expected: RawNode) {
        let expected = expected.into();
        let nodes = init(input).unwrap();
        let node = nodes.first().unwrap();

        assert!(
            node == &expected,
            "INPUT: {input}\n----------------------------\nEXPECTED: {expected:#?}\n----------------------------\nRESULT: {node:#?}\n----------------------------\nNODES: {nodes:#?}",
        );
    }
    fn parser_neq(input: &'static str) {
        init(input).expect_err("Expected to fail!");
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
    fn empty_block() -> RawNode {
        Block(Vec::new())
    }

    fn tuple(nodes: Vec<RawNode>) -> RawNode {
        RawNode::Tuple(nodes.into_iter().map(|n| n.into()).collect())
    }
    fn wrapped(node: RawNode) -> RawNode {
        RawNode::Wrapped(Some(node.into()))
    }
    fn function(
        name: &str,
        parameters: Vec<RawParameter>,
        return_type: RawType,
        body: RawNode,
    ) -> RawNode {
        RawNode::Function {
            name: name.to_string().into(),
            parameters: parameters.into_iter().map(|param| param.into()).collect(),
            return_type: return_type.into(),
            body: body.into(),
        }
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
    fn condition(
        condition: RawNode,
        body: RawNode,
        conditions: Vec<(RawNode, RawNode)>,
        else_condition: Option<RawNode>,
    ) -> RawNode {
        Conditional {
            condition: condition.into(),
            body: body.into(),
            conditions: conditions
                .into_iter()
                .map(|(cond, body)| (cond.into(), body.into()))
                .collect(),
            else_condition: match else_condition {
                Some(body) => Some(Box::new(body.into())),
                None => None,
            },
        }
    }
    fn parameter(mutable: bool, name: &str, data_type: RawType) -> RawParameter {
        RawParameter {
            reference: None,
            mutable: mutable.then_some(LocatedAt::default()),
            name: name.to_string().into(),
            data_type: data_type.into(),
        }
    }
    fn attribute(key: &str) -> RawNode {
        ast::RawNode::Attribute(ast::Attribute::new(
            ast::RawAttribute::Simple(key.to_string().into()),
            PositionRange::default(),
        ))
    }

    parser_test_fail!(non_closing_block, "{");

    parser_test!(only_block, "{}", block(Vec::new()));
    parser_test!(
        main_function,
        "func main() void {}",
        function("main", vec![], RawType::Void, empty_block())
    );
    parser_test!(
        main_function_no_type,
        "func main() {}",
        function("main", vec![], RawType::Void, empty_block())
    );
    parser_test!(attributes, "#[test]", attribute("test"));
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
        add_field_left,
        "a + b.c",
        arithmetic(identifier("a"), field(identifier("b"), "c"), Plus)
    );
    parser_test!(
        add_field_right,
        "a.b + c",
        arithmetic(field(identifier("a"), "b"), identifier("c"), Plus)
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
    parser_test!(
        if_condition,
        "if true == true {}",
        condition(
            compare(Bool(true), Bool(true), CompareOperator::Compare),
            empty_block(),
            Vec::new(),
            None
        )
    );
    parser_test!(
        if_condition_else,
        "if true == true {} else {}",
        condition(
            compare(Bool(true), Bool(true), CompareOperator::Compare),
            empty_block(),
            Vec::new(),
            Some(empty_block())
        )
    );
}
