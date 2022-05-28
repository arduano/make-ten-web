use std::cmp::Ordering;

use super::operation::{Operation, OperationKind};
use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Expression {
    Op(Box<Operation>),
    Num(i32),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct EvaluatedExpr {
    value: i32,
    expression: Expression,
}

impl Expression {
    pub fn to_text(&self) -> String {
        match self {
            Expression::Op(op) => op.to_text(),
            Expression::Num(num) => num.to_string(),
        }
    }

    pub fn to_text_child(&self, parent_op: OperationKind, is_left: bool) -> String {
        match self {
            Expression::Op(op) => op.to_text_child(parent_op, is_left),
            Expression::Num(num) => num.to_string(),
        }
    }

    /// Create a new expression from a number
    pub fn new_num(num: i32) -> EvaluatedExpr {
        EvaluatedExpr::new(Expression::Num(num))
    }

    /// Create a new expression from an operation
    pub fn new_op(
        left: EvaluatedExpr,
        right: EvaluatedExpr,
        kind: OperationKind,
    ) -> Option<EvaluatedExpr> {
        let left_val = left.value;
        let right_val = right.value;

        match kind {
            OperationKind::Divide => {
                if right_val == 0 || left_val % right_val != 0 {
                    return None;
                }

                // Only leave multiply by zero instead
                if left_val == 0 {
                    return None;
                }

                // Only leave multiply by one instead
                if right_val == 1 {
                    return None;
                }
            }
            OperationKind::Subtract => {
                if left_val < right_val {
                    return None;
                }

                // Only leave add zero instead
                if right_val == 0 {
                    return None;
                }
            }
            OperationKind::Power => {
                if right_val < 0 {
                    return None;
                }

                // Only leave multiply by one instead
                if right_val == 1 {
                    return None;
                }

                // If the number is overflowing, then ignore
                if left_val.checked_pow(right_val as u32).is_none() {
                    return None;
                }
            }
            _ => {}
        }

        let expr = Expression::Op(Box::new(Operation { left, right, kind }));

        Some(EvaluatedExpr::new(expr))
    }

    /// Compare the precedence of the expression. This is useful for shuffling
    /// expressions into a normalized form.
    pub fn compare_shuffle_precidence(&self, other: &Self) -> Ordering {
        match &self {
            Expression::Num(n1) => match other {
                Expression::Num(n2) => n1.cmp(&n2),
                _ => Ordering::Less,
            },
            op1 => match &other {
                Expression::Num(_) => Ordering::Greater,
                op2 => {
                    let depth_ord = op1.depth().cmp(&op2.depth());
                    if depth_ord == Ordering::Equal {
                        op1.evaluate().cmp(&op2.evaluate())
                    } else {
                        depth_ord
                    }
                }
            },
        }
    }
}

impl Evaluate for Expression {
    fn evaluate(&self) -> i32 {
        match self {
            Expression::Num(n) => *n,
            Expression::Op(op) => op.evaluate(),
        }
    }
}

impl Depth for Expression {
    fn depth(&self) -> usize {
        match self {
            Expression::Num(_) => 1,
            Expression::Op(op) => op.depth() + 1,
        }
    }
}

impl ExpressionEquals for Expression {
    fn expr_equals(&self, other: &Expression) -> bool {
        match self {
            Expression::Num(n) => match other {
                Expression::Num(m) => *n == *m,
                _ => false,
            },
            Expression::Op(op) => match other {
                Expression::Op(op2) => op.expr_equals(op2),
                _ => false,
            },
        }
    }
}

impl Complexity for Expression {
    fn get_complexity(&self) -> u32 {
        match self {
            Expression::Num(_) => 10,
            Expression::Op(op) => op.get_complexity(),
        }
    }

    fn get_complexity_internal(&self, parent_op: OperationKind, is_left: bool) -> u32 {
        match self {
            Expression::Num(_) => 10,
            Expression::Op(op) => op.get_complexity_internal(parent_op, is_left),
        }
    }
}

impl std::ops::Deref for EvaluatedExpr {
    type Target = Expression;

    fn deref(&self) -> &Self::Target {
        &self.expression
    }
}

impl std::ops::DerefMut for EvaluatedExpr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.expression
    }
}

impl EvaluatedExpr {
    fn new(expression: Expression) -> EvaluatedExpr {
        EvaluatedExpr {
            value: expression.evaluate(),
            expression,
        }
    }

    pub fn re_evaluate(&mut self) {
        self.value = self.expression.evaluate();
        if let Expression::Op(op) = &mut self.expression {
            op.re_evaluate();
        }
    }
}
