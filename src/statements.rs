use expressions::Expression;
use expressions::expression;
use whitespace::opt_space;
use idents::{symbol, string};

#[derive(Debug, PartialEq)]
pub enum AssignOperator {
    Equals,
    Plus,
    Minus,
    Multiply,
    Divide,
    ShiftLeft,
    ShiftRight,
    And,
    Or,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Assign {
        name: String,
        operator: AssignOperator,
        expression: Box<Expression>,
    },
    Hidden {
        name: String,
        expression: Box<Expression>,
    },
    Provide {
        name: String,
        expression: Box<Expression>,
    },
    ProvideHidden {
        name: String,
        expression: Box<Expression>,
    },
    Assert { expr: Box<Expression>, text: String },
}

named!(assign_operator<&str, AssignOperator>, map!(
    alt_complete!(
        tag!("=") | tag!("+=") | tag!("-=") | tag!("*=") | tag!("/=") |
        tag!("<<=") | tag!(">>=") | tag!("&=") | tag!("|=")
    ),
    |op: &str| match op {
        "=" => AssignOperator::Equals,
        "+=" => AssignOperator::Plus,
        "-=" => AssignOperator::Minus,
        "*=" => AssignOperator::Multiply,
        "/=" => AssignOperator::Divide,
        "<<=" => AssignOperator::ShiftLeft,
        ">>=" => AssignOperator::ShiftRight,
        "&=" => AssignOperator::And,
        "|=" => AssignOperator::Or,
        _ => panic!("wrong operator")
    }
));

named!(special_assign<&str, Statement>, do_parse!(
    keyword: alt_complete!(
        tag!("PROVIDE_HIDDEN") | tag!("PROVIDE") | tag!("HIDDEN")
    )
    >>
    wsc!(tag!("("))
    >>
    name: symbol
    >>
    wsc!(tag!("="))
    >>
    expr: expression
    >>
    wsc!(tag!(")"))
    >>
    tag!(";")
    >>
    (match keyword {
        "HIDDEN" => Statement::Hidden {
            name: name.into(),
            expression: Box::new(expr)
        },
        "PROVIDE" => Statement::Provide {
            name: name.into(),
            expression: Box::new(expr)
        },
        "PROVIDE_HIDDEN" => Statement::ProvideHidden {
            name: name.into(),
            expression: Box::new(expr)
        },
        _ => panic!("invalid assign keyword")
    })
));

named!(assign<&str, Statement>, do_parse!(
    name: symbol
    >>
    op: wsc!(assign_operator)
    >>
    expr: expression
    >>
    opt_space
    >>
    tag!(";")
    >>
    (Statement::Assign {
        name: name.into(),
        operator: op,
        expression: Box::new(expr)
    })
));

named!(assert_stmt<&str, Statement>, do_parse!(
    tag!("ASSERT")
    >>
    wsc!(tag!("("))
    >>
    expr: expression
    >>
    wsc!(tag!(","))
    >>
    text: string
    >>
    wsc!(tag!(")"))
    >>
    opt_complete!(tag!(";"))
    >>
    (Statement::Assert {
        expr: Box::new(expr),
        text: text.into(),
    })
));

named!(pub statement<&str, Statement>, alt_complete!(
    special_assign | assign | assert_stmt
));

#[cfg(test)]
mod tests {
    use statements::*;
    use expressions::Expression;

    #[test]
    fn test_statement() {
        assert_done!(
            statement("A = 11 ;"),
            Statement::Assign {
                name: "A".into(),
                operator: AssignOperator::Equals,
                expression: Box::new(Expression::Number(11)),
            }
        );
        assert_done!(
            statement("PROVIDE ( x = x ) ;"),
            Statement::Provide {
                name: "x".into(),
                expression: Box::new(Expression::Ident("x".into())),
            }
        );
        assert_done!(statement("PROBLEM += HELLO ( WORLD , 0 ) + 1 ;"));
    }
}
