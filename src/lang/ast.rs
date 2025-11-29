/// Abstract Syntax Tree for FieldCalc
use super::types::{Type, Nat};

/// Variable name
pub type Var = String;

/// Type variable
pub type TyVar = String;

/// Expressions
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    // ===== Lambda Calculus =====

    /// Variable: x
    Var(Var),

    /// Lambda abstraction: λ(x : τ). e
    Lambda {
        param: Var,
        param_ty: Type,
        body: Box<Expr>,
    },

    /// Application: e₁ e₂
    App {
        func: Box<Expr>,
        arg: Box<Expr>,
    },

    /// Type abstraction: Λα. e
    TyLambda {
        ty_var: TyVar,
        body: Box<Expr>,
    },

    /// Type application: e [τ]
    TyApp {
        expr: Box<Expr>,
        ty: Type,
    },

    /// Dimension abstraction: Λ(n : Nat). e
    DimLambda {
        dim_var: Var,
        body: Box<Expr>,
    },

    /// Dimension application: e [n]
    DimApp {
        expr: Box<Expr>,
        dim: Nat,
    },

    // ===== Literals =====

    /// Scalar literal: 3.14
    Scalar(f64),

    /// Boolean literal: true/false
    Bool(bool),

    /// Vector literal: ⟨e₁, ..., eₙ⟩
    Vec(Vec<Expr>),

    /// Color literal: rgb(r, g, b) or rgba(r, g, b, a)
    Color {
        r: Box<Expr>,
        g: Box<Expr>,
        b: Box<Expr>,
        a: Box<Expr>,
    },

    /// List literal: []
    Nil,

    /// List cons: e₁ :: e₂
    Cons {
        head: Box<Expr>,
        tail: Box<Expr>,
    },

    // ===== Field Construction =====

    /// Field literal: field (p : Vec n) ↦ e
    Field {
        param: Var,
        dim: Nat,
        body: Box<Expr>,
    },

    // ===== Let Binding =====

    /// Let binding: let x : τ = e₁ in e₂
    Let {
        var: Var,
        ty: Type,
        value: Box<Expr>,
        body: Box<Expr>,
    },

    // ===== Structural Recursion =====

    /// Fold: fold f z xs
    Fold {
        func: Box<Expr>,
        init: Box<Expr>,
        list: Box<Expr>,
    },

    // ===== Built-in Operations =====

    /// Built-in operation (referenced by name)
    Builtin(String),

    // ===== Conditionals =====

    /// If-then-else: if e₁ then e₂ else e₃
    If {
        cond: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Box<Expr>,
    },
}

impl Expr {
    /// Helper constructors

    pub fn var(name: impl Into<String>) -> Self {
        Expr::Var(name.into())
    }

    pub fn scalar(value: f64) -> Self {
        Expr::Scalar(value)
    }

    pub fn bool(value: bool) -> Self {
        Expr::Bool(value)
    }

    pub fn lambda(param: impl Into<String>, param_ty: Type, body: Expr) -> Self {
        Expr::Lambda {
            param: param.into(),
            param_ty,
            body: Box::new(body),
        }
    }

    pub fn app(func: Expr, arg: Expr) -> Self {
        Expr::App {
            func: Box::new(func),
            arg: Box::new(arg),
        }
    }

    pub fn let_bind(var: impl Into<String>, ty: Type, value: Expr, body: Expr) -> Self {
        Expr::Let {
            var: var.into(),
            ty,
            value: Box::new(value),
            body: Box::new(body),
        }
    }

    pub fn field(param: impl Into<String>, dim: Nat, body: Expr) -> Self {
        Expr::Field {
            param: param.into(),
            dim,
            body: Box::new(body),
        }
    }

    pub fn if_then_else(cond: Expr, then_branch: Expr, else_branch: Expr) -> Self {
        Expr::If {
            cond: Box::new(cond),
            then_branch: Box::new(then_branch),
            else_branch: Box::new(else_branch),
        }
    }

    pub fn nil() -> Self {
        Expr::Nil
    }

    pub fn cons(head: Expr, tail: Expr) -> Self {
        Expr::Cons {
            head: Box::new(head),
            tail: Box::new(tail),
        }
    }

    pub fn builtin(name: impl Into<String>) -> Self {
        Expr::Builtin(name.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expr_construction() {
        // λ(x : Scalar). x + 1.0
        let expr = Expr::lambda(
            "x",
            Type::Scalar,
            Expr::app(
                Expr::app(
                    Expr::builtin("+"),
                    Expr::var("x")
                ),
                Expr::scalar(1.0)
            )
        );

        match expr {
            Expr::Lambda { param, param_ty, .. } => {
                assert_eq!(param, "x");
                assert_eq!(param_ty, Type::Scalar);
            }
            _ => panic!("Expected lambda"),
        }
    }

    #[test]
    fn test_field_construction() {
        // field (p : Vec 2) ↦ length p
        let field_expr = Expr::field(
            "p",
            2,
            Expr::app(
                Expr::builtin("length"),
                Expr::var("p")
            )
        );

        match field_expr {
            Expr::Field { param, dim, .. } => {
                assert_eq!(param, "p");
                assert_eq!(dim, 2);
            }
            _ => panic!("Expected field"),
        }
    }
}
