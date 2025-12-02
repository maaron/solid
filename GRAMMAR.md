# FieldCalc S-Expression Grammar

Formal grammar for FieldCalc with optional type annotations.

## Lexical Elements

```
comment       ::= ';' <any character>* <newline>
whitespace    ::= <space> | <tab> | <newline> | comment

identifier    ::= <alpha> (<alpha> | <digit> | '_' | '-')*
                | '+' | '-' | '*' | '/' | '<' | '>' | '=='

scalar        ::= ['-'] <digit>+ '.' <digit>+ (['e' | 'E'] ['+' | '-'] <digit>+)?

nat           ::= <digit>+
```

**Notes:**
- Identifiers can contain underscores and hyphens: `vec_add`, `my-var`
- Operators are valid identifiers: `+`, `-`, `*`, `/`, `<`, `>`, `==`
- Keywords (like `fn`, `let`, `vec`) must be followed by non-identifier characters
- Whitespace and comments are ignored between tokens

## Type Grammar

```
type          ::= base_type
                | '(' 'Vec' nat ')'
                | '(' type '->' type ')'
                | '(' 'Field' nat type ')'
                | '(' 'forall' identifier '.' type ')'
                | identifier                           -- type variable

base_type     ::= 'Scalar'
                | 'Bool'
                | 'Color'
```

**Examples:**
```
Scalar
Bool
(Vec 2)
(Vec 3)
(Field 2 Scalar)
(Field 3 Color)
(Scalar -> Scalar)
(Scalar -> Scalar -> Scalar)
(Field 2 Scalar -> Field 2 Scalar)
(forall a . (a -> a))
```

**Note:** Types are currently used only in annotations. Type inference is not yet implemented.

## Expression Grammar

```
expr          ::= literal
                | identifier                           -- variable
                | '(' ')'                              -- nil (empty list)
                | '(' 'vec' expr* ')'                  -- vector
                | '(' 'rgb' expr expr expr ')'         -- color (RGB, alpha = 1.0)
                | '(' 'rgba' expr expr expr expr ')'   -- color (RGBA)
                | '(' 'cons' expr expr ')'             -- list cons
                | '(' 'fn' param expr ')'              -- lambda
                | '(' 'let' identifier expr expr ')'   -- let binding (infer type)
                | '(' 'field' field_param expr ')'     -- field construction
                | '(' 'if' expr expr expr ')'          -- conditional
                | '(' 'fold' expr expr expr ')'        -- fold (structural recursion)
                | '(' expr expr+ ')'                   -- function application

literal       ::= scalar
                | bool

bool          ::= 'true' | 'false'

param         ::= '(' identifier type ')'              -- typed parameter

field_param   ::= '(' identifier nat ')'               -- parameter with dimension
```

**Notes:**
- **Parameters must have types**: `(fn (x Scalar) ...)` - the type `Scalar` is required
- **Field parameters need dimension**: `(field (p 2) ...)` - the `2` is required
- **Let bindings don't require type**: `(let x 5.0 ...)` - type can be inferred or checked
- Application is left-associative: `(f a b c)` = `(((f a) b) c)`

## Complete Grammar (EBNF)

```ebnf
(* Lexical *)
Comment       = ';' {AnyChar} Newline ;
Whitespace    = Space | Tab | Newline | Comment ;
Identifier    = (Alpha {Alpha | Digit | '_' | '-'})
              | ('+' | '-' | '*' | '/' | '<' | '>' | '==') ;
Scalar        = ['-'] Digit+ '.' Digit+ [('e' | 'E') ['+' | '-'] Digit+] ;
Nat           = Digit+ ;

(* Types *)
Type          = BaseType
              | '(' 'Vec' Nat ')'
              | '(' Type '->' Type ')'
              | '(' 'Field' Nat Type ')'
              | '(' 'forall' Identifier '.' Type ')'
              | Identifier ;

BaseType      = 'Scalar' | 'Bool' | 'Color' ;

(* Expressions *)
Expr          = Literal
              | Identifier
              | Nil
              | Vector
              | Color
              | Cons
              | Lambda
              | Let
              | Field
              | If
              | Fold
              | Application ;

Literal       = Scalar | Bool ;
Bool          = 'true' | 'false' ;
Nil           = '(' ')' ;

Vector        = '(' 'vec' {Expr} ')' ;
Color         = '(' 'rgb' Expr Expr Expr ')'
              | '(' 'rgba' Expr Expr Expr Expr ')' ;
Cons          = '(' 'cons' Expr Expr ')' ;

Lambda        = '(' 'fn' Param Expr ')' ;
Param         = '(' Identifier Type ')' ;

Let           = '(' 'let' Identifier Expr Expr ')' ;

Field         = '(' 'field' FieldParam Expr ')' ;
FieldParam    = '(' Identifier Nat ')' ;

If            = '(' 'if' Expr Expr Expr ')' ;
Fold          = '(' 'fold' Expr Expr Expr ')' ;

Application   = '(' Expr Expr+ ')' ;
```

## Examples with Annotations

### Literals
```scheme
42.0                    ; Scalar
true                    ; Bool
false                   ; Bool
```

### Vectors and Colors
```scheme
(vec 1.0 2.0)           ; Vec 2 (implicit)
(vec 1.0 2.0 3.0)       ; Vec 3 (implicit)
(rgb 1.0 0.0 0.0)       ; Color (red, alpha=1.0)
(rgba 1.0 0.0 0.0 0.5)  ; Color (red, alpha=0.5)
```

### Variables
```scheme
x                       ; variable reference
my-variable             ; kebab-case allowed
vec_add                 ; snake_case allowed
+                       ; operators are identifiers
```

### Lambda (Type Required)
```scheme
; Identity function
(fn (x Scalar) x)

; Add one
(fn (x Scalar) (+ x 1.0))

; Function returning function
(fn (x Scalar)
  (fn (y Scalar)
    (+ x y)))
```

### Let Binding (Type Optional)
```scheme
; Simple let
(let x 5.0
  (+ x 1.0))

; Nested lets
(let x 5.0
  (let y 10.0
    (+ x y)))

; Let with function
(let double (fn (x Scalar) (* x 2.0))
  (double 21.0))
```

### Field Construction (Dimension Required)
```scheme
; 2D field: distance from origin
(field (p 2)
  (length p))

; 3D field: sphere SDF
(field (p 3)
  (- (length p) 5.0))

; Field using let
(let radius 10.0
  (field (p 2)
    (- (length p) radius)))
```

### Conditionals
```scheme
; Absolute value
(fn (x Scalar)
  (if (< x 0.0)
      (negate x)
      x))

; Conditional field
(field (p 2)
  (if (< (length p) 5.0)
      1.0
      0.0))
```

### Lists and Fold
```scheme
; Empty list
()

; List construction
(cons 1.0 (cons 2.0 (cons 3.0 ())))

; Sum a list
(fold (fn (acc Scalar)
        (fn (x Scalar)
          (+ acc x)))
      0.0
      (cons 1.0 (cons 2.0 (cons 3.0 ()))))
```

### Function Application
```scheme
; Binary operation
(+ 2.0 3.0)

; Currying (left-associative)
(+ 2.0 3.0)           ; same as ((+ 2.0) 3.0)

; Multiple arguments
(vec_add
  (vec 1.0 2.0)
  (vec 3.0 4.0))

; Sampling a field
(sample
  (field (p 2) (length p))
  (vec 3.0 4.0))
```

### Complete Example: Circle SDF
```scheme
; Function that creates a circle SDF for a given radius
(fn (radius Scalar)
  (field (p 2)
    (- (length p) radius)))

; Apply it to create a circle of radius 5
(let circle
  ((fn (radius Scalar)
     (field (p 2)
       (- (length p) radius)))
   5.0)

  ; Sample at a point
  (sample circle (vec 3.0 4.0)))
```

### Complete Example: CSG Union
```scheme
; Two circles unioned via min
(let c1 (field (p 2)
          (- (length p) 5.0))
(let c2 (field (p 2)
          (- (length (vec_sub p (vec 10.0 0.0))) 3.0))

  ; Union
  (field (p 2)
    (min (sample c1 p)
         (sample c2 p)))))
```

## Type Annotation Requirements

### Required Type Annotations

1. **Lambda parameters**: `(fn (x Scalar) ...)`
   - The type `Scalar` is **required**
   - Enables type checking of function body

2. **Field parameters**: `(field (p 2) ...)`
   - The dimension `2` is **required**
   - Determines field dimensionality at runtime

### Optional Type Annotations

1. **Let bindings**: `(let x 5.0 ...)`
   - Type can be inferred from the value
   - Could support explicit annotation in future: `(let (x Scalar) 5.0 ...)`

2. **Expression-level types**: Currently not annotatable
   - Could add in future: `(the Scalar (+ x y))`

## Desugaring and Semantics

### Currying
Function application is left-associative:
```scheme
(f a b c)  ≡  (((f a) b) c)
```

### Multi-argument Functions
Must be defined using nested lambdas:
```scheme
; Two-argument function
(fn (x Scalar)
  (fn (y Scalar)
    (+ x y)))

; Usage
((fn (x Scalar) (fn (y Scalar) (+ x y))) 2.0 3.0)
; = (((fn (x Scalar) (fn (y Scalar) (+ x y))) 2.0) 3.0)
; = ((fn (y Scalar) (+ 2.0 y)) 3.0)
; = (+ 2.0 3.0)
; = 5.0
```

Built-in functions handle multiple arguments via currying automatically.

### Lists
Lists are built from `cons` and `()`:
```scheme
()                                    ; empty list
(cons x ())                          ; [x]
(cons x (cons y ()))                 ; [x, y]
(cons x (cons y (cons z ())))        ; [x, y, z]
```

Could add sugar later: `[x y z]` → `(cons x (cons y (cons z ())))`

## Reserved Keywords

The following are reserved and cannot be used as identifiers:

- `fn` - lambda abstraction
- `let` - let binding
- `field` - field construction
- `if` - conditional
- `fold` - structural recursion
- `cons` - list construction
- `vec` - vector literal
- `rgb` - color literal (RGB)
- `rgba` - color literal (RGBA)
- `true` - boolean literal
- `false` - boolean literal

Type keywords (used in annotations):
- `Scalar` - scalar type
- `Bool` - boolean type
- `Color` - color type
- `Vec` - vector type constructor
- `Field` - field type constructor
- `forall` - universal quantification

## Comments

```scheme
; Single-line comment (semicolon to end of line)

(+ 1.0 2.0)  ; Inline comments work too

; Multi-line comments require semicolon on each line:
; This is line 1
; This is line 2
; This is line 3
```

## Future Extensions

Possible syntax additions that don't break existing code:

1. **Let with type annotation**: `(let (x Scalar) 5.0 body)`
2. **Type ascription**: `(the Scalar (+ x y))`
3. **List sugar**: `[1.0 2.0 3.0]` → `(cons 1.0 (cons 2.0 (cons 3.0 ())))`
4. **Vector index sugar**: `(@ v 0)` → first element
5. **Record types**: `{x: 1.0, y: 2.0}`
6. **Pattern matching**: `(match list ...)`

All can be added without breaking existing S-expression code.
