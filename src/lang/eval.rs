/// Reference evaluator for FieldCalc (strict evaluation)
use super::ast::Expr;
use super::env::{Env, Value, EvalError};
use super::builtins;

/// Evaluate an expression in an environment
pub fn eval(expr: &Expr, env: &Env) -> Result<Value, EvalError> {
    match expr {
        // ===== Variables =====
        Expr::Var(name) => env
            .lookup(name)
            .cloned()
            .ok_or_else(|| EvalError::UnboundVariable(name.clone())),

        // ===== Literals =====
        Expr::Scalar(v) => Ok(Value::Scalar(*v)),

        Expr::Bool(b) => Ok(Value::Bool(*b)),

        Expr::Vec(components) => {
            let values: Result<Vec<f64>, EvalError> = components
                .iter()
                .map(|e| {
                    match eval(e, env)? {
                        Value::Scalar(v) => Ok(v),
                        _ => Err(EvalError::TypeMismatch(
                            "Vector components must be scalars".to_string()
                        )),
                    }
                })
                .collect();
            Ok(Value::Vec(values?))
        }

        Expr::Color { r, g, b, a } => {
            let r_val = expect_scalar(&eval(r, env)?)?;
            let g_val = expect_scalar(&eval(g, env)?)?;
            let b_val = expect_scalar(&eval(b, env)?)?;
            let a_val = expect_scalar(&eval(a, env)?)?;
            Ok(Value::Color(r_val, g_val, b_val, a_val))
        }

        Expr::Nil => Ok(Value::Nil),

        Expr::Cons { head, tail } => {
            let h = eval(head, env)?;
            let t = eval(tail, env)?;
            Ok(Value::Cons(Box::new(h), Box::new(t)))
        }

        // ===== Lambda Calculus =====
        Expr::Lambda { param, body, .. } => Ok(Value::Closure {
            param: param.clone(),
            body: (**body).clone(),
            env: env.clone(),
        }),

        Expr::App { func, arg } => {
            let func_val = eval(func, env)?;
            let arg_val = eval(arg, env)?;
            apply(func_val, arg_val)
        }

        Expr::TyLambda { ty_var, body } => Ok(Value::TyClosure {
            ty_var: ty_var.clone(),
            body: (**body).clone(),
            env: env.clone(),
        }),

        Expr::TyApp { expr, .. } => {
            // For now, we erase types at runtime
            // Just evaluate the expression
            eval(expr, env)
        }

        Expr::DimLambda { dim_var, body } => Ok(Value::DimClosure {
            dim_var: dim_var.clone(),
            body: (**body).clone(),
            env: env.clone(),
        }),

        Expr::DimApp { expr, .. } => {
            // For now, we erase dimensions at runtime
            eval(expr, env)
        }

        // ===== Field Construction =====
        // KEY: Fields are NOT evaluated! They remain as closures.
        Expr::Field { param, dim, body } => Ok(Value::Field {
            param: param.clone(),
            dim: *dim,
            body: (**body).clone(),
            env: env.clone(),
        }),

        // ===== Let Binding =====
        Expr::Let { var, value, body, .. } => {
            let val = eval(value, env)?;
            let new_env = env.extend(var.clone(), val);
            eval(body, &new_env)
        }

        // ===== Structural Recursion =====
        Expr::Fold { func, init, list } => {
            let f = eval(func, env)?;
            let z = eval(init, env)?;
            let xs = eval(list, env)?;
            fold_list(f, z, xs)
        }

        // ===== Built-ins =====
        Expr::Builtin(name) => builtins::lookup_builtin(name)
            .ok_or_else(|| EvalError::UnboundVariable(format!("builtin: {}", name))),

        // ===== Conditionals =====
        Expr::If { cond, then_branch, else_branch } => {
            let cond_val = eval(cond, env)?;
            match cond_val {
                Value::Bool(true) => eval(then_branch, env),
                Value::Bool(false) => eval(else_branch, env),
                _ => Err(EvalError::TypeMismatch("Condition must be boolean".to_string())),
            }
        }
    }
}

/// Apply a function to an argument
fn apply(func: Value, arg: Value) -> Result<Value, EvalError> {
    match func {
        Value::Closure { param, body, env } => {
            let new_env = env.extend(param, arg);
            eval(&body, &new_env)
        }
        Value::Builtin(f) => {
            // Start partial application with arity detection
            // Most builtins are binary (arity 2), some are unary (arity 1)
            let arity = builtins::get_arity(f);
            if arity == 1 {
                f(&[arg])
            } else {
                Ok(Value::PartialBuiltin {
                    func: f,
                    args: vec![arg],
                    arity,
                })
            }
        }
        Value::PartialBuiltin { func, mut args, arity } => {
            args.push(arg);
            if args.len() == arity {
                // Fully applied - call the function
                func(&args)
            } else {
                // Still partial
                Ok(Value::PartialBuiltin { func, args, arity })
            }
        }
        _ => Err(EvalError::NotAFunction),
    }
}

/// Sample a field at a point
pub fn sample_field(field: Value, point: Value) -> Result<Value, EvalError> {
    match field {
        Value::Field { param, dim, body, env } => {
            // Verify point has correct dimension
            let point_vec = match point {
                Value::Vec(v) if v.len() == dim => v,
                Value::Vec(v) => {
                    return Err(EvalError::InvalidDimension(v.len()));
                }
                _ => {
                    return Err(EvalError::TypeMismatch(
                        "Field must be sampled at a vector".to_string()
                    ));
                }
            };

            // Evaluate the body with the point bound
            let new_env = env.extend(param, Value::Vec(point_vec));
            eval(&body, &new_env)
        }
        _ => Err(EvalError::NotAField),
    }
}

/// Fold over a list
fn fold_list(f: Value, z: Value, xs: Value) -> Result<Value, EvalError> {
    match xs {
        Value::Nil => Ok(z),
        Value::Cons(head, tail) => {
            let rest = fold_list(f.clone(), z, *tail)?;
            // Apply f to head and rest: f head rest
            let partial = apply(f, *head)?;
            apply(partial, rest)
        }
        _ => Err(EvalError::TypeMismatch("fold expects a list".to_string())),
    }
}

// Helper functions

fn expect_scalar(val: &Value) -> Result<f64, EvalError> {
    match val {
        Value::Scalar(v) => Ok(*v),
        _ => Err(EvalError::TypeMismatch("Expected scalar".to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lang::types::Type;
    use crate::lang::ast::Expr;

    #[test]
    fn test_eval_scalar() {
        let env = Env::new();
        let expr = Expr::scalar(42.0);
        let result = eval(&expr, &env).unwrap();
        assert_eq!(result, Value::Scalar(42.0));
    }

    #[test]
    fn test_eval_vector() {
        let env = Env::new();
        let expr = Expr::Vec(vec![Expr::scalar(1.0), Expr::scalar(2.0)]);
        let result = eval(&expr, &env).unwrap();
        assert_eq!(result, Value::Vec(vec![1.0, 2.0]));
    }

    #[test]
    fn test_eval_lambda() {
        let env = Env::new();
        // λ(x : Scalar). x
        let expr = Expr::lambda("x", Type::Scalar, Expr::var("x"));
        let result = eval(&expr, &env).unwrap();

        match result {
            Value::Closure { param, .. } => {
                assert_eq!(param, "x");
            }
            _ => panic!("Expected closure"),
        }
    }

    #[test]
    fn test_eval_application() {
        // Create environment with a simple addition function
        let mut env = Env::new();

        // Test: (λx. x) 42
        let identity = Expr::lambda("x", Type::Scalar, Expr::var("x"));
        let app = Expr::app(identity, Expr::scalar(42.0));

        let result = eval(&app, &env).unwrap();
        assert_eq!(result, Value::Scalar(42.0));
    }

    #[test]
    fn test_eval_let() {
        let env = Env::new();
        // let x = 5.0 in x
        let expr = Expr::let_bind(
            "x",
            Type::Scalar,
            Expr::scalar(5.0),
            Expr::var("x"),
        );

        let result = eval(&expr, &env).unwrap();
        assert_eq!(result, Value::Scalar(5.0));
    }
}
