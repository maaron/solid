/// Examples demonstrating Field 0 semantics
///
/// Key concepts:
/// - All values are Field n α for some n ≥ 0
/// - Field 0 α represents constants (same value everywhere)
/// - Literals (5.0, true, (vec 1.0 2.0)) are Field 0
/// - sample :: Field n α → Vec n → Field 0 α
/// - Field 0 α can lift to Field n α (constant field)

use solid_modeling::lang::parser::parse_program;
use solid_modeling::lang::eval::{eval, sample_field};
use solid_modeling::lang::env::{Env, Value};

fn main() {
    println!("=== Field 0 Semantics Examples ===\n");

    let env = Env::with_builtins();

    // Example 1: Basic field and sampling
    println!("Example 1: Field Creation and Sampling");
    println!("---------------------------------------");
    let code = r#"
; Create a field: distance from origin
(field (p 2) (length p))
    "#;
    println!("Code:\n{}", code);

    match parse_program(code.trim()) {
        Ok(expr) => {
            match eval(&expr, &env) {
                Ok(field) => {
                    println!("Field created: Field 2 Scalar (at type level)");
                    println!("Runtime representation: Value::Field\n");

                    // Sample at point (3.0, 4.0)
                    let point = Value::Vec(vec![3.0, 4.0]);
                    match sample_field(field, point) {
                        Ok(Value::Scalar(v)) => {
                            println!("sample field (vec 3.0 4.0) = {}", v);
                            println!("Type: Field 0 Scalar");
                            println!("Runtime: Value::Scalar({})\n", v);
                        }
                        Ok(v) => println!("Unexpected result: {:?}\n", v),
                        Err(e) => println!("Error: {}\n", e),
                    }
                }
                Err(e) => println!("Error: {}\n", e),
            }
        }
        Err(e) => println!("Parse error: {:?}\n", e),
    }

    // Example 2: Using sampled values (Field 0)
    println!("\nExample 2: Composing with Sampled Values");
    println!("-----------------------------------------");
    let code = r#"
; Sample and add to constant
(let f (field (p 2) (length p))
  (+ (sample f (vec 3.0 4.0)) 10.0))
    "#;
    println!("Code:\n{}", code);

    match parse_program(code.trim()) {
        Ok(expr) => {
            match eval(&expr, &env) {
                Ok(Value::Scalar(v)) => {
                    println!("Result: {}", v);
                    println!("Type: Field 0 Scalar (constant)");
                    println!("Semantics: sample returns Field 0, + works on Field 0 values\n");
                }
                Ok(v) => println!("Unexpected result: {:?}\n", v),
                Err(e) => println!("Error: {}\n", e),
            }
        }
        Err(e) => println!("Parse error: {:?}\n", e),
    }

    // Example 3: Field composition via sampling
    println!("\nExample 3: Field Composition (Warping)");
    println!("---------------------------------------");
    let code = r#"
; Simple example: offset a field
(let base (field (p 2) (length p))
(let offset (vec 1.0 0.0)
  (field (p 2)
    (sample base (vec_add p offset)))))
    "#;
    println!("Code:\n{}", code);
    println!("Semantics:");
    println!("  base : Field 2 Scalar");
    println!("  offset : Field 0 (Vec 2)");
    println!("  Inside field body:");
    println!("    p : Vec 2 (the parameter - Field 0 at type level)");
    println!("    vec_add p offset : Field 0 (Vec 2)");
    println!("    sample base (...) : Field 0 Scalar");
    println!("  Outer field : Field 2 Scalar\n");

    match parse_program(code.trim()) {
        Ok(expr) => {
            match eval(&expr, &env) {
                Ok(field) => {
                    println!("Field created successfully");

                    // Sample at origin - should give length of (1, 0)
                    let point = Value::Vec(vec![0.0, 0.0]);
                    match sample_field(field, point) {
                        Ok(Value::Scalar(v)) => {
                            println!("At origin (0, 0): {:.2}", v);
                            println!("(Should be 1.0 since we sample at (0+1, 0+0) = (1, 0))");
                        }
                        Ok(v) => println!("Unexpected: {:?}", v),
                        Err(e) => println!("Error: {}", e),
                    }
                }
                Err(e) => println!("Error: {}\n", e),
            }
        }
        Err(e) => println!("Parse error: {:?}\n", e),
    }

    // Example 4: CSG via min (demonstrating Field 0 lifting)
    println!("\n\nExample 4: CSG Operations (Union)");
    println!("----------------------------------");
    let code = r#"
; Two circles via min
(let c1 (field (p 2) (- (length p) 5.0))
(let c2 (field (p 2) (- (length (vec_sub p (vec 10.0 0.0))) 3.0))
  ; Inside a field, we can't directly min the fields
  ; We need to sample them or work pointwise
  (field (p 2)
    (min (sample c1 p) (sample c2 p)))))
    "#;
    println!("Code:\n{}", code);
    println!("Semantics:");
    println!("  c1, c2 : Field 2 Scalar");
    println!("  Inside union field:");
    println!("    p : Vec 2 (Field 0)");
    println!("    sample c1 p : Field 0 Scalar");
    println!("    sample c2 p : Field 0 Scalar");
    println!("    min : Field 0 Scalar → Field 0 Scalar → Field 0 Scalar");
    println!("  Result: Field 2 Scalar\n");

    match parse_program(code.trim()) {
        Ok(expr) => {
            match eval(&expr, &env) {
                Ok(field) => {
                    println!("Union field created");

                    // Test at a few points
                    let test_points = vec![
                        (vec![0.0, 0.0], "origin (inside c1)"),
                        (vec![5.0, 0.0], "boundary of c1"),
                        (vec![10.0, 0.0], "center of c2"),
                    ];

                    for (coords, desc) in test_points {
                        let point = Value::Vec(coords.clone());
                        match sample_field(field.clone(), point) {
                            Ok(Value::Scalar(v)) => {
                                println!("  {:?}: {:.2} - {}", coords, v, desc);
                            }
                            _ => {}
                        }
                    }
                }
                Err(e) => println!("Error: {}\n", e),
            }
        }
        Err(e) => println!("Parse error: {:?}\n", e),
    }

    // Example 5: Demonstrating Field 0 concept
    println!("\n\nExample 5: Understanding Field 0");
    println!("----------------------------------");
    println!("At the type level:");
    println!("  5.0           : Field 0 Scalar");
    println!("  (vec 1.0 2.0) : Field 0 (Vec 2)");
    println!("  true          : Field 0 Bool");
    println!();
    println!("At runtime:");
    println!("  Field 0 values are represented directly as Value::Scalar, Value::Vec, etc.");
    println!("  Field n values (n > 0) are represented as Value::Field with closures");
    println!();
    println!("Operations:");
    println!("  + : Field 0 Scalar → Field 0 Scalar → Field 0 Scalar");
    println!("  length : Field 0 (Vec n) → Field 0 Scalar");
    println!("  sample : Field n α → Field 0 (Vec n) → Field 0 α");
    println!();
    println!("Auto-lifting (not yet implemented):");
    println!("  When mixing Field 0 and Field n, Field 0 lifts to constant Field n");
    println!("  Example: (+ field 5.0) would lift 5.0 to (field (p n) 5.0)");
}
