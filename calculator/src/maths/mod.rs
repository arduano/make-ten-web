use self::operation::OperationKind;

pub mod expression;
pub mod operation;

// Below are traits for functionality that is shared between both expression and operation

pub trait Complexity {
    /// An aribtary recursive complexity metric that I came up with, where
    /// addition and subtraction are simple, multiplication and division are more complex
    /// and powers are the most complex.
    fn get_complexity(&self) -> u32;

    fn get_complexity_internal(&self, parent_op: OperationKind, is_left: bool) -> u32;
}

pub trait Evaluate {
    fn evaluate(&self) -> i32;
}

pub trait ExpressionEquals {
    /// Check if the expression (inner operations tree) equals another expression
    fn expr_equals(&self, other: &Self) -> bool;
}

pub trait Depth {
    /// Recursively get the depth of the expression
    fn depth(&self) -> usize;
}
