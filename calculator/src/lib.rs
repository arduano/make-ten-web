#![feature(generators)]

use expression::{EvaluatedExpr, Expression};
use gen_iter::gen_iter;
use itertools::Itertools;
use operation::{are_operations_reverse, reverse_operation, OperationKind};
use std::{cmp::Ordering, ops::DerefMut};

use wasm_bindgen::prelude::*;

mod expression;
mod operation;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
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

/// A function that simplifies the expression based on criteria. This helps eliminate solutions
/// that are too similar to each other, for example a + b is the same as b + a.
fn recursively_shuffle_expr(expression: &mut EvaluatedExpr) -> bool {
    let mut changed = false;

    let parent_op = if let Expression::Op(op) = expression.deref_mut() {
        op
    } else {
        return false;
    };

    changed |= recursively_shuffle_expr(&mut parent_op.left);
    changed |= recursively_shuffle_expr(&mut parent_op.right);

    if let OperationKind::Add | OperationKind::Multiply = parent_op.kind {
        // Compare 2 operations inside the same expression
        // E.g. swap x and y in (x + y)
        if parent_op.left.compare_position(&parent_op.right) == Ordering::Less {
            std::mem::swap(&mut parent_op.left, &mut parent_op.right);

            changed = true;
        }
    }

    if let OperationKind::Add | OperationKind::Multiply = parent_op.kind {
        // Compare the right element of the internal expression with the external right element
        // As long as they are on the same order of operations with each other
        // E.g. convert ((a - x) + y) into ((a + y) - x)
        if let Expression::Op(left_op) = parent_op.left.deref_mut() {
            if are_operations_reverse(left_op.kind, parent_op.kind) {
                std::mem::swap(&mut left_op.right, &mut parent_op.right);
                std::mem::swap(&mut left_op.kind, &mut parent_op.kind);

                changed = true;
                parent_op.re_evaluate();
            }
        }
    }

    if let OperationKind::Add | OperationKind::Multiply = parent_op.kind {
        // Change the order of operations for reverse operations
        // E.g. convert (y + (a - x)) into ((y + a) - x))
        if let Expression::Op(right_op) = parent_op.right.deref_mut() {
            if are_operations_reverse(right_op.kind, parent_op.kind) {
                std::mem::swap(&mut right_op.right, &mut right_op.left);
                std::mem::swap(&mut right_op.left, &mut parent_op.left);
                std::mem::swap(&mut right_op.kind, &mut parent_op.kind);
                std::mem::swap(&mut parent_op.left, &mut parent_op.right);

                changed = true;
                parent_op.re_evaluate();
            }
        }
    }

    if let OperationKind::Subtract | OperationKind::Divide = parent_op.kind {
        if let Expression::Op(right_op) = parent_op.right.deref_mut() {
            // Unwrap right side addition/multiplication
            // E.g. (a - (b + c)) becomes ((a - c) - b)
            if are_operations_reverse(parent_op.kind, right_op.kind) {
                right_op.kind = parent_op.kind;
                std::mem::swap(&mut parent_op.left, &mut right_op.left);
                std::mem::swap(&mut parent_op.left, &mut parent_op.right);

                changed = true;
                parent_op.re_evaluate();
            }
        }
    }

    if let OperationKind::Subtract | OperationKind::Divide = parent_op.kind {
        if let Expression::Op(right_op) = parent_op.right.deref_mut() {
            // Unwrap right side subtraction/division
            // E.g. (a - (b - c)) becomes ((a + c) - b)
            if parent_op.kind == right_op.kind {
                right_op.kind = reverse_operation(parent_op.kind);
                std::mem::swap(&mut parent_op.left, &mut right_op.left);
                std::mem::swap(&mut parent_op.left, &mut parent_op.right);

                changed = true;
                parent_op.re_evaluate();
            }
        }
    }

    // Compare the right element of the internal expression with the external right element
    // Basically, compare x and y in ((a + x) + y) and swap if needed
    if let Expression::Op(left_op) = parent_op.left.deref_mut() {
        if left_op.kind == parent_op.kind
            && left_op.right.compare_position(&parent_op.right) == Ordering::Less
        {
            std::mem::swap(&mut left_op.right, &mut parent_op.right);

            changed = true;
            parent_op.re_evaluate();
        }
    }

    // Same as above, but check if the operations are reverse but x and y are equal
    // If they're equal, swap them according to precedence
    if let Expression::Op(left_op) = parent_op.left.deref_mut() {
        if are_operations_reverse(left_op.kind, parent_op.kind)
            && left_op.right.evaluate() == parent_op.right.evaluate()
            && left_op.right.compare_position(&parent_op.right) == Ordering::Less
        {
            std::mem::swap(&mut left_op.right, &mut parent_op.right);

            changed = true;
            parent_op.re_evaluate();
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
pub fn run(inputs: &[i32]) -> js_sys::Array {
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

    let tens_vec = tens_vec
        .into_iter()
        .map(|expr| (expr.get_complexity(), expr))
        .sorted_by(|a, b| a.0.cmp(&b.0))
        .map(|(_, expr)| expr);

    let tens = tens_vec.into_iter().map(|t| t.to_text());

    tens.map(|s| JsValue::from_str(&s)).collect()
}
