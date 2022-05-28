#![feature(generators)]

use generate::get_tens;
use itertools::Itertools;
use maths::{expression::EvaluatedExpr, ExpressionEquals, Complexity};

use shuffle::fully_shuffle_expr;

use wasm_bindgen::prelude::*;

mod generate;
mod maths;
mod shuffle;

/// A function (callable from js) that takes an aray of numbers and returns
/// an array of strings for all the possible solutions
#[wasm_bindgen]
pub fn generate_solutions(inputs: &[i32]) -> js_sys::Array {
    // Get all the possible expressions that add to ten then map them to be shuffled
    let tens = get_tens(inputs).map(|mut e| {
        fully_shuffle_expr(&mut e);
        e
    });

    let mut tens_vec: Vec<EvaluatedExpr> = Vec::new();

    // Push all expressions into an array, except remove duplicates based on equality
    for ten in tens {
        if tens_vec.iter().any(|t| t.expr_equals(&ten)) {
            continue;
        }
        tens_vec.push(ten);
    }

    // Sort by complexity
    let tens_vec = tens_vec
        .into_iter()
        .map(|expr| (expr.get_complexity(), expr))
        .sorted_by(|a, b| a.0.cmp(&b.0))
        .map(|(_, expr)| expr);

    // Map all expressions to text
    let tens = tens_vec.into_iter().map(|t| t.to_text());

    // Map all strings to JsValue to pass back to javascript
    tens.map(|s| JsValue::from_str(&s)).collect()
}
