use crate::expressions::{BinaryOperator, Expression};

use crate::script::PARSE_STATE;

fn _evaluate_expression(expr: &Expression) -> Result<u64, String> {
    Ok(match expr {
        Expression::Number(n) => *n,
        Expression::Ident(s) => {
            return PARSE_STATE.with_borrow(|state| {
                for item in &state.items {
                    if let crate::RootItem::Statement(stmt) = item {
                        if let crate::Statement::Assign {
                            name, expression, ..
                        } = stmt
                        {
                            if name == s {
                                return _evaluate_expression(&**expression);
                            }
                        }
                    }
                }
                Err(format!("Variable {:?} not found", s))
            });
        }
        Expression::Call {
            function,
            arguments,
        } => {
            return match function.as_str() {
                "ORIGIN" | "LENGTH" => {
                    if arguments.len() != 1 {
                        return Err(format!("function {:?} only support 1 argument", function));
                    }
                    if let Expression::Ident(s) = &arguments[0] {
                        return PARSE_STATE.with_borrow(|state| {
                            for item in &state.items {
                                if let crate::RootItem::Memory { regions } = item {
                                    for region in regions {
                                        if region.name == *s {
                                            return Ok(match function.as_str() {
                                                "ORIGIN" => region.origin,
                                                "LENGTH" => region.length,
                                                _ => unreachable!(),
                                            });
                                        }
                                    }
                                }
                            }
                            Err(format!("Variable {:?} not found", s))
                        });
                    } else {
                        return Err(format!("function {:?} argument must be string", function));
                    }
                }
                _ => Err(format!("function {:?} not supported", function)),
            }
        }
        Expression::BinaryOp {
            left,
            operator,
            right,
        } => {
            let left = _evaluate_expression(&**left)?;
            let right = _evaluate_expression(&**right)?;
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

pub fn evaluate_expression(expr: Expression) -> Result<u64, String> {
    _evaluate_expression(&expr)
}

#[cfg(test)]
mod tests {
    use crate::{script::clear_state, AssignOperator, Region, RootItem, Statement};

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

        clear_state();
        PARSE_STATE.with_borrow_mut(|state| {
            state.items.push(RootItem::Statement(Statement::Assign {
                name: "A".into(),
                operator: AssignOperator::Equals,
                expression: Box::new(Expression::Number(11)),
            }));
        });
        expr_result("A * 2", 22);
        PARSE_STATE.with_borrow_mut(|state| {
            state.items.push(RootItem::Statement(Statement::Assign {
                name: "B".into(),
                operator: AssignOperator::Equals,
                expression: Box::new(Expression::BinaryOp {
                    left: Box::new(Expression::Number(2)),
                    operator: BinaryOperator::Plus,
                    right: Box::new(Expression::Number(4)),
                }),
            }));
        });
        expr_result("A * B", 66);
        PARSE_STATE.with_borrow_mut(|state| {
            state.items.push(RootItem::Memory {
                regions: vec![Region {
                    name: String::from("AA"),
                    origin: 66,
                    length: 12,
                }],
            });
        });
        expr_result("ORIGIN(AA)", 66);
        expr_result("LENGTH(AA)", 12);
    }
}
