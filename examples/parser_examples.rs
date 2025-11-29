/// Examples using the FieldCalc parser
use solid_modeling::lang::*;

fn main() {
    println!("=== FieldCalc Parser Examples ===\n");

    example_1_simple();
    example_2_lambda();
    example_3_field();
    example_4_circle();
    example_5_let_binding();
    example_6_comments();
}

fn run_example(name: &str, code: &str) {
    println!("Example: {}", name);
    println!("Code:\n{}", code);

    match parse_program(code) {
        Ok(expr) => {
            let env = Env::with_builtins();
            match eval(&expr, &env) {
                Ok(value) => println!("Result: {:?}\n", value),
                Err(e) => println!("Evaluation error: {}\n", e),
            }
        }
        Err(e) => println!("Parse error: {}\n", e),
    }
}

fn example_1_simple() {
    run_example(
        "Simple Arithmetic",
        "(+ 2.0 3.0)",
    );
}

fn example_2_lambda() {
    run_example(
        "Lambda Function",
        r#"
        ; Identity function applied to 42
        ((fn (x Scalar) x) 42.0)
        "#,
    );
}

fn example_3_field() {
    println!("Example: Field Construction and Sampling");

    let code = r#"
        ; Create a field: distance from origin
        (field (p 2) (length p))
    "#;

    println!("Code:\n{}", code);

    match parse_program(code) {
        Ok(expr) => {
            let env = Env::with_builtins();
            match eval(&expr, &env) {
                Ok(field_value) => {
                    println!("Field created successfully");

                    // Sample at (3.0, 4.0) - should be 5.0
                    let point = Value::Vec(vec![3.0, 4.0]);
                    match sample_field(field_value, point) {
                        Ok(Value::Scalar(d)) => {
                            println!("Sampled at (3.0, 4.0): {}\n", d);
                        }
                        Ok(v) => println!("Unexpected value: {:?}\n", v),
                        Err(e) => println!("Sampling error: {}\n", e),
                    }
                }
                Err(e) => println!("Evaluation error: {}\n", e),
            }
        }
        Err(e) => println!("Parse error: {}\n", e),
    }
}

fn example_4_circle() {
    println!("Example: Circle SDF");

    let code = r#"
        ; Circle with radius 2.0
        (fn (r Scalar)
            (field (p 2)
                (- (length p) r)))
    "#;

    println!("Code:\n{}", code);

    match parse_program(code) {
        Ok(expr) => {
            let env = Env::with_builtins();
            match eval(&expr, &env) {
                Ok(circle_func) => {
                    // Apply to radius 2.0
                    match apply_value(circle_func, Value::Scalar(2.0)) {
                        Ok(field_value) => {
                            println!("Circle SDF created");

                            // Test points
                            let test_points = vec![
                                (0.0, 0.0, "center (inside)"),
                                (2.0, 0.0, "boundary"),
                                (3.0, 0.0, "outside"),
                            ];

                            for (x, y, desc) in test_points {
                                let point = Value::Vec(vec![x, y]);
                                if let Ok(Value::Scalar(d)) = sample_field(field_value.clone(), point) {
                                    println!("  ({}, {}): {:.3} - {}", x, y, d, desc);
                                }
                            }
                            println!();
                        }
                        Err(e) => println!("Application error: {}\n", e),
                    }
                }
                Err(e) => println!("Evaluation error: {}\n", e),
            }
        }
        Err(e) => println!("Parse error: {}\n", e),
    }
}

fn example_5_let_binding() {
    run_example(
        "Let Binding",
        r#"
        ; Let binding
        (let x 5.0
            (+ x 1.0))
        "#,
    );
}

fn example_6_comments() {
    run_example(
        "Comments and Whitespace",
        r#"
        ; This is a comment
        ; Multiple lines work
        (+
            ; Even inline
            2.0
            3.0)  ; End comment
        "#,
    );
}

// Helper to apply a value (for the examples)
fn apply_value(func: Value, arg: Value) -> Result<Value, EvalError> {
    match func {
        Value::Closure { param, body, env } => {
            let new_env = env.extend(param, arg);
            eval(&body, &new_env)
        }
        _ => Err(EvalError::NotAFunction),
    }
}
