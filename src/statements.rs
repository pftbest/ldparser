use expressions::expression;
use expressions::Expression;
use idents::{string, symbol};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::combinator::opt;
use nom::IResult;
use whitespace::opt_space;

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
    Assert {
        expr: Box<Expression>,
        text: String,
    },
}

fn assign_operator(input: &str) -> IResult<&str, AssignOperator> {
    map(
        alt((
            tag("="),
            tag("+="),
            tag("-="),
            tag("*="),
            tag("/="),
            tag("<<="),
            tag(">>="),
            tag("&="),
            tag("|="),
        )),
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
            _ => panic!("wrong operator"),
        },
    )(input)
}

fn special_assign(input: &str) -> IResult<&str, Statement> {
    let (input, keyword) = alt((tag("PROVIDE_HIDDEN"), tag("PROVIDE"), tag("HIDDEN")))(input)?;
    let (input, _) = wsc!(tag("("))(input)?;
    let (input, name) = symbol(input)?;
    let (input, _) = wsc!(tag("="))(input)?;
    let (input, expr) = expression(input)?;
    let (input, _) = wsc!(tag(")"))(input)?;
    let (input, _) = tag(";")(input)?;
    Ok((
        input,
        match keyword {
            "HIDDEN" => Statement::Hidden {
                name: name.into(),
                expression: Box::new(expr),
            },
            "PROVIDE" => Statement::Provide {
                name: name.into(),
                expression: Box::new(expr),
            },
            "PROVIDE_HIDDEN" => Statement::ProvideHidden {
                name: name.into(),
                expression: Box::new(expr),
            },
            _ => panic!("invalid assign keyword"),
        },
    ))
}

fn assign(input: &str) -> IResult<&str, Statement> {
    let (input, name) = symbol(input)?;
    let (input, op) = wsc!(assign_operator)(input)?;
    let (input, expr) = expression(input)?;
    let (input, _) = opt_space(input)?;
    let (input, _) = tag(";")(input)?;
    Ok((
        input,
        Statement::Assign {
            name: name.into(),
            operator: op,
            expression: Box::new(expr),
        },
    ))
}

fn assert_stmt(input: &str) -> IResult<&str, Statement> {
    let (input, _) = tag("ASSERT")(input)?;
    let (input, _) = wsc!(tag("("))(input)?;
    let (input, expr) = expression(input)?;
    let (input, _) = wsc!(tag(","))(input)?;
    let (input, text) = string(input)?;
    let (input, _) = wsc!(tag(")"))(input)?;
    let (input, _) = opt(tag(";"))(input)?;
    Ok((
        input,
        Statement::Assert {
            expr: Box::new(expr),
            text: text.into(),
        },
    ))
}

pub fn statement(input: &str) -> IResult<&str, Statement> {
    alt((special_assign, assign, assert_stmt))(input)
}

#[cfg(test)]
mod tests {
    use expressions::Expression;
    use statements::*;

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
