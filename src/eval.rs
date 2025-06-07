use crate::expressions::{BinaryOperator, Expression};

pub fn evaluate_expression(expr: Expression) -> Result<u64, String> {
    Ok(match expr {
        Expression::Number(n) => n,
        Expression::BinaryOp {
            left,
            operator,
            right,
        } => {
            let left = evaluate_expression(*left)?;
            let right = evaluate_expression(*right)?;
            match operator {
                BinaryOperator::Plus => left.wrapping_add(right),
                BinaryOperator::Minus => left.wrapping_sub(right),
                BinaryOperator::Multiply => left.wrapping_mul(right),
                BinaryOperator::Divide => left.wrapping_div(right),
                _ => return Err(format!("Binary operator {:?} not supported", operator)),
            }
        }
        _ => return Err(format!("Expression {:?} not supported", expr)),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::combinator::map_res;
    use BinaryOperator::*;

    #[test]
    fn test_evaluate_expression() {
        assert_eq!(evaluate_expression(Expression::Number(42)), Ok(42));

        assert_eq!(
            evaluate_expression(Expression::BinaryOp {
                left: Box::new(Expression::Number(42)),
                operator: Plus,
                right: Box::new(Expression::Number(42))
            }),
            Ok(84)
        );
        assert_eq!(
            evaluate_expression(Expression::BinaryOp {
                left: Box::new(Expression::Number(42)),
                operator: Minus,
                right: Box::new(Expression::Number(42))
            }),
            Ok(0)
        );
        assert_eq!(
            evaluate_expression(Expression::BinaryOp {
                left: Box::new(Expression::Number(42)),
                operator: Multiply,
                right: Box::new(Expression::Number(42))
            }),
            Ok(1764)
        );
        assert_eq!(
            evaluate_expression(Expression::BinaryOp {
                left: Box::new(Expression::Number(42)),
                operator: Divide,
                right: Box::new(Expression::Number(42))
            }),
            Ok(1)
        );
    }

    fn expr_result(input: &str, expected: u64) {
        assert_done!(
            map_res(crate::expressions::expression, evaluate_expression)(input),
            expected
        );
    }

    #[test]
    fn test_parsed_expressions() {
        expr_result("42 - (20 + 21)", 1);
        expr_result("42 - (4 * 8)", 10);
        expr_result("42", 42);
        expr_result("42 + 42", 84);
        expr_result("42 - 42", 0);
        expr_result("42 * 42", 1764);
        expr_result("42 / 42", 1);
        expr_result("0x2000000 + (4k * 4)", 0x2000000 + (4 * 1024 * 4));
    }
}
