/// Environment for evaluation
use std::collections::HashMap;
use super::ast::{Var, Expr};

/// Runtime values
#[derive(Debug, Clone)]
pub enum Value {
    /// Scalar value
    Scalar(f64),

    /// Boolean value
    Bool(bool),

    /// Vector value
    Vec(Vec<f64>),

    /// Color value (RGBA)
    Color(f64, f64, f64, f64),

    /// Function closure
    Closure {
        param: Var,
        body: Expr,
        env: Env,
    },

    /// Type-level closure
    TyClosure {
        ty_var: String,
        body: Expr,
        env: Env,
    },

    /// Dimension-level closure
    DimClosure {
        dim_var: String,
        body: Expr,
        env: Env,
    },

    /// Field closure (NOT evaluated until sampled!)
    Field {
        param: Var,
        dim: usize,
        body: Expr,
        env: Env,
    },

    /// List (nil or cons)
    Nil,
    Cons(Box<Value>, Box<Value>),

    /// Built-in function (stored as Rust function pointer)
    Builtin(fn(&[Value]) -> Result<Value, EvalError>),

    /// Partially applied built-in (for currying)
    PartialBuiltin {
        func: fn(&[Value]) -> Result<Value, EvalError>,
        args: Vec<Value>,
        arity: usize,
    },
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Scalar(a), Value::Scalar(b)) => (a - b).abs() < 1e-10,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Vec(a), Value::Vec(b)) => a == b,
            (Value::Color(r1, g1, b1, a1), Value::Color(r2, g2, b2, a2)) => {
                (r1 - r2).abs() < 1e-10
                    && (g1 - g2).abs() < 1e-10
                    && (b1 - b2).abs() < 1e-10
                    && (a1 - a2).abs() < 1e-10
            }
            (Value::Nil, Value::Nil) => true,
            (Value::Cons(h1, t1), Value::Cons(h2, t2)) => h1 == h2 && t1 == t2,
            // For closures and fields, we can't compare structurally
            // Just check if they're the same variant
            (Value::Closure { .. }, Value::Closure { .. }) => false,
            (Value::Field { .. }, Value::Field { .. }) => false,
            (Value::Builtin(_), Value::Builtin(_)) => false,
            (Value::PartialBuiltin { .. }, Value::PartialBuiltin { .. }) => false,
            _ => false,
        }
    }
}

/// Evaluation environment (maps variables to values)
#[derive(Debug, Clone, Default)]
pub struct Env {
    bindings: HashMap<Var, Value>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    pub fn extend(&self, var: Var, value: Value) -> Self {
        let mut new_env = self.clone();
        new_env.bindings.insert(var, value);
        new_env
    }

    pub fn lookup(&self, var: &str) -> Option<&Value> {
        self.bindings.get(var)
    }

    pub fn insert(&mut self, var: Var, value: Value) {
        self.bindings.insert(var, value);
    }
}

/// Evaluation errors
#[derive(Debug, Clone, PartialEq)]
pub enum EvalError {
    UnboundVariable(String),
    TypeMismatch(String),
    InvalidOperation(String),
    DivisionByZero,
    InvalidDimension(usize),
    EmptyList,
    NotAFunction,
    NotAField,
    ArityMismatch { expected: usize, got: usize },
}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalError::UnboundVariable(v) => write!(f, "Unbound variable: {}", v),
            EvalError::TypeMismatch(msg) => write!(f, "Type mismatch: {}", msg),
            EvalError::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
            EvalError::DivisionByZero => write!(f, "Division by zero"),
            EvalError::InvalidDimension(d) => write!(f, "Invalid dimension: {}", d),
            EvalError::EmptyList => write!(f, "Empty list"),
            EvalError::NotAFunction => write!(f, "Not a function"),
            EvalError::NotAField => write!(f, "Not a field"),
            EvalError::ArityMismatch { expected, got } => {
                write!(f, "Arity mismatch: expected {} args, got {}", expected, got)
            }
        }
    }
}

impl std::error::Error for EvalError {}
