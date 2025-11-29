/// Built-in functions for FieldCalc
use super::env::{Value, EvalError};
use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Lookup a built-in function by name
pub fn lookup_builtin(name: &str) -> Option<Value> {
    BUILTINS.get(name).copied().map(Value::Builtin)
}

/// Get the arity of a built-in function
pub fn get_arity(f: fn(&[Value]) -> Result<Value, EvalError>) -> usize {
    // Match against known functions to determine arity
    // This is a bit hacky but works for our purposes
    if f as usize == builtin_add as usize
        || f as usize == builtin_sub as usize
        || f as usize == builtin_mul as usize
        || f as usize == builtin_div as usize
        || f as usize == builtin_min as usize
        || f as usize == builtin_max as usize
        || f as usize == builtin_lt as usize
        || f as usize == builtin_gt as usize
        || f as usize == builtin_eq as usize
        || f as usize == builtin_dot as usize
        || f as usize == builtin_vec_add as usize
        || f as usize == builtin_vec_sub as usize
        || f as usize == builtin_vec_scale as usize
    {
        2
    } else {
        1
    }
}

/// Registry of all built-in functions
static BUILTINS: Lazy<HashMap<&'static str, fn(&[Value]) -> Result<Value, EvalError>>> =
    Lazy::new(|| {
        let mut map = HashMap::new();

        // Scalar operations
        map.insert("+", builtin_add as fn(&[Value]) -> Result<Value, EvalError>);
        map.insert("-", builtin_sub as fn(&[Value]) -> Result<Value, EvalError>);
        map.insert("*", builtin_mul as fn(&[Value]) -> Result<Value, EvalError>);
        map.insert("/", builtin_div as fn(&[Value]) -> Result<Value, EvalError>);
        map.insert("min", builtin_min as fn(&[Value]) -> Result<Value, EvalError>);
        map.insert("max", builtin_max as fn(&[Value]) -> Result<Value, EvalError>);
        map.insert("abs", builtin_abs as fn(&[Value]) -> Result<Value, EvalError>);
        map.insert("sin", builtin_sin as fn(&[Value]) -> Result<Value, EvalError>);
        map.insert("cos", builtin_cos as fn(&[Value]) -> Result<Value, EvalError>);
        map.insert("sqrt", builtin_sqrt as fn(&[Value]) -> Result<Value, EvalError>);
        map.insert("negate", builtin_negate as fn(&[Value]) -> Result<Value, EvalError>);

        // Comparison
        map.insert("<", builtin_lt as fn(&[Value]) -> Result<Value, EvalError>);
        map.insert(">", builtin_gt as fn(&[Value]) -> Result<Value, EvalError>);
        map.insert("==", builtin_eq as fn(&[Value]) -> Result<Value, EvalError>);

        // Vector operations
        map.insert("length", builtin_length as fn(&[Value]) -> Result<Value, EvalError>);
        map.insert("dot", builtin_dot as fn(&[Value]) -> Result<Value, EvalError>);
        map.insert("normalize", builtin_normalize as fn(&[Value]) -> Result<Value, EvalError>);
        map.insert("vec_add", builtin_vec_add as fn(&[Value]) -> Result<Value, EvalError>);
        map.insert("vec_sub", builtin_vec_sub as fn(&[Value]) -> Result<Value, EvalError>);
        map.insert("vec_scale", builtin_vec_scale as fn(&[Value]) -> Result<Value, EvalError>);

        map
    });

// ===== Scalar Operations =====

fn builtin_add(args: &[Value]) -> Result<Value, EvalError> {
    check_arity(args, 2)?;
    let a = expect_scalar(&args[0])?;
    let b = expect_scalar(&args[1])?;
    Ok(Value::Scalar(a + b))
}

fn builtin_sub(args: &[Value]) -> Result<Value, EvalError> {
    check_arity(args, 2)?;
    let a = expect_scalar(&args[0])?;
    let b = expect_scalar(&args[1])?;
    Ok(Value::Scalar(a - b))
}

fn builtin_mul(args: &[Value]) -> Result<Value, EvalError> {
    check_arity(args, 2)?;
    let a = expect_scalar(&args[0])?;
    let b = expect_scalar(&args[1])?;
    Ok(Value::Scalar(a * b))
}

fn builtin_div(args: &[Value]) -> Result<Value, EvalError> {
    check_arity(args, 2)?;
    let a = expect_scalar(&args[0])?;
    let b = expect_scalar(&args[1])?;
    if b.abs() < 1e-10 {
        return Err(EvalError::DivisionByZero);
    }
    Ok(Value::Scalar(a / b))
}

fn builtin_min(args: &[Value]) -> Result<Value, EvalError> {
    check_arity(args, 2)?;
    let a = expect_scalar(&args[0])?;
    let b = expect_scalar(&args[1])?;
    Ok(Value::Scalar(a.min(b)))
}

fn builtin_max(args: &[Value]) -> Result<Value, EvalError> {
    check_arity(args, 2)?;
    let a = expect_scalar(&args[0])?;
    let b = expect_scalar(&args[1])?;
    Ok(Value::Scalar(a.max(b)))
}

fn builtin_abs(args: &[Value]) -> Result<Value, EvalError> {
    check_arity(args, 1)?;
    let a = expect_scalar(&args[0])?;
    Ok(Value::Scalar(a.abs()))
}

fn builtin_sin(args: &[Value]) -> Result<Value, EvalError> {
    check_arity(args, 1)?;
    let a = expect_scalar(&args[0])?;
    Ok(Value::Scalar(a.sin()))
}

fn builtin_cos(args: &[Value]) -> Result<Value, EvalError> {
    check_arity(args, 1)?;
    let a = expect_scalar(&args[0])?;
    Ok(Value::Scalar(a.cos()))
}

fn builtin_sqrt(args: &[Value]) -> Result<Value, EvalError> {
    check_arity(args, 1)?;
    let a = expect_scalar(&args[0])?;
    Ok(Value::Scalar(a.sqrt()))
}

fn builtin_negate(args: &[Value]) -> Result<Value, EvalError> {
    check_arity(args, 1)?;
    let a = expect_scalar(&args[0])?;
    Ok(Value::Scalar(-a))
}

// ===== Comparison =====

fn builtin_lt(args: &[Value]) -> Result<Value, EvalError> {
    check_arity(args, 2)?;
    let a = expect_scalar(&args[0])?;
    let b = expect_scalar(&args[1])?;
    Ok(Value::Bool(a < b))
}

fn builtin_gt(args: &[Value]) -> Result<Value, EvalError> {
    check_arity(args, 2)?;
    let a = expect_scalar(&args[0])?;
    let b = expect_scalar(&args[1])?;
    Ok(Value::Bool(a > b))
}

fn builtin_eq(args: &[Value]) -> Result<Value, EvalError> {
    check_arity(args, 2)?;
    let a = expect_scalar(&args[0])?;
    let b = expect_scalar(&args[1])?;
    Ok(Value::Bool((a - b).abs() < 1e-10))
}

// ===== Vector Operations =====

fn builtin_length(args: &[Value]) -> Result<Value, EvalError> {
    check_arity(args, 1)?;
    let v = expect_vec(&args[0])?;
    let len_sq: f64 = v.iter().map(|x| x * x).sum();
    Ok(Value::Scalar(len_sq.sqrt()))
}

fn builtin_dot(args: &[Value]) -> Result<Value, EvalError> {
    check_arity(args, 2)?;
    let a = expect_vec(&args[0])?;
    let b = expect_vec(&args[1])?;

    if a.len() != b.len() {
        return Err(EvalError::InvalidDimension(a.len()));
    }

    let result: f64 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    Ok(Value::Scalar(result))
}

fn builtin_normalize(args: &[Value]) -> Result<Value, EvalError> {
    check_arity(args, 1)?;
    let v = expect_vec(&args[0])?;
    let len_sq: f64 = v.iter().map(|x| x * x).sum();
    let len = len_sq.sqrt();

    if len < 1e-10 {
        return Err(EvalError::InvalidOperation("Cannot normalize zero vector".to_string()));
    }

    let normalized: Vec<f64> = v.iter().map(|x| x / len).collect();
    Ok(Value::Vec(normalized))
}

fn builtin_vec_add(args: &[Value]) -> Result<Value, EvalError> {
    check_arity(args, 2)?;
    let a = expect_vec(&args[0])?;
    let b = expect_vec(&args[1])?;

    if a.len() != b.len() {
        return Err(EvalError::InvalidDimension(a.len()));
    }

    let result: Vec<f64> = a.iter().zip(b.iter()).map(|(x, y)| x + y).collect();
    Ok(Value::Vec(result))
}

fn builtin_vec_sub(args: &[Value]) -> Result<Value, EvalError> {
    check_arity(args, 2)?;
    let a = expect_vec(&args[0])?;
    let b = expect_vec(&args[1])?;

    if a.len() != b.len() {
        return Err(EvalError::InvalidDimension(a.len()));
    }

    let result: Vec<f64> = a.iter().zip(b.iter()).map(|(x, y)| x - y).collect();
    Ok(Value::Vec(result))
}

fn builtin_vec_scale(args: &[Value]) -> Result<Value, EvalError> {
    check_arity(args, 2)?;
    let s = expect_scalar(&args[0])?;
    let v = expect_vec(&args[1])?;

    let result: Vec<f64> = v.iter().map(|x| s * x).collect();
    Ok(Value::Vec(result))
}

// ===== Helper Functions =====

fn check_arity(args: &[Value], expected: usize) -> Result<(), EvalError> {
    if args.len() != expected {
        Err(EvalError::ArityMismatch {
            expected,
            got: args.len(),
        })
    } else {
        Ok(())
    }
}

fn expect_scalar(val: &Value) -> Result<f64, EvalError> {
    match val {
        Value::Scalar(v) => Ok(*v),
        _ => Err(EvalError::TypeMismatch("Expected scalar".to_string())),
    }
}

fn expect_vec(val: &Value) -> Result<Vec<f64>, EvalError> {
    match val {
        Value::Vec(v) => Ok(v.clone()),
        _ => Err(EvalError::TypeMismatch("Expected vector".to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builtin_add() {
        let result = builtin_add(&[Value::Scalar(2.0), Value::Scalar(3.0)]).unwrap();
        assert_eq!(result, Value::Scalar(5.0));
    }

    #[test]
    fn test_builtin_length() {
        let v = Value::Vec(vec![3.0, 4.0]);
        let result = builtin_length(&[v]).unwrap();
        assert_eq!(result, Value::Scalar(5.0));
    }

    #[test]
    fn test_builtin_dot() {
        let a = Value::Vec(vec![1.0, 2.0, 3.0]);
        let b = Value::Vec(vec![4.0, 5.0, 6.0]);
        let result = builtin_dot(&[a, b]).unwrap();
        assert_eq!(result, Value::Scalar(32.0)); // 1*4 + 2*5 + 3*6 = 32
    }
}
