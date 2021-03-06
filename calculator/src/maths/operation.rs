use super::expression::EvaluatedExpr;
use super::*;

#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
pub enum OperationKind {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Operation {
    pub left: EvaluatedExpr,
    pub right: EvaluatedExpr,
    pub kind: OperationKind,
}

impl Operation {
    /// Converts the operation into text
    pub fn to_text(&self) -> String {
        let left = self.left.to_text_child(self.kind, true);
        let right = self.right.to_text_child(self.kind, false);

        match self.kind {
            OperationKind::Add => format!("{} + {}", left, right),
            OperationKind::Subtract => format!("{} - {}", left, right),
            OperationKind::Multiply => format!("{} * {}", left, right),
            OperationKind::Divide => format!("{} / {}", left, right),
            OperationKind::Power => format!("{} ^ {}", left, right),
        }
    }

    /// Converts the operation into text, except considering the operator precedence to include or ignore parenthises
    pub fn to_text_child(&self, parent_op: OperationKind, is_left: bool) -> String {
        let use_parenthises = is_operator_greater_than(self.kind, parent_op) || !is_left;

        if use_parenthises {
            format!("({})", self.to_text())
        } else {
            self.to_text()
        }
    }

    /// Recursively update the EvaluatedExpr cache
    pub fn re_evaluate(&mut self) {
        self.left.re_evaluate();
        self.right.re_evaluate();
    }
}

impl Evaluate for Operation {
    fn evaluate(&self) -> i32 {
        match self.kind {
            OperationKind::Add => self.left.evaluate() + self.right.evaluate(),
            OperationKind::Subtract => self.left.evaluate() - self.right.evaluate(),
            OperationKind::Multiply => self.left.evaluate() * self.right.evaluate(),
            OperationKind::Divide => self.left.evaluate() / self.right.evaluate(),
            OperationKind::Power => self.left.evaluate().pow(self.right.evaluate() as u32),
        }
    }
}

impl Depth for Operation {
    fn depth(&self) -> usize {
        let left_depth = self.left.depth();
        let right_depth = self.right.depth();

        left_depth.max(right_depth)
    }
}

impl ExpressionEquals for Operation {
    fn expr_equals(&self, other: &Operation) -> bool {
        if self.kind != other.kind {
            return false;
        }

        let mut same = self.left.expr_equals(&other.left) && self.right.expr_equals(&other.right);

        // Reverse addition/multiplication are equal
        match self.kind {
            OperationKind::Add | OperationKind::Multiply => {
                same |= self.left.expr_equals(&other.right) && self.right.expr_equals(&other.left);
            }
            _ => {}
        }

        // Ignore redundant operations
        match self.kind {
            OperationKind::Power => {
                if self.left.evaluate() == 1 && other.left.evaluate() == 1 {
                    same = true;
                }
                if self.right.evaluate() == 0 && other.right.evaluate() == 0 {
                    same = true;
                }
            }
            OperationKind::Divide => {
                if self.right.evaluate() == 1 && other.right.evaluate() == 1 {
                    same = true;
                }
                if self.left.evaluate() == 0 && other.left.evaluate() == 0 {
                    same = true;
                }
            }
            OperationKind::Multiply => {
                if self.left.evaluate() == 0 && other.left.evaluate() == 0 {
                    same = true;
                }
                if self.right.evaluate() == 0 && other.right.evaluate() == 0 {
                    same = true;
                }
            }
            _ => {}
        }

        same
    }
}

impl Complexity for Operation {
    fn get_complexity(&self) -> u32 {
        let left = self.left.get_complexity_internal(self.kind, true);
        let right = self.right.get_complexity_internal(self.kind, false);

        let complexity = left + right;

        match self.kind {
            OperationKind::Add | OperationKind::Subtract => complexity,
            OperationKind::Multiply | OperationKind::Divide => complexity * 2,
            OperationKind::Power => complexity * 5,
        }
    }

    fn get_complexity_internal(&self, parent_op: OperationKind, is_left: bool) -> u32 {
        let internal_complexity = self.get_complexity();

        let use_parenthises = is_operator_greater_than(self.kind, parent_op) || !is_left;

        if use_parenthises {
            internal_complexity + 10
        } else {
            internal_complexity
        }
    }
}

pub fn is_operator_greater_than(op1: OperationKind, op2: OperationKind) -> bool {
    match op1 {
        OperationKind::Add | OperationKind::Subtract => match op2 {
            OperationKind::Power | OperationKind::Multiply | OperationKind::Divide => true,
            _ => false,
        },
        OperationKind::Multiply | OperationKind::Divide => match op2 {
            OperationKind::Power => true,
            _ => false,
        },
        OperationKind::Power => false,
    }
}

pub fn reverse_operation(op: OperationKind) -> OperationKind {
    match op {
        OperationKind::Add => OperationKind::Subtract,
        OperationKind::Subtract => OperationKind::Add,
        OperationKind::Multiply => OperationKind::Divide,
        OperationKind::Divide => OperationKind::Multiply,
        OperationKind::Power => panic!("No reverse operation for Power"),
    }
}

pub fn are_operations_reverse(op1: OperationKind, op2: OperationKind) -> bool {
    match (op1, op2) {
        (OperationKind::Add, OperationKind::Subtract) => true,
        (OperationKind::Subtract, OperationKind::Add) => true,
        (OperationKind::Multiply, OperationKind::Divide) => true,
        (OperationKind::Divide, OperationKind::Multiply) => true,
        _ => false,
    }
}
