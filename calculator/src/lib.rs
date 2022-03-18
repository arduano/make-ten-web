#![feature(generators)]

use gen_iter::gen_iter;
use itertools::Itertools;
use std::{cmp::Ordering, hash::Hash};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Expression {
    Op(Box<Operation>),
    Num(i32),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct EvaluatedExpr {
    value: i32,
    expression: Expression,
}

#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
enum OperationKind {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Operation {
    left: EvaluatedExpr,
    right: EvaluatedExpr,
    kind: OperationKind,
}

impl Expression {
    fn to_text(&self) -> String {
        match self {
            Expression::Op(op) => op.to_text(),
            Expression::Num(num) => num.to_string(),
        }
    }

    fn to_text_child(&self, parent_op: OperationKind, is_left: bool) -> String {
        match self {
            Expression::Op(op) => op.to_text_child(parent_op, is_left),
            Expression::Num(num) => num.to_string(),
        }
    }

    fn new_num(num: i32) -> EvaluatedExpr {
        EvaluatedExpr::new(Expression::Num(num))
    }

    fn new_op(
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
            }
            _ => {}
        }

        let expr = Expression::Op(Box::new(Operation { left, right, kind }));

        Some(EvaluatedExpr::new(expr))
    }

    fn evaluate(&self) -> i32 {
        match self {
            Expression::Num(n) => *n,
            Expression::Op(op) => op.evaluate(),
        }
    }

    fn equals(&self, other: &Expression) -> bool {
        match self {
            Expression::Num(n) => match other {
                Expression::Num(m) => *n == *m,
                _ => false,
            },
            Expression::Op(op) => match other {
                Expression::Op(op2) => op.equals(op2),
                _ => false,
            },
        }
    }

    fn compare_position(&self, other: &Self) -> Ordering {
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

    fn depth(&self) -> usize {
        match self {
            Expression::Num(_) => 0,
            Expression::Op(op) => op.depth(),
        }
    }
}

impl Operation {
    fn to_text(&self) -> String {
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

    fn to_text_child(&self, parent_op: OperationKind, is_left: bool) -> String {
        let use_parenthises = is_operator_greater_than(self.kind, parent_op) || !is_left;

        if use_parenthises {
            format!("({})", self.to_text())
        } else {
            self.to_text()
        }
    }

    fn evaluate(&self) -> i32 {
        match self.kind {
            OperationKind::Add => self.left.evaluate() + self.right.evaluate(),
            OperationKind::Subtract => self.left.evaluate() - self.right.evaluate(),
            OperationKind::Multiply => self.left.evaluate() * self.right.evaluate(),
            OperationKind::Divide => self.left.evaluate() / self.right.evaluate(),
            OperationKind::Power => self.left.evaluate().pow(self.right.evaluate() as u32),
        }
    }

    fn equals(&self, other: &Operation) -> bool {
        if self.kind != other.kind {
            return false;
        }

        let mut same = self.left.equals(&other.left) && self.right.equals(&other.right);

        // Reverse addition/multiplication are equal
        match self.kind {
            OperationKind::Add | OperationKind::Multiply => {
                same |= self.left.equals(&other.right) && self.right.equals(&other.left);
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

    fn depth(&self) -> usize {
        let left_depth = self.left.expression.depth();
        let right_depth = self.right.expression.depth();

        if left_depth > right_depth {
            left_depth
        } else {
            right_depth
        }
    }
}

impl std::ops::Deref for EvaluatedExpr {
    type Target = Expression;

    fn deref(&self) -> &Self::Target {
        &self.expression
    }
}

impl EvaluatedExpr {
    fn new(expression: Expression) -> EvaluatedExpr {
        EvaluatedExpr {
            value: expression.evaluate(),
            expression,
        }
    }

    fn evaluate(&self) -> i32 {
        self.value
    }
}

fn generate_expressions<'a>(inputs: &'a [i32]) -> Box<dyn 'a + Iterator<Item = EvaluatedExpr>> {
    let operations = &[
        OperationKind::Add,
        OperationKind::Subtract,
        OperationKind::Multiply,
        OperationKind::Divide,
        OperationKind::Power,
    ];

    let iter = gen_iter!(move {
        if inputs.len() == 1 {
            yield Some(Expression::new_num(inputs[0]));
        } else {

            for i in  1..(inputs.len()) {
                // Make the smaller sequence first
                let (left, right) = if i < inputs.len() / 2 {
                    (&inputs[0..i], &inputs[i..])
                } else {
                    (&inputs[i..], &inputs[0..i])
                };

                let left_options_collected: Vec<_> = generate_expressions(left).collect();

                let right_options = generate_expressions(right);

                for right_expr in right_options {
                    for left_index in 0..left_options_collected.len() {
                        for operator in operations.iter().cloned() {
                            match operator {
                                OperationKind::Add | OperationKind::Multiply => {
                                    let left_expr = &left_options_collected[left_index];
                                    yield Expression::new_op(left_expr.clone(), right_expr.clone(), operator);
                                }
                                _ => {
                                    let left_expr = &left_options_collected[left_index];
                                    yield Expression::new_op(left_expr.clone(), right_expr.clone(), operator);

                                    let left_expr = &left_options_collected[left_index];
                                    if left_expr.evaluate() != right_expr.evaluate(){
                                        yield Expression::new_op(right_expr.clone(), left_expr.clone(), operator);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    });

    Box::new(iter.flatten())
}

fn get_tens<'a>(inputs: &'a [i32]) -> impl 'a + Iterator<Item = EvaluatedExpr> {
    generate_expressions(inputs).filter(|expr| expr.evaluate() == 10)
}

fn is_operator_greater_than(op1: OperationKind, op2: OperationKind) -> bool {
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

fn is_operation_negative(op: OperationKind) -> bool {
    match op {
        OperationKind::Subtract | OperationKind::Divide => true,
        _ => false,
    }
}

fn reverse_operation(op: OperationKind) -> OperationKind {
    match op {
        OperationKind::Add => OperationKind::Subtract,
        OperationKind::Subtract => OperationKind::Add,
        OperationKind::Multiply => OperationKind::Divide,
        OperationKind::Divide => OperationKind::Multiply,
        OperationKind::Power => OperationKind::Power,
    }
}

fn are_operations_reverse(op1: OperationKind, op2: OperationKind) -> bool {
    match (op1, op2) {
        (OperationKind::Add, OperationKind::Subtract) => true,
        (OperationKind::Subtract, OperationKind::Add) => true,
        (OperationKind::Multiply, OperationKind::Divide) => true,
        (OperationKind::Divide, OperationKind::Multiply) => true,
        _ => false,
    }
}

fn recursively_shuffle_expr(expression: &mut EvaluatedExpr) -> bool {
    let mut changed = false;

    let operation = if let Expression::Op(op) = &mut expression.expression {
        op
    } else {
        return false;
    };

    changed |= recursively_shuffle_expr(&mut operation.left);
    changed |= recursively_shuffle_expr(&mut operation.right);

    // Fixme: remove
    let operation = if let Expression::Op(op) = &mut expression.expression {
        op
    } else {
        return false;
    };

    match operation.kind {
        OperationKind::Add | OperationKind::Multiply => {
            // Compare 2 operations inside the same expression
            if operation.left.compare_position(&operation.right) == Ordering::Less {
                std::mem::swap(&mut operation.left, &mut operation.right);
                changed = true;
            }

            // Compare the right element of the internal expression with the external right element
            // As long as they are on the same order of operations with each other
            // E.g. convert ((a - x) + y) into ((a + y) - x)
            if let Expression::Op(op) = &mut operation.left.expression {
                if are_operations_reverse(op.kind, operation.kind) && is_operation_negative(op.kind)
                {
                    std::mem::swap(&mut op.right, &mut operation.right);
                    std::mem::swap(&mut op.kind, &mut operation.kind);

                    changed = true;
                }
            }
        }
        OperationKind::Subtract | OperationKind::Divide => {
            if let Expression::Op(right_op) = &mut operation.right.expression {
                // Unwrap right side addition/multiplication
                // E.g. (a - (b + c)) becomes ((a - c) - b)
                if are_operations_reverse(operation.kind, right_op.kind) {
                    right_op.kind = operation.kind;
                    std::mem::swap(&mut operation.left, &mut right_op.left);
                    std::mem::swap(&mut operation.left, &mut operation.right);
                    changed = true;
                }
            }

            if let Expression::Op(right_op) = &mut operation.right.expression {
                // Unwrap right side subtraction/division
                // E.g. (a - (b - c)) becomes ((a + c) - b)
                if operation.kind == right_op.kind {
                    right_op.kind = reverse_operation(operation.kind);
                    std::mem::swap(&mut operation.left, &mut right_op.left);
                    std::mem::swap(&mut operation.left, &mut operation.right);
                    changed = true;
                }
            }
        }
        _ => {}
    }

    // Compare the right element of the internal expression with the external right element
    // Basically, compare x and y in ((a + x) + y) and swap if needed
    if let Expression::Op(op) = &mut operation.left.expression {
        if op.kind == operation.kind
            && op.right.compare_position(&operation.right) == Ordering::Less
        {
            std::mem::swap(&mut op.right, &mut operation.right);
            changed = true;
        }
    }

    // Same as above, but check if the operations are reverse but x and y are equal
    // If they're equal, swap them according to precedence
    if let Expression::Op(op) = &mut operation.left.expression {
        if are_operations_reverse(op.kind, operation.kind)
            && op.right.evaluate() == operation.right.evaluate()
            && op.right.compare_position(&operation.right) == Ordering::Less
        {
            std::mem::swap(&mut op.right, &mut operation.right);
            changed = true;
        }
    }

    changed
}

fn fully_shuffle_expr(expression: &mut EvaluatedExpr, print: bool) {
    if print {
        log(&format!("initial: {}", expression.to_text()));
    }
    loop {
        let shuffled = recursively_shuffle_expr(expression);

        if !shuffled {
            break;
        }
    }
}

#[wasm_bindgen]
pub fn run() {
    let inputs = &[6, 2, 5, 6];

    let tens = get_tens(inputs).map(|mut e| {
        fully_shuffle_expr(&mut e, false);
        e
    });

    let mut tens_vec: Vec<EvaluatedExpr> = Vec::new();

    for ten in tens {
        if tens_vec.iter().any(|t| t.equals(&ten)) {
            continue;
        }
        tens_vec.push(ten);
    }

    let tens = tens_vec
        .into_iter()
        .map(|t| t.expression.to_text())
        .sorted();

    for ten in tens {
        log(&ten);
    }
}
