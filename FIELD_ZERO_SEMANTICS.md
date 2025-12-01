# Field 0 Semantics: Everything is a Field

## Core Principle

**All values in FieldCalc are fields** `Field n α` for some dimension `n ≥ 0`.

- `Field 0 α` represents **constant fields** (same value everywhere)
- `Field n α` (n > 0) represents **spatial fields** that vary across n-dimensional space

## The Key Insight

`Field 0 α` is the **identity element** of the field system:

```
5.0               ≅  Field 0 Scalar
(vec 1.0 2.0)     ≅  Field 0 (Vec 2)
true              ≅  Field 0 Bool
(rgb 1.0 0.0 0.0) ≅  Field 0 Color
```

At the **type level**, everything is uniformly a field.
At **runtime**, Field 0 values are represented efficiently as direct values (`Value::Scalar`, `Value::Vec`, etc.).

## Type System

```
τ ::= Scalar | Bool | Color | Vec n
    | Field n τ
    | τ → τ
    | ∀α. τ
    | ∀(n : Nat). τ

Field 0 α  <:  Field n α  for all n ≥ 0    (subtyping/lifting)
```

### Lifting Rule

Any `Field 0 α` can be used where `Field n α` is expected (for any n ≥ 0).
The constant value is simply returned at every point in the space.

**Runtime implementation**: Constant fields are created on-demand when operations mix Field 0 and Field n values.

## Operations

### Core Operations (Field 0)

Operations on base types work at Field 0:

```scheme
+  : Field 0 Scalar → Field 0 Scalar → Field 0 Scalar
length : Field 0 (Vec n) → Field 0 Scalar
<  : Field 0 Scalar → Field 0 Scalar → Field 0 Bool
```

**Runtime**: These are just regular value operations (fast, direct computation).

### Field Construction (Field 0 → Field n)

```scheme
field : (p : Vec n) → τ → Field n τ

; Example:
(field (p 2) (length p))  ; Field 2 Scalar
```

Inside a field body, the parameter `p` is bound to a `Vec n` value (which is `Field 0 (Vec n)` at the type level).
The body evaluates to some type `τ` (typically `Field 0 τ`).
The whole construct produces `Field n τ`.

### Sample (Field n → Field 0)

```scheme
sample : Field n α → Field 0 (Vec n) → Field 0 α

; Example:
(sample (field (p 2) (length p)) (vec 3.0 4.0))  ; → Field 0 Scalar (value: 5.0)
```

**Semantics**: Sample evaluates the field at a specific point and returns the result as a constant field (Field 0).

**Type-level view**: `sample :: Field n α → Vec n → α` where `α` is understood as `Field 0 α`
**Runtime**: Returns `Value::Scalar`, `Value::Vec`, etc. (Field 0 representation)

## Examples

### 1. Field Creation and Sampling

```scheme
; Create distance field
(field (p 2) (length p))  ; Field 2 Scalar

; Sample at (3, 4)
(sample field (vec 3.0 4.0))  ; → Field 0 Scalar (value: 5.0)
```

### 2. Field Composition

```scheme
; Offset a field
(let base (field (p 2) (length p))
(let offset (vec 1.0 0.0)
  (field (p 2)
    (sample base (vec_add p offset)))))

; Type analysis:
; base   : Field 2 Scalar
; offset : Field 0 (Vec 2)
; Inside field body:
;   p              : Vec 2 (parameter - Field 0 at type level)
;   vec_add p offset : Field 0 (Vec 2)
;   sample base ... : Field 0 Scalar
; Whole: Field 2 Scalar
```

### 3. CSG via Min

```scheme
(let c1 (field (p 2) (- (length p) 5.0))
(let c2 (field (p 2) (- (length (vec_sub p (vec 10.0 0.0))) 3.0))
  (field (p 2)
    (min (sample c1 p) (sample c2 p)))))

; c1, c2 : Field 2 Scalar
; Inside union field:
;   sample c1 p : Field 0 Scalar
;   sample c2 p : Field 0 Scalar
;   min : Field 0 Scalar → Field 0 Scalar → Field 0 Scalar
; Result: Field 2 Scalar
```

## Why This Design?

### ✅ Advantages

1. **Uniform type system**: Everything is a field
2. **Efficient runtime**: Field 0 is just values (no overhead)
3. **Natural composition**: `sample` returns Field 0, which composes naturally
4. **Clear semantics**: Field bodies work with values, sample extracts values
5. **Matches theory**: Applicative functor structure with `Field 0` as `pure`

### ⚠️ Current Limitations

**Auto-lifting not yet implemented**: Operations don't automatically lift Field 0 → Field n.

Currently, to combine fields, you must explicitly sample:

```scheme
; Want: (+ field 5.0) to work directly
; Currently must: (field (p n) (+ (sample field p) 5.0))
```

**Future enhancement**: Operations could auto-lift to work pointwise:

```scheme
+ : Field n Scalar → Field n Scalar → Field n Scalar
; When given (Field 2, Field 0), lift Field 0 to Field 2
```

## Comparison to Alternative Design

In the earlier `sample :: Field n α → Vec n → α` design, we had:

- **Pro**: Simpler signature
- **Con**: Breaks "everything is a field" conceptually
- **Con**: `α` at type level vs `Field 0 α` at runtime caused confusion

With `sample :: Field n α → Vec n → Field 0 α`:

- **Pro**: Uniform type system (everything is Field)
- **Pro**: No confusion about type vs runtime representation
- **Pro**: Natural composition (Field 0 composes with Field 0)
- **Equivalent**: Runtime representation is the same (both use `Value::Scalar` etc.)

## Implementation Notes

### Runtime Representation

```rust
pub enum Value {
    Scalar(f64),      // Field 0 Scalar
    Vec(Vec<f64>),    // Field 0 (Vec n)
    Bool(bool),       // Field 0 Bool
    Color(r, g, b, a),// Field 0 Color

    // Field n α (n > 0)
    Field {
        param: Var,
        dim: usize,    // n
        body: Expr,
        env: Env,
    },
}
```

### Type-Level vs Runtime

- **Type level**: `Field 0 Scalar`, `Field 2 Scalar`
- **Runtime**: `Value::Scalar`, `Value::Field { ... }`

The type system tracks dimensions, but runtime uses efficient representations.

### Future: Auto-Lifting

To implement auto-lifting:

1. Operations detect mixing of Field 0 and Field n
2. Wrap Field 0 in a constant field closure
3. Apply operation pointwise

```rust
fn builtin_add_lifted(a: Value, b: Value) -> Value {
    match (a, b) {
        (Scalar(x), Scalar(y)) => Scalar(x + y),  // Both Field 0
        (Scalar(x), Field { dim, ... }) => {
            // Lift x to constant Field dim, apply pointwise
            Field { dim, body: (+ x (sample b p)) }
        }
        // ... similar for other cases
    }
}
```

This is deferred to maintain simplicity for now.

## Summary

**Field 0 semantics provide a uniform, mathematically clean foundation** where:

- Everything is a field (type uniformity)
- Constants are Field 0 (efficient runtime)
- `sample` maps Field n → Field 0 (consistent types)
- Operations compose naturally (Field 0 values work together)
- Future auto-lifting will eliminate manual sampling boilerplate

This design matches the Applicative Functor structure from Conal Elliott's Pan system while maintaining runtime efficiency.
