#![feature(generators)]

use std::{iter, rc::Rc};
use gen_iter::gen_iter;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub struct Baz {
    name: String,
}

#[wasm_bindgen]
impl Baz {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String) -> Self {
        Self { name }
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

#[wasm_bindgen]
pub fn fields_obj(name: Baz) -> Baz {
    greet(&name.name);

    Baz::new("cringe".to_string())
}

#[derive(Debug, Clone)]
enum Expression {
    Op(Rc<Operation>),
    Num(i32),
}

#[derive(Debug, Clone)]
struct EvaluatedExpr {
    value: i32,
    expression: Expression,
}

#[derive(Eq, PartialEq, Debug, Clone)]
enum OperationKind {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
}

#[derive(Debug, Clone)]
struct Operation {
    left: EvaluatedExpr,
    right: EvaluatedExpr,
    kind: OperationKind,
}

impl Expression {
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
            }
            OperationKind::Subtract => {
                if left_val < right_val {
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

        let expr = Expression::Op(Rc::new(Operation { left, right, kind }));

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
}

impl Operation {
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

        let same = self.left.equals(&other.left) && self.right.equals(&other.right);

        match self.kind {
            OperationKind::Add | OperationKind::Multiply => {
                let reverse = self.left.equals(&other.right) && self.right.equals(&other.left);
                return same || reverse;
            }
            _ => {
                return same;
            }
        }
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

    fn equals(&self, other: &EvaluatedExpr) -> bool {
        self.expression.equals(&other.expression)
    }
}

fn generate_expressions(inputs: &[i32]) -> impl Iterator<Item = EvaluatedExpr> {
    let operations = &[
        OperationKind::Add,
        OperationKind::Subtract,
        OperationKind::Multiply,
        OperationKind::Divide,
        OperationKind::Power,
    ];

    if inputs.len() == 1 {
        let iter = iter::once(Expression::new_num(inputs[0]));
        itertools::Either::Left(iter)
    } else {
        let iter = 1..(inputs.len());

        // gen_iter!(move {
        //     for i in  1..(inputs.len()) {
        //         yield i;
        //     }
        // });

        // // Map the indexes into sequences
        // let iter = iter.map(|i| {
        //     // Make the smaller sequence first
        //     if i < inputs.len() / 2 {
        //         (&inputs[0..i], &inputs[i..])
        //     } else {
        //         (&inputs[i..], &inputs[0..i])
        //     }
        // });

        // let iter = iter.map(|(left, right)| {
        //     let left_options_collected: Vec<_> = generate_expressions(left).collect();

        //     let right_options = generate_expressions(right);

        //     right_options.map(move |right| {
        //         let cloned_iter = left_options_collected.iter().cloned();
        //         cloned_iter.map(move |left| {
        //             operations
        //                 .iter()
        //                 .cloned()
        //                 .map(move |kind| Expression::new_op(left, right.clone(), kind))
        //         })
        //     })
        // });

        itertools::Either::Right(iter::once(Expression::new_num(inputs[0])))
    }
}
