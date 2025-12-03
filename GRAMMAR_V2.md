# FieldCalc S-Expression Grammar (Revised)

Formal grammar for FieldCalc with optional type annotations and consistent S-expression syntax.

## Design Principles

1. **Consistent S-expression style**: No infix operators, even in types
2. **Laziness is operational**: `field` vs `fn` differs in evaluation, not types
3. **All types optional**: Every type annotation can be omitted
4. **Explicit annotation syntax**: Use `(: Type expr)` to annotate

## Lexical Elements

```
comment       ::= ';' <any character>* <newline>
whitespace    ::= <space> | <tab> | <newline> | comment

identifier    ::= <alpha> (<alpha> | <digit> | '_' | '-')*
                | '+' | '-' | '*' | '/' | '<' | '>' | '=='

scalar        ::= ['-'] <digit>+ '.' <digit>+ (['e' | 'E'] ['+' | '-'] <digit>+)?

nat           ::= <digit>+
```

## Type Grammar

```
type          ::= base_type
                | '(' 'Vec' nat ')'
                | '(' '->' type+ ')'                   -- function type (S-expression style)
                | '(' 'forall' identifier type ')'     -- universal quantification
                | identifier                           -- type variable

base_type     ::= 'Scalar'
                | 'Bool'
                | 'Color'
```

**Key changes from previous version:**
- Function types use `(-> α β)` instead of `(α -> β)` - consistent prefix notation
- No `Field` type - fields have regular function types `(-> (Vec n) α)`
- Forall uses S-expression: `(forall a τ)` instead of `(forall a . τ)`

**Examples:**
```scheme
Scalar                          ; base type
(Vec 2)                         ; 2D vector type
(Vec 3)                         ; 3D vector type

(-> Scalar Scalar)              ; function Scalar → Scalar
(-> Scalar Scalar Bool)         ; function Scalar → Scalar → Bool
(-> (Vec 2) Scalar)             ; function Vec 2 → Scalar (e.g., a 2D field)
(-> (Vec 2) Scalar Scalar)      ; function Vec 2 → Scalar → Scalar

(forall a (-> a a))             ; polymorphic identity
(forall n (-> (Vec n) Scalar))  ; polymorphic length (dimension-polymorphic)
```

## Expression Grammar

```
expr          ::= literal
                | identifier                           -- variable
                | '(' ':' type expr ')'                -- type annotation
                | '(' ')'                              -- nil (empty list)
                | '(' 'vec' expr* ')'                  -- vector
                | '(' 'rgb' expr expr expr ')'         -- color (RGB)
                | '(' 'rgba' expr expr expr expr ')'   -- color (RGBA)
                | '(' 'cons' expr expr ')'             -- list cons
                | '(' 'fn' param expr ')'              -- lambda (strict)
                | '(' 'field' param expr ')'           -- field (lazy)
                | '(' 'let' identifier expr expr ')'   -- let binding
                | '(' 'if' expr expr expr ')'          -- conditional
                | '(' 'fold' expr expr expr ')'        -- fold
                | '(' expr expr+ ')'                   -- application

literal       ::= scalar
                | bool

bool          ::= 'true' | 'false'

param         ::= identifier                           -- unannotated parameter
                | '(' ':' identifier type ')'          -- annotated parameter
```

**Key changes:**
- New form: `(: Type expr)` for type annotations
- Parameters can be `x` or `(: x Scalar)` - annotation is optional
- `field` has same parameter syntax as `fn` - both use `param`
- No special dimension syntax for field (dimension comes from type or runtime)

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
              | '(' '->' Type+ ')'
              | '(' 'forall' Identifier Type ')'
              | Identifier ;

BaseType      = 'Scalar' | 'Bool' | 'Color' ;

(* Expressions *)
Expr          = Literal
              | Identifier
              | TypeAnnotation
              | Nil
              | Vector
              | Color
              | Cons
              | Lambda
              | Field
              | Let
              | If
              | Fold
              | Application ;

Literal       = Scalar | Bool ;
Bool          = 'true' | 'false' ;

TypeAnnotation = '(' ':' Type Expr ')' ;
Nil           = '(' ')' ;

Vector        = '(' 'vec' {Expr} ')' ;
Color         = '(' 'rgb' Expr Expr Expr ')'
              | '(' 'rgba' Expr Expr Expr Expr ')' ;
Cons          = '(' 'cons' Expr Expr ')' ;

Lambda        = '(' 'fn' Param Expr ')' ;
Field         = '(' 'field' Param Expr ')' ;
Param         = Identifier
              | '(' ':' Identifier Type ')' ;

Let           = '(' 'let' Identifier Expr Expr ')' ;

If            = '(' 'if' Expr Expr Expr ')' ;
Fold          = '(' 'fold' Expr Expr Expr ')' ;

Application   = '(' Expr Expr+ ')' ;
```

## Type Annotation Form

The `(: Type expr)` form wraps any expression with a type annotation:

```scheme
; Annotate literals
(: Scalar 5.0)
(: Bool true)
(: (Vec 2) (vec 1.0 2.0))

; Annotate variables
(: Scalar x)

; Annotate complex expressions
(: Scalar (+ x y))

; Annotate parameters (special position)
(fn (: x Scalar) body)
(field (: p (Vec 2)) body)

; Annotate let values
(let x (: Scalar 5.0) body)
```

**Semantics**: `(: Type expr)` evaluates to the value of `expr`, with an assertion/check that it has type `Type`.

## Examples

### Type Annotations (New Syntax)

```scheme
; Types use prefix notation
(: (-> Scalar Scalar) (fn x (* x 2.0)))

; Function types with multiple arguments
(: (-> Scalar Scalar Scalar) (fn x (fn y (+ x y))))

; Field type (just a function type)
(: (-> (Vec 2) Scalar) (field p (length p)))
```

### Lambdas (All Optional Annotations)

```scheme
; Unannotated
(fn x (+ x 1.0))

; Annotated parameter
(fn (: x Scalar) (+ x 1.0))

; Nested functions
(fn (: x Scalar)
  (fn (: y Scalar)
    (+ x y)))

; Can also annotate the whole expression
(: (-> Scalar Scalar)
   (fn x (+ x 1.0)))
```

### Fields (New Semantics)

```scheme
; Unannotated field (dimension inferred/runtime)
(field p (length p))

; Annotated with type (dimension from Vec 2)
(field (: p (Vec 2)) (length p))

; Type: (-> (Vec 2) Scalar) - same as a function!
; Runtime: Value::Field (lazy closure)

; 3D field
(field (: p (Vec 3)) (- (length p) 5.0))
; Type: (-> (Vec 3) Scalar)
```

### Let Bindings (Optional Types)

```scheme
; Unannotated
(let x 5.0
  (+ x 1.0))

; Annotated value
(let x (: Scalar 5.0)
  (+ x 1.0))

; Annotated function
(let double (: (-> Scalar Scalar)
               (fn x (* x 2.0)))
  (double 21.0))
```

### Comparison: Field vs Function

```scheme
; Field (lazy)
(field (: p (Vec 2)) (length p))
; Type: (-> (Vec 2) Scalar)
; Runtime: Value::Field { ... } - not evaluated until sampled

; Function (strict)
(fn (: p (Vec 2)) (length p))
; Type: (-> (Vec 2) Scalar) - same type!
; Runtime: Value::Closure { ... } - evaluates body when called

; The type system doesn't distinguish them
; Only the evaluation strategy differs
```

### Backward Compatibility

For migration, we could support legacy syntax:

```scheme
; Old field syntax (dimension literal)
(field (p 2) body)

; New field syntax (dimension from type)
(field (: p (Vec 2)) body)

; Both mean the same thing
; Old syntax could be deprecated later
```

## Complete Example: Circle SDF

```scheme
; Old style
(fn (radius Scalar)
  (field (p 2)
    (- (length p) radius)))

; New style (unannotated)
(fn radius
  (field p
    (- (length p) radius)))

; New style (fully annotated)
(: (-> Scalar (-> (Vec 2) Scalar))
   (fn (: radius Scalar)
     (field (: p (Vec 2))
       (- (length p) radius))))
```

## Complete Example: CSG Union

```scheme
; Unannotated
(let c1 (field p (- (length p) 5.0))
(let c2 (field p (- (length (vec_sub p (vec 10.0 0.0))) 3.0))
  (field p
    (min (sample c1 p)
         (sample c2 p)))))

; Annotated
(let c1 (: (-> (Vec 2) Scalar)
           (field (: p (Vec 2))
             (- (length p) 5.0)))
(let c2 (: (-> (Vec 2) Scalar)
           (field (: p (Vec 2))
             (- (length (vec_sub p (vec 10.0 0.0))) 3.0)))
  (field (: p (Vec 2))
    (min (sample c1 p)
         (sample c2 p)))))
```

## Type Annotation Requirements

**NONE! All type annotations are optional.**

Previously required:
- ~~Lambda parameters~~: Now optional
- ~~Field dimensions~~: Now optional (from type or inferred)

Now:
- Everything can be unannotated
- Runtime type checking catches errors
- Static type checker (when added) uses annotations when present

## Reserved Keywords

```
; Expression keywords
fn field let if fold cons vec rgb rgba true false

; Type keywords
Scalar Bool Color Vec -> forall

; Special forms
:  ; type annotation marker
```

## Migration Path

### Phase 1: Support Both Syntaxes
```scheme
; Old: (fn (x Scalar) body)
; New: (fn (: x Scalar) body)
; Both work

; Old: (field (p 2) body)
; New: (field (: p (Vec 2)) body)
; Both work
```

### Phase 2: Deprecate Old Syntax
Warnings for:
- `(fn (x Scalar) ...)` → suggest `(fn (: x Scalar) ...)`
- `(field (p 2) ...)` → suggest `(field (: p (Vec 2)) ...)`

### Phase 3: Remove Old Syntax
Only new syntax supported.

## Rationale

### 1. Why S-expression types?
- **Consistency**: Everything uses prefix notation
- **Simplicity**: No need to parse infix operators
- **Extensibility**: Easy to add new type constructors

### 2. Why remove Field type?
- **Laziness is operational**: Type system doesn't need to know
- **Simplicity**: One less concept to explain
- **Flexibility**: Can change evaluation strategy without type changes

### 3. Why all annotations optional?
- **Progressive disclosure**: Beginners don't need types
- **Visual editing**: Types come from UI, not text
- **Flexibility**: Add types when you want static checking

### 4. Why `:` syntax?
- **Explicitness**: Clear what is a type vs value
- **Uniformity**: Same syntax everywhere
- **Familiarity**: Common in Lisp dialects (Common Lisp declarations, Typed Racket)

## Summary

**New syntax is:**
- ✅ More consistent (S-expressions everywhere)
- ✅ Simpler (fewer special cases)
- ✅ More flexible (all annotations optional)
- ✅ More explicit (`:` clearly marks annotations)
- ✅ Future-proof (easy to extend)

**Breaking changes from old syntax:**
- Function types: `(α -> β)` → `(-> α β)`
- Field type: `(Field 2 α)` → `(-> (Vec 2) α)`
- Forall: `(forall a . τ)` → `(forall a τ)`
- Parameters: `(fn (x Scalar) ...)` → `(fn (: x Scalar) ...)`
- Field params: `(field (p 2) ...)` → `(field (: p (Vec 2)) ...)`

**Compatible changes:**
- Type annotations now optional everywhere
- New `(: Type expr)` form for explicit annotations
