/// Type system for FieldCalc
use std::fmt;

/// Natural numbers (for dimensions)
pub type Nat = usize;

/// Kinds
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Kind {
    /// Kind of types (*)
    Star,
    /// Kind of natural numbers
    Nat,
    /// Higher-kinded type constructor (κ₁ → κ₂)
    Arrow(Box<Kind>, Box<Kind>),
}

/// Types
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    /// Type variable (α)
    Var(String),

    /// Dimension variable (n : Nat)
    DimVar(String),

    /// Base types
    Scalar,
    Vec(Nat),
    Color,

    /// Field type: Vec n → α
    Field(Nat, Box<Type>),

    /// Function type: τ₁ → τ₂
    Arrow(Box<Type>, Box<Type>),

    /// Universal quantification: ∀α. τ
    Forall(String, Box<Type>),

    /// Dimension polymorphism: ∀(n : Nat). τ
    ForallDim(String, Box<Type>),

    /// List type: List α
    List(Box<Type>),

    /// Boolean (for internal use)
    Bool,
}

impl Type {
    /// Type aliases for common types
    pub fn sdf(dim: Nat) -> Type {
        Type::Field(dim, Box::new(Type::Scalar))
    }

    pub fn image(dim: Nat) -> Type {
        Type::Field(dim, Box::new(Type::Color))
    }

    pub fn vfield(dim: Nat) -> Type {
        Type::Field(dim, Box::new(Type::Vec(dim)))
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Var(name) => write!(f, "{}", name),
            Type::DimVar(name) => write!(f, "{}", name),
            Type::Scalar => write!(f, "Scalar"),
            Type::Vec(n) => write!(f, "Vec {}", n),
            Type::Color => write!(f, "Color"),
            Type::Field(n, t) => write!(f, "Field {} ({})", n, t),
            Type::Arrow(t1, t2) => write!(f, "({} → {})", t1, t2),
            Type::Forall(v, t) => write!(f, "∀{}. {}", v, t),
            Type::ForallDim(v, t) => write!(f, "∀({} : Nat). {}", v, t),
            Type::List(t) => write!(f, "List ({})", t),
            Type::Bool => write!(f, "Bool"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_aliases() {
        let sdf2 = Type::sdf(2);
        assert_eq!(sdf2, Type::Field(2, Box::new(Type::Scalar)));

        let img3 = Type::image(3);
        assert_eq!(img3, Type::Field(3, Box::new(Type::Color)));
    }
}
