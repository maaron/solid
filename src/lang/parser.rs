/// Parser for FieldCalc
///
/// Simple S-expression syntax:
/// - (+ 2.0 3.0)
/// - (fn (x Scalar) (+ x 1.0))
/// - (field (p 2) (length p))
/// - (let x 5.0 (+ x 1.0))

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while1},
    character::complete::{char, multispace1},
    combinator::{map, recognize, value},
    multi::many0,
    number::complete::double,
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};

use super::ast::Expr;
use super::types::Type;

/// Parse error type
#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    pub message: String,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse error: {}", self.message)
    }
}

impl std::error::Error for ParseError {}

// ===== Lexical Elements =====

/// Comment starts with ; and goes to end of line
fn comment(input: &str) -> IResult<&str, ()> {
    value(
        (),
        pair(
            char(';'),
            take_while(|c| c != '\n'),
        ),
    )(input)
}

/// Whitespace including comments
fn ws(input: &str) -> IResult<&str, ()> {
    value(
        (),
        many0(alt((
            value((), multispace1),
            comment,
        ))),
    )(input)
}

/// Wrap a parser to skip whitespace before and after
fn lexeme<'a, F, O>(parser: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: FnMut(&'a str) -> IResult<&'a str, O>,
{
    delimited(ws, parser, ws)
}

/// Identifier (variable name or operator)
fn identifier(input: &str) -> IResult<&str, String> {
    alt((
        // Operators: +, -, *, /, <, >, ==, etc.
        map(
            alt((
                tag("=="),
                tag("+"),
                tag("-"),
                tag("*"),
                tag("/"),
                tag("<"),
                tag(">"),
            )),
            |s: &str| s.to_string(),
        ),
        // Regular identifiers
        map(
            recognize(pair(
                take_while1(|c: char| c.is_alphabetic() || c == '_'),
                take_while(|c: char| c.is_alphanumeric() || c == '_' || c == '-'),
            )),
            |s: &str| s.to_string(),
        ),
    ))(input)
}

// ===== Type Parsing =====

fn parse_type(input: &str) -> IResult<&str, Type> {
    alt((
        value(Type::Scalar, tag("Scalar")),
        value(Type::Color, tag("Color")),
        value(Type::Bool, tag("Bool")),
        map(
            preceded(tag("Vec"), preceded(multispace1, nom::character::complete::u64)),
            |n| Type::Vec(n as usize),
        ),
    ))(input)
}

// ===== Expression Parsing =====

/// Parse a scalar literal
fn parse_scalar(input: &str) -> IResult<&str, Expr> {
    map(lexeme(double), Expr::scalar)(input)
}

/// Parse a boolean literal
fn parse_bool(input: &str) -> IResult<&str, Expr> {
    lexeme(alt((
        value(Expr::bool(true), tag("true")),
        value(Expr::bool(false), tag("false")),
    )))(input)
}

/// Parse a variable
fn parse_var(input: &str) -> IResult<&str, Expr> {
    map(lexeme(identifier), Expr::var)(input)
}

/// Parse a vector: (vec 1.0 2.0 3.0)
fn parse_vector(input: &str) -> IResult<&str, Expr> {
    map(
        delimited(
            tuple((lexeme(char('(')), lexeme(tag("vec")))),
            many0(parse_expr),
            lexeme(char(')')),
        ),
        |components| Expr::Vec(components),
    )(input)
}

/// Parse a color: (rgb 1.0 0.0 0.0) or (rgba 1.0 0.0 0.0 0.5)
fn parse_color(input: &str) -> IResult<&str, Expr> {
    alt((
        // rgba
        map(
            delimited(
                tuple((lexeme(char('(')), lexeme(tag("rgba")))),
                tuple((parse_expr, parse_expr, parse_expr, parse_expr)),
                lexeme(char(')')),
            ),
            |(r, g, b, a)| Expr::Color {
                r: Box::new(r),
                g: Box::new(g),
                b: Box::new(b),
                a: Box::new(a),
            },
        ),
        // rgb (alpha defaults to 1.0)
        map(
            delimited(
                tuple((lexeme(char('(')), lexeme(tag("rgb")))),
                tuple((parse_expr, parse_expr, parse_expr)),
                lexeme(char(')')),
            ),
            |(r, g, b)| Expr::Color {
                r: Box::new(r),
                g: Box::new(g),
                b: Box::new(b),
                a: Box::new(Expr::scalar(1.0)),
            },
        ),
    ))(input)
}

/// Parse empty list: ()
fn parse_nil(input: &str) -> IResult<&str, Expr> {
    value(
        Expr::nil(),
        tuple((lexeme(char('(')), lexeme(char(')')))),
    )(input)
}

/// Parse cons: (cons head tail)
fn parse_cons(input: &str) -> IResult<&str, Expr> {
    map(
        delimited(
            tuple((lexeme(char('(')), lexeme(tag("cons")))),
            pair(parse_expr, parse_expr),
            lexeme(char(')')),
        ),
        |(head, tail)| Expr::cons(head, tail),
    )(input)
}

/// Parse lambda: (fn (x Type) body)
fn parse_lambda(input: &str) -> IResult<&str, Expr> {
    map(
        delimited(
            tuple((lexeme(char('(')), lexeme(tag("fn")))),
            tuple((
                // Parameter with type: (x Type)
                delimited(
                    lexeme(char('(')),
                    pair(lexeme(identifier), lexeme(parse_type)),
                    lexeme(char(')')),
                ),
                // Body
                parse_expr,
            )),
            lexeme(char(')')),
        ),
        |((param, param_ty), body)| Expr::lambda(param, param_ty, body),
    )(input)
}

/// Parse let: (let var value body)
fn parse_let(input: &str) -> IResult<&str, Expr> {
    map(
        delimited(
            tuple((lexeme(char('(')), lexeme(tag("let")))),
            tuple((
                lexeme(identifier),
                parse_expr,
                parse_expr,
            )),
            lexeme(char(')')),
        ),
        |(var, value, body)| {
            // Infer type from value (for now, we'll use Scalar as default)
            // A proper implementation would do type inference
            Expr::let_bind(var, Type::Scalar, value, body)
        },
    )(input)
}

/// Parse field: (field (p dim) body)
fn parse_field(input: &str) -> IResult<&str, Expr> {
    map(
        delimited(
            tuple((lexeme(char('(')), lexeme(tag("field")))),
            tuple((
                // Parameter with dimension: (p 2)
                delimited(
                    lexeme(char('(')),
                    pair(lexeme(identifier), lexeme(nom::character::complete::u64)),
                    lexeme(char(')')),
                ),
                // Body
                parse_expr,
            )),
            lexeme(char(')')),
        ),
        |((param, dim), body)| Expr::field(param, dim as usize, body),
    )(input)
}

/// Parse if: (if cond then else)
fn parse_if(input: &str) -> IResult<&str, Expr> {
    map(
        delimited(
            tuple((lexeme(char('(')), lexeme(tag("if")))),
            tuple((parse_expr, parse_expr, parse_expr)),
            lexeme(char(')')),
        ),
        |(cond, then_branch, else_branch)| Expr::if_then_else(cond, then_branch, else_branch),
    )(input)
}

/// Parse fold: (fold func init list)
fn parse_fold(input: &str) -> IResult<&str, Expr> {
    map(
        delimited(
            tuple((lexeme(char('(')), lexeme(tag("fold")))),
            tuple((parse_expr, parse_expr, parse_expr)),
            lexeme(char(')')),
        ),
        |(func, init, list)| Expr::Fold {
            func: Box::new(func),
            init: Box::new(init),
            list: Box::new(list),
        },
    )(input)
}

/// Parse function application: (func arg1 arg2 ...)
fn parse_application(input: &str) -> IResult<&str, Expr> {
    map(
        delimited(
            lexeme(char('(')),
            pair(
                parse_expr,
                many0(parse_expr),
            ),
            lexeme(char(')')),
        ),
        |(func, args)| {
            // Left-associate applications: (f a b c) = (((f a) b) c)
            args.into_iter().fold(func, |acc, arg| Expr::app(acc, arg))
        },
    )(input)
}

/// Parse any expression
pub fn parse_expr(input: &str) -> IResult<&str, Expr> {
    alt((
        parse_bool,
        parse_scalar,
        parse_nil,  // Must come before application!
        parse_vector,
        parse_color,
        parse_lambda,
        parse_let,
        parse_field,
        parse_if,
        parse_fold,
        parse_cons,
        parse_application,
        parse_var,
    ))(input)
}

/// Parse a complete program (expression with trailing whitespace)
pub fn parse_program(input: &str) -> Result<Expr, ParseError> {
    match delimited(ws, parse_expr, ws)(input) {
        Ok(("", expr)) => Ok(expr),
        Ok((remaining, _)) => Err(ParseError {
            message: format!("Unexpected input remaining: {}", remaining),
        }),
        Err(e) => Err(ParseError {
            message: format!("Parse error: {:?}", e),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_scalar() {
        let expr = parse_program("42.0").unwrap();
        assert_eq!(expr, Expr::scalar(42.0));

        let expr = parse_program("3.14159").unwrap();
        assert_eq!(expr, Expr::scalar(3.14159));
    }

    #[test]
    fn test_parse_bool() {
        let expr = parse_program("true").unwrap();
        assert_eq!(expr, Expr::bool(true));

        let expr = parse_program("false").unwrap();
        assert_eq!(expr, Expr::bool(false));
    }

    #[test]
    fn test_parse_var() {
        let expr = parse_program("x").unwrap();
        assert_eq!(expr, Expr::var("x"));

        let expr = parse_program("my_var").unwrap();
        assert_eq!(expr, Expr::var("my_var"));
    }

    #[test]
    fn test_parse_vector() {
        let expr = parse_program("(vec 1.0 2.0)").unwrap();
        match expr {
            Expr::Vec(components) => {
                assert_eq!(components.len(), 2);
            }
            _ => panic!("Expected vector"),
        }
    }

    #[test]
    fn test_parse_application() {
        let expr = parse_program("(+ 2.0 3.0)").unwrap();
        match expr {
            Expr::App { func, arg } => {
                match (*func, *arg) {
                    (Expr::App { func: f2, arg: a1 }, a2) => {
                        assert_eq!(*f2, Expr::var("+"));
                        assert_eq!(*a1, Expr::scalar(2.0));
                        assert_eq!(a2, Expr::scalar(3.0));
                    }
                    _ => panic!("Expected nested application"),
                }
            }
            _ => panic!("Expected application"),
        }
    }

    #[test]
    fn test_parse_lambda() {
        let expr = parse_program("(fn (x Scalar) (+ x 1.0))").unwrap();
        match expr {
            Expr::Lambda { param, param_ty, body: _ } => {
                assert_eq!(param, "x");
                assert_eq!(param_ty, Type::Scalar);
            }
            _ => panic!("Expected lambda"),
        }
    }

    #[test]
    fn test_parse_let() {
        let expr = parse_program("(let x 5.0 (+ x 1.0))").unwrap();
        match expr {
            Expr::Let { var, value, .. } => {
                assert_eq!(var, "x");
                assert_eq!(*value, Expr::scalar(5.0));
            }
            _ => panic!("Expected let"),
        }
    }

    #[test]
    fn test_parse_field() {
        let expr = parse_program("(field (p 2) (length p))").unwrap();
        match expr {
            Expr::Field { param, dim, .. } => {
                assert_eq!(param, "p");
                assert_eq!(dim, 2);
            }
            _ => panic!("Expected field"),
        }
    }

    #[test]
    fn test_parse_if() {
        let expr = parse_program("(if (< x 0.0) 0.0 x)").unwrap();
        match expr {
            Expr::If { .. } => {}
            _ => panic!("Expected if"),
        }
    }

    #[test]
    fn test_parse_with_comments() {
        let program = r#"
            ; This is a comment
            (+ 2.0 3.0)  ; Another comment
        "#;
        let expr = parse_program(program).unwrap();
        match expr {
            Expr::App { .. } => {}
            _ => panic!("Expected application"),
        }
    }

    #[test]
    fn test_parse_nested() {
        let expr = parse_program("(+ (* 2.0 3.0) 4.0)").unwrap();
        match expr {
            Expr::App { .. } => {}
            _ => panic!("Expected application"),
        }
    }

    #[test]
    fn test_parse_circle() {
        let program = r#"
            (fn (r Scalar)
                (field (p 2)
                    (- (length p) r)))
        "#;
        let expr = parse_program(program).unwrap();
        match expr {
            Expr::Lambda { .. } => {}
            _ => panic!("Expected lambda"),
        }
    }
}
