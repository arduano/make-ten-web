use std::{cmp::Ordering, ops::DerefMut};

use crate::maths::{
    expression::{EvaluatedExpr, Expression},
    operation::{are_operations_reverse, reverse_operation, OperationKind},
    Evaluate,
};

/// A function that simplifies the expression based on criteria. This helps eliminate solutions
/// that are too similar to each other, for example a + b is the same as b + a.
/// This function runs a single permutation of the shuffle, and returns a true if anything was changed.
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
        if parent_op.left.compare_shuffle_precidence(&parent_op.right) == Ordering::Less {
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
            && left_op.right.compare_shuffle_precidence(&parent_op.right) == Ordering::Less
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
            && left_op.right.compare_shuffle_precidence(&parent_op.right) == Ordering::Less
        {
            std::mem::swap(&mut left_op.right, &mut parent_op.right);

            changed = true;
            parent_op.re_evaluate();
        }
    }

    changed
}

/// Shuffle an expression until fully shuffled
pub fn fully_shuffle_expr(expression: &mut EvaluatedExpr) {
    loop {
        let shuffled = recursively_shuffle_expr(expression);

        if !shuffled {
            break;
        }
    }
}
