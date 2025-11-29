# FieldCalc Syntax Reference

Simple, unambiguous S-expression syntax for experimentation.

## Philosophy

- **Prefix notation**: No operator precedence to remember
- **Parentheses for structure**: Clear boundaries
- **No syntactic sugar**: One way to do things
- **Comments**: Start with `;`

## Literals

```scheme
; Numbers (floating point)
42.0
3.14159
-1.5

; Booleans
true
false

; Vectors
(vec 1.0 2.0)        ; 2D vector
(vec 1.0 2.0 3.0)    ; 3D vector

; Colors
(rgb 1.0 0.0 0.0)           ; Red (alpha = 1.0)
(rgba 1.0 0.0 0.0 0.5)      ; Semi-transparent red

; Empty list
()

; Cons cell
(cons 1.0 ())
(cons 1.0 (cons 2.0 (cons 3.0 ())))  ; List [1, 2, 3]
```

## Variables

```scheme
x
my_var
some-name
```

## Function Application

```scheme
; Binary operators (prefix notation)
(+ 2.0 3.0)              ; 2.0 + 3.0
(* 4.0 5.0)              ; 4.0 * 5.0
(- x 1.0)                ; x - 1.0

; Unary functions
(length p)               ; length(p)
(sin x)                  ; sin(x)
(abs x)                  ; abs(x)

; Multiple arguments (left-associative)
(+ 1.0 2.0 3.0)          ; ((+ 1.0) 2.0) 3.0

; Nested calls
(+ (* 2.0 3.0) 4.0)      ; (2.0 * 3.0) + 4.0
```

## Lambda Functions

```scheme
; Syntax: (fn (param Type) body)

(fn (x Scalar) (+ x 1.0))

(fn (p Vec 2) (length p))

(fn (r Scalar)
    (fn (g Scalar)
        (fn (b Scalar)
            (rgb r g b))))
```

## Let Bindings

```scheme
; Syntax: (let var value body)

(let x 5.0
    (+ x 1.0))

(let radius 2.0
    (let center (vec 0.0 0.0)
        (field (p 2)
            (- (length (vec_sub p center)) radius))))
```

## Fields

```scheme
; Syntax: (field (param dim) body)

; 2D field
(field (p 2)
    (length p))

; 3D field
(field (p 3)
    (- (length p) 1.0))

; Field using other variables
(field (p 2)
    (- (length p) radius))
```

## Conditionals

```scheme
; Syntax: (if condition then-expr else-expr)

(if (< x 0.0)
    0.0
    x)

(if (< d 0.0)
    (rgb 1.0 0.0 0.0)
    (rgb 0.0 0.0 1.0))
```

## Structural Recursion

```scheme
; Syntax: (fold func init list)

; Sum a list
(fold + 0.0 my_list)

; Length of a list
(fold (fn (x Scalar) (fn (acc Scalar) (+ acc 1.0)))
      0.0
      my_list)
```

## Comments

```scheme
; Single-line comments start with semicolon

(+ 2.0 3.0)  ; Inline comments work too

; Multi-line "comments" are just multiple single-line comments
; Like this
; And this
```

## Complete Examples

### Circle SDF

```scheme
; Define circle as a function
(fn (r Scalar)
    (field (p 2)
        (- (length p) r)))

; Apply to radius 2.0
((fn (r Scalar)
     (field (p 2)
         (- (length p) r)))
 2.0)
```

### Union of Two Circles

```scheme
(let circle1
    (field (p 2)
        (- (length (vec_sub p (vec -1.0 0.0))) 1.5))
    (let circle2
        (field (p 2)
            (- (length (vec_sub p (vec 1.0 0.0))) 1.5))
        ; Union is min
        (field (p 2)
            (min (circle1 p) (circle2 p)))))
```

### Smooth Union (Metaballs)

```scheme
(let k 0.8
    (let d1 (field (p 2) (- (length (vec_sub p (vec -1.0 0.0))) 1.5))
        (let d2 (field (p 2) (- (length (vec_sub p (vec 1.0 0.0))) 1.5))
            (field (p 2)
                (let a (d1 p)
                    (let b (d2 p)
                        (let h (max 0.0 (min 1.0 (+ 0.5 (* 0.5 (/ (- b a) k)))))
                            (- (+ (* b (- 1.0 h)) (* a h))
                               (* k h (- 1.0 h))))))))))
```

### List Sum

```scheme
(let my_list
    (cons 1.0 (cons 2.0 (cons 3.0 (cons 4.0 ()))))
    (fold + 0.0 my_list))
```

## Built-in Functions

### Scalar Operations
- `+`, `-`, `*`, `/` - Arithmetic (curried binary)
- `min`, `max` - Min/max (curried binary)
- `abs`, `sin`, `cos`, `sqrt` - Unary math functions
- `negate` - Negation

### Comparison
- `<`, `>`, `==` - Comparison (return Bool)

### Vector Operations
- `length` - Vector magnitude
- `dot` - Dot product (curried binary)
- `normalize` - Normalize vector
- `vec_add`, `vec_sub` - Vector addition/subtraction (curried binary)
- `vec_scale` - Scalar multiplication (curried binary)

## Types

Available types for annotations:
- `Scalar` - Floating point number
- `Bool` - Boolean
- `Vec n` - n-dimensional vector (e.g., `Vec 2`, `Vec 3`)
- `Color` - RGBA color

## Tips

1. **Start simple**: Test expressions in the REPL first
2. **Use let liberally**: Name intermediate values for clarity
3. **Parentheses matter**: Every function call needs them
4. **Comments help**: Document what complex expressions do
5. **Whitespace is free**: Use indentation to show structure

## Quick Reference

```scheme
; Literals
42.0    true    (vec 1.0 2.0)    (rgb 1 0 0)    ()

; Application
(f x)    (+ 2 3)    (length p)

; Abstraction
(fn (x Scalar) body)

; Binding
(let x 5.0 body)

; Fields
(field (p 2) body)

; Control
(if cond then else)

; Recursion
(fold f z xs)
```
