use crate::maths::{
    expression::{EvaluatedExpr, Expression},
    operation::OperationKind, Evaluate,
};
use gen_iter::gen_iter;

/// Recursively generate every possible expression in an interator.
/// Because this is an iterator, the whole set of all possible equations
/// isn't stored in memory at once, rather they're created on the go.
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
                // Make the smaller sequence be the collected one
                let (left, right) = if i < inputs.len() / 2 {
                    (&inputs[0..i], &inputs[i..])
                } else {
                    (&inputs[i..], &inputs[0..i])
                };

                // The left side, which will be looped over repeatedly (a whole loop for every right element),
                // which is why it needs to be a vec
                let left_options_collected: Vec<_> = generate_expressions(left).collect();

                // The right side, which will be looped over once
                let right_options = generate_expressions(right);

                // For each possible expression on the left, and each possible expression
                // on the right, and each possible operator generate and yield a new expression
                for right_expr in right_options {
                    for left_index in 0..left_options_collected.len() {
                        for operator in operations.iter().cloned() {
                            match operator {
                                OperationKind::Add | OperationKind::Multiply => {
                                    // Add and multiply don't depend on the orientation, so only one orientation is added
                                    let left_expr = &left_options_collected[left_index];
                                    yield Expression::new_op(left_expr.clone(), right_expr.clone(), operator);
                                }
                                _ => {
                                    // The other operators do depend on the orientation, so both orientations are added
                                    // (though only if the values aren't equal)
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

/// Generate every possible expression but filter out the ones that don't equal 10
pub fn get_tens<'a>(inputs: &'a [i32]) -> impl 'a + Iterator<Item = EvaluatedExpr> {
    generate_expressions(inputs).filter(|expr| expr.evaluate() == 10)
}
