/// Examples using the FieldCalc language
use solid_modeling::lang::*;

fn main() {
    println!("=== FieldCalc Language Examples ===\n");

    example_1_simple_arithmetic();
    example_2_lambda();
    example_3_field();
    example_4_circle_sdf();
    example_5_structural_recursion();
}

fn example_1_simple_arithmetic() {
    println!("Example 1: Simple Arithmetic");
    println!("Expression: 2.0 + 3.0");

    let env = Env::new();

    // (+ 2.0 3.0)
    let expr = Expr::app(
        Expr::app(
            Expr::builtin("+"),
            Expr::scalar(2.0)
        ),
        Expr::scalar(3.0)
    );

    match eval(&expr, &env) {
        Ok(Value::Scalar(v)) => println!("Result: {}\n", v),
        Ok(v) => println!("Unexpected result: {:?}\n", v),
        Err(e) => println!("Error: {}\n", e),
    }
}

fn example_2_lambda() {
    println!("Example 2: Lambda Function");
    println!("Expression: (λx. x + 1.0) 5.0");

    let env = Env::new();

    // λ(x : Scalar). x + 1.0
    let lambda = Expr::lambda(
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

    // Apply to 5.0
    let expr = Expr::app(lambda, Expr::scalar(5.0));

    match eval(&expr, &env) {
        Ok(Value::Scalar(v)) => println!("Result: {}\n", v),
        Ok(v) => println!("Unexpected result: {:?}\n", v),
        Err(e) => println!("Error: {}\n", e),
    }
}

fn example_3_field() {
    println!("Example 3: Field Construction");
    println!("Expression: field (p : Vec 2) ↦ length p");

    let env = Env::new();

    // field (p : Vec 2) ↦ length p
    let field_expr = Expr::field(
        "p",
        2,
        Expr::app(
            Expr::builtin("length"),
            Expr::var("p")
        )
    );

    let field_value = eval(&field_expr, &env).unwrap();

    // Sample the field at point (3.0, 4.0)
    let point = Value::Vec(vec![3.0, 4.0]);

    match sample_field(field_value, point) {
        Ok(Value::Scalar(v)) => println!("Sampled at (3.0, 4.0): {}\n", v),
        Ok(v) => println!("Unexpected result: {:?}\n", v),
        Err(e) => println!("Error: {}\n", e),
    }
}

fn example_4_circle_sdf() {
    println!("Example 4: Circle SDF");
    println!("Expression: field (p : Vec 2) ↦ length p − radius");

    let env = Env::new();

    // circle(r) = field (p : Vec 2) ↦ length p − r
    let circle = Expr::lambda(
        "r",
        Type::Scalar,
        Expr::field(
            "p",
            2,
            Expr::app(
                Expr::app(
                    Expr::builtin("-"),
                    Expr::app(
                        Expr::builtin("length"),
                        Expr::var("p")
                    )
                ),
                Expr::var("r")
            )
        )
    );

    // circle 2.0
    let circle_2 = Expr::app(circle, Expr::scalar(2.0));
    let field_value = eval(&circle_2, &env).unwrap();

    // Sample at various points
    let test_points = vec![
        (0.0, 0.0),   // Center (inside)
        (2.0, 0.0),   // On boundary
        (3.0, 0.0),   // Outside
    ];

    for (x, y) in test_points {
        let point = Value::Vec(vec![x, y]);
        if let Ok(Value::Scalar(d)) = sample_field(field_value.clone(), point) {
            let status = if d < 0.0 {
                "inside"
            } else if d.abs() < 0.001 {
                "boundary"
            } else {
                "outside"
            };
            println!("  Point ({}, {}): distance = {:.3} ({})", x, y, d, status);
        }
    }
    println!();
}

fn example_5_structural_recursion() {
    println!("Example 5: Structural Recursion (List Sum)");
    println!("Expression: fold (+) 0 [1, 2, 3, 4]");

    let env = Env::new();

    // Build a list: [1, 2, 3, 4]
    let list = Expr::cons(
        Expr::scalar(1.0),
        Expr::cons(
            Expr::scalar(2.0),
            Expr::cons(
                Expr::scalar(3.0),
                Expr::cons(
                    Expr::scalar(4.0),
                    Expr::nil()
                )
            )
        )
    );

    // fold (+) 0 list
    let fold_expr = Expr::Fold {
        func: Box::new(Expr::builtin("+")),
        init: Box::new(Expr::scalar(0.0)),
        list: Box::new(list),
    };

    match eval(&fold_expr, &env) {
        Ok(Value::Scalar(v)) => println!("Sum: {}\n", v),
        Ok(v) => println!("Unexpected result: {:?}\n", v),
        Err(e) => println!("Error: {}\n", e),
    }
}
