# Sample Semantics: Field-Returning vs Value-Returning

Exploring whether `sample` should return a constant field or break the "everything is a field" abstraction.

## Proposed Signature

```
sample :: Field n α → Vec n → Field n α
```

Where the result is a **constant field** (same value everywhere) containing the sampled value.

---

## Example 1: Basic Sampling (✓ Works)

```scheme
(let f (field (p 2) (length p))
  (sample f (vec 3.0 4.0)))

; f : Field 2 Scalar
; sample f (vec 3.0 4.0) : Field 2 Scalar (constant everywhere = 5.0)
; Semantics: λp. 5.0
```

**Analysis**: Clean and consistent. The result is a field like any other.

---

## Example 2: Using Sampled Value (✓ Works)

```scheme
(let f (field (p 2) (length p))
(let sampled (sample f (vec 3.0 4.0))
  (+ sampled 10.0)))

; sampled : Field 2 Scalar (constant 5.0)
; 10.0 auto-lifts to Field 2 Scalar (constant 10.0)
; + operates pointwise: λp. sampled(p) + 10.0 = λp. 5.0 + 10.0 = λp. 15.0
; Result: Field 2 Scalar (constant 15.0)
```

**Analysis**: Works perfectly! Sampled value composes like any other field.

---

## Example 3: Field Composition (❌ Problem!)

```scheme
; Warp texture by displacement field
(let texture (field (p 2) (checkerboard p))
(let warp (field (p 2) (* 0.1 (vec (sin (* 10.0 (index p 1)))
                                     (cos (* 10.0 (index p 0))))))
  ; Want: λp. texture(p + warp(p))

  (field (p 2)
    (sample texture (vec_add p (sample warp p))))))

; Let's trace the types inside the field body:
; p : Vec 2 (the position parameter - NOT a field)
; warp : Field 2 (Vec 2)
; (sample warp p) : Field 2 (Vec 2) - constant field
;
; Now: (vec_add p (sample warp p))
; p : Vec 2
; (sample warp p) : Field 2 (Vec 2)
;
; Problem: vec_add expects Vec 2 → Vec 2 → Vec 2
; We have Vec 2 and Field 2 (Vec 2)
;
; Option A: Auto-lift p to Field 2 (Vec 2)?
;   But p is not a constant - it's the parameter we're binding!
;   We can't turn the parameter into a constant field.
;
; Option B: Auto-unwrap (sample warp p) to Vec 2?
;   But that breaks "everything is a field" - we're extracting values.
;   Also, which Field 2 values unwrap? Only constant ones? Complex!
;
; Option C: vec_add should lift to work on fields?
;   vec_add : Vec 2 → Vec 2 → Vec 2
;   lifts to: Field 2 (Vec 2) → Field 2 (Vec 2) → Field 2 (Vec 2)
;   But then we need p to be a field... we're back to Option A
```

**Analysis**: **Fundamental problem**. Inside a field body, the parameter `p` is a **value** (Vec 2), not a field. When we sample and get a constant field back, we can't use it with the parameter without some unwrapping mechanism.

---

## Example 4: The Unwrapping Dilemma

To make Example 3 work, we'd need something like:

```scheme
; Option A: Explicit unwrap operation
(field (p 2)
  (sample texture (vec_add p (unwrap-constant (sample warp p)))))

; unwrap-constant :: Field n α → α  (only works for constant fields!)

; Option B: Context-sensitive auto-unwrapping
(field (p 2)
  (sample texture (vec_add p (sample warp p))))
  ; The system detects vec_add needs Vec 2 → Vec 2 → Vec 2
  ; Auto-unwraps the constant field

; Option C: Two different sample operations
(field (p 2)
  (sample texture (vec_add p (sample-value warp p))))

; sample-value :: Field n α → Vec n → α  (returns value, not field)
```

**Analysis**: All three options break the "everything is a field" abstraction in different ways:
- Option A: Explicit unwrapping (honest but verbose)
- Option B: Magic auto-unwrapping (implicit, potentially confusing)
- Option C: Two operations with different semantics (honest, clear)

---

## Example 5: Nested Fields - Field n (Field m α)

```scheme
; A field of fields: for each 2D point, we have a 3D field
(let ff (field (p 2)
          (field (q 3)
            (+ (length p) (length q))))
  ; ff : Field 2 (Field 3 Scalar)

  (sample ff (vec 1.0 2.0)))

; sample :: Field 2 (Field 3 Scalar) → Vec 2 → Field 2 (Field 3 Scalar)
; Instantiates: α = Field 3 Scalar, n = 2
;
; Result: Constant Field 2 (Field 3 Scalar)
; The constant value at every 2D point is:
;   (field (q 3) (+ (length (vec 1.0 2.0)) (length q)))
; = (field (q 3) (+ 2.236... (length q)))
;
; This is semantically well-defined!
```

Now if we want to sample the inner field:

```scheme
(let outer (sample ff (vec 1.0 2.0))
  ; outer : Field 2 (Field 3 Scalar) - constant field

  (sample outer (vec 3.0 4.0 5.0)))

; sample :: Field 2 (Field 3 Scalar) → Vec 2 → Field 2 (Field 3 Scalar)
;                                       ^^^^^
; Wait! We're passing a Vec 3, but sample expects Vec 2!
;
; The signature says: sample a Field 2 at a 2D position
; But we want to sample the Field 3 Scalar that's stored IN the Field 2
```

**The issue**: With `sample :: Field n α → Vec n → Field n α`, we always sample in the **outer field's dimension**, not the inner value's dimension.

To sample the inner field, we'd need:

```scheme
(let outer-sampled (sample ff (vec 1.0 2.0))
  ; outer-sampled : Field 2 (Field 3 Scalar) - constant

  (let inner (unwrap-constant outer-sampled)
    ; inner : Field 3 Scalar

    (sample inner (vec 3.0 4.0 5.0))))
    ; → Field 3 Scalar (constant = 2.236... + 7.07... ≈ 9.3)
```

**Analysis**: Nested fields are **semantically well-defined**, but practically awkward. You need unwrapping to access inner fields. This is not just implementation complexity - it's a **semantic requirement** that sampling operates at a specific dimension level.

---

## Example 6: Computing with Parameter vs Sampled Field

```scheme
; Inside a field body
(field (p 2)
  (let dist-from-origin (length p)
  (let dist-from-another-field (sample other-dist-field p)
    ; Problem: different types!
    ; dist-from-origin : Scalar (value)
    ; dist-from-another-field : Field 2 Scalar (constant field)

    ; Want to compare them:
    (< dist-from-origin dist-from-another-field)

    ; < : Scalar → Scalar → Bool
    ; But we have Scalar and Field 2 Scalar
    ;
    ; Option A: Auto-unwrap constant field
    ; Option B: Auto-lift scalar to constant field, then pointwise <
    ;   < : Field 2 Scalar → Field 2 Scalar → Field 2 Bool
    ;   Result: Field 2 Bool (constant)
    ;
    ; But we're inside a field body! We want a Bool value, not a Field 2 Bool!
    )))
```

**Analysis**: Inside a field body, you're working with **values at points**, not fields. If sample returns a field, you have a **type mismatch** between parameters (values) and sampled results (fields).

---

## Summary: The Core Tension

The proposed `sample :: Field n α → Vec n → Field n α` has one fundamental problem:

**Inside field bodies, parameters are VALUES, not fields.**

```scheme
(field (p 2)      ; p : Vec 2 (value)
  (... p ...))    ; working with values

; But if sample returns fields:
(field (p 2)
  (sample f p))   ; Returns Field 2 α, but we're in a value context!
```

This creates constant friction between:
1. The field abstraction (everything is a field)
2. The reality of field bodies (working with values at points)

---

## Alternative Approaches

### Approach 1: Two Sample Operations (Honest)

```scheme
sample-value :: Field n α → Vec n → α           ; extracts value
sample-const :: Field n α → Vec n → Field n α   ; produces constant field
```

- Clear semantics: choose based on context
- Honest about abstraction levels
- More verbose

### Approach 2: Sample Returns Values (Pragmatic)

```scheme
sample :: Field n α → Vec n → α
```

- Field composition works naturally
- Honest about abstraction break
- Constants auto-lift when needed
- This is what most systems do (Pan, Fran, etc.)

### Approach 3: Context-Sensitive Unwrapping (Magic)

```scheme
sample :: Field n α → Vec n → Field n α

; But constant fields auto-unwrap in "value contexts"
```

- Requires sophisticated type system
- Potentially confusing behavior
- Hard to predict when unwrapping happens
- Implementation complexity AND semantic complexity

---

## Recommendation

**Use Approach 2**: `sample :: Field n α → Vec n → α`

**Rationale:**
1. **Field bodies work with values** - this is fundamental to the abstraction
2. **Composition becomes natural** - sampled values can be used directly
3. **Auto-lifting handles the reverse** - when you use a value where a field is expected, it lifts to constant
4. **Honest about abstraction levels** - sampling explicitly moves from field-level to value-level
5. **Precedent**: Conal Elliott's Pan uses this approach

**The "everything is a field" abstraction is mostly maintained:**
- User code operates on fields via pointwise operations
- Sampling is explicitly marked as a meta-operation
- Values automatically lift back to fields when needed

**For visual editing:**
- Most nodes work with fields (pointwise operations)
- Sample nodes are clearly marked as "extraction" operations
- The UI can show the type transition visually

---

## Conclusion

Making sample return a constant field is **semantically definable** but **practically problematic**. The issue is not just implementation complexity - it's a fundamental mismatch between:

- Field bodies operating on values at points
- Sampled results being fields

You'd need unwrapping operations anyway, which defeats the purity of "everything is a field."

Better to be honest: sampling is a meta-operation that extracts values from fields. Values then auto-lift to constants when used in field contexts.
