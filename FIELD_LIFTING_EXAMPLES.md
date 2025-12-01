# Field Lifting Examples: Everything is a Field

Exploring the "radical" approach where all values are fields and operations auto-lift to work pointwise.

## Core Principle

```
Scalar(x)     ≅  Field 0 Scalar     -- constant field
Vec(3,4)      ≅  Field 0 (Vec 2)    -- constant vector
Bool(true)    ≅  Field 0 Bool       -- constant boolean
Field 2 α     =  ℝ² → α             -- spatial field (depends on position)
```

All operations lift pointwise: `f : α → β` becomes `f : Field n α → Field n β`

---

## Example 1: Basic SDF (✓ Works Well)

```scheme
; Circle at origin, radius 5
(field (p 2)
  (- (length p) 5.0))
; → Field 2 Scalar

; Inside field body, p is Vec 2, operations are normal
; length : Vec 2 → Scalar
; - : Scalar → Scalar → Scalar
; All composed normally
```

**Analysis**: No issues. Field construct introduces position parameter, inside is normal value code.

---

## Example 2: CSG Operations (✓ Works Well)

```scheme
; Two circles
(let c1 (field (p 2) (- (length p) 5.0))
(let c2 (field (p 2) (- (length (vec_sub p (vec 10.0 0.0))) 3.0))

  ; Union: min auto-lifts!
  (min c1 c2)))

; min : Scalar → Scalar → Scalar
; auto-lifts to: min : Field 2 Scalar → Field 2 Scalar → Field 2 Scalar
; Semantics: λp. min (c1 p) (c2 p)
; → Field 2 Scalar
```

**Analysis**: Beautiful! No special CSG operators needed. Standard min/max just work.

---

## Example 3: Offset/Inflate (✓ Works Well)

```scheme
; Inflate circle by 2 units
(let circle (field (p 2) (- (length p) 5.0))
  (- circle 2.0))

; The 2.0 is Field 0 Scalar (constant)
; - auto-lifts to: λp. circle(p) - 2.0
; → Field 2 Scalar with radius 7
```

**Analysis**: Mixing fields and constants is seamless. Constants auto-lift.

---

## Example 4: Conditional Coloring (✓ Works Well)

```scheme
; Red inside circle, blue outside
(let dist (field (p 2) (- (length p) 10.0))
  (if (< dist 0.0)
      (rgb 1.0 0.0 0.0)
      (rgb 0.0 0.0 1.0)))

; Breakdown:
; dist : Field 2 Scalar
; 0.0 : Field 0 Scalar (constant)
; < auto-lifts to: Field 2 Bool
; (rgb ...) : Field 0 Color (constant colors)
; if auto-lifts to: Field 2 Color
; Semantics: λp. if dist(p) < 0.0 then red else blue
```

**Analysis**: Perfect! Conditionals work pointwise naturally.

---

## Example 5: Smooth Blending (✓ Works Well)

```scheme
; Smooth union with k=2.0
(let c1 (field (p 2) (- (length p) 5.0))
(let c2 (field (p 2) (- (length (vec_sub p (vec 10.0 0.0))) 3.0))
(let k 2.0
  (let h (max 0.0 (- k (abs (- c1 c2))))
    (- (min c1 c2) (* (* h h) (/ 1.0 (* 4.0 k))))))))

; Every operation (-, abs, min, *, /) auto-lifts
; All work pointwise on Field 2 Scalar
; Result: Field 2 Scalar
```

**Analysis**: Complex formula, but all operations just lift. Works beautifully.

---

## Example 6: Gradient Field (⚠️ Ambiguity)

```scheme
; Problem: What does this mean?
(let vfield (field (p 2) p)  ; identity field: returns position
  (length vfield))

; Interpretation 1: length of the field itself (doesn't make sense)
; Interpretation 2: pointwise length - λp. length(vfield(p)) = λp. length(p)
;
; Must be interpretation 2, but this is subtle!
; We're lifting: length : Vec 2 → Scalar
; to: length : Field 2 (Vec 2) → Field 2 Scalar
```

**Analysis**: Semantically clear (must be pointwise), but conceptually confusing. Are we taking the length of "the field" or "the value at each point"?

---

## Example 7: Sampling (⚠️ Breaks the Abstraction)

```scheme
; We need to get concrete values somewhere!
; For rendering, we sample the field at pixel positions

; Option A: sample returns a base value
(sample (field (p 2) (length p)) (vec 3.0 4.0))
; → Scalar(5.0)  [NOT a field]

; Option B: sample returns Field 0 α
(sample (field (p 2) (length p)) (vec 3.0 4.0))
; → Field 0 Scalar [constant field of 5.0]

; Problem: Either way, 'sample' is not a pointwise operation!
; sample : Field n α → Vec n → ???
; It's a meta-level operation that extracts values
```

**Analysis**: Sampling necessarily breaks the "everything is a field" abstraction. We need values at some point.

---

## Example 8: Custom Algorithm - Find Zero Set (❌ Gets in the Way)

```scheme
; Find points where field crosses zero (the boundary)
; This requires iterating over positions and testing

(let sdf (field (p 2) (- (length p) 5.0))
  ; Pseudo-code for marching squares:
  (for-each grid_cell
    (let corners (map (fn (offset)
                        (sample sdf (vec_add base_pos offset)))
                      corner_offsets)
      ; Check if sign changes
      (if (has-sign-change corners)
          (triangulate-cell corners)
          nil))))

; Problems:
; 1. Need explicit sampling - breaks abstraction
; 2. for-each is not pointwise - it's a reduction
; 3. Working with lists of samples, not fields
; 4. The "field" abstraction doesn't help here
```

**Analysis**: For algorithms that need to inspect field structure (sampling, optimization, root-finding), the field abstraction gets in the way. You need to break out to explicit sampling.

---

## Example 9: Texture Warping (⚠️ Requires Meta-Level Operations)

```scheme
; Warp a texture by a displacement field
(let texture (field (p 2) (checkerboard p))
(let warp (field (p 2) (* 0.1 (vec (sin (* 10.0 (index p 1)))
                                     (cos (* 10.0 (index p 0))))))
  ; Want: λp. texture(p + warp(p))
  ; But with auto-lifting, how do we express this?

  ; Attempt 1: Can't add fields of different types
  ; (+ p warp)  -- p is Vec 2, warp is Field 2 (Vec 2)

  ; Attempt 2: Need to work inside a field
  (field (p 2)
    (sample texture (vec_add p (sample warp p))))))

; The inner 'sample' operations are necessary but awkward
; We're manually controlling evaluation, not using auto-lift
```

**Analysis**: Composition of fields (using one field to index another) requires explicit sampling. Auto-lifting doesn't help.

---

## Example 10: Animation / Time (⚠️ Dimension Explosion?)

```scheme
; Animate a circle growing over time
; Option A: Time as another dimension
(field (pt 3)  ; pt is Vec 3: (x, y, time)
  (let p (vec (index pt 0) (index pt 1))
  (let t (index pt 2)
    (- (length p) t))))  ; radius grows with time
; → Field 3 Scalar

; To render at time=5:
(sample-at-time animated_sdf 5.0)
; → Field 2 Scalar

; Problem: Everything becomes Field 3! Extra dimension everywhere.

; Option B: Time as a parameter (not a field dimension)
(fn (t Scalar)
  (field (p 2)
    (- (length p) t)))
; → Scalar → Field 2 Scalar

; This is cleaner but now time is NOT part of the field abstraction
```

**Analysis**: Adding extra dimensions (time, etc.) inflates complexity. Better to keep them as separate parameters, but then they're not "fields."

---

## Example 11: Reduction - Bounding Box (❌ Doesn't Fit)

```scheme
; Compute bounding box of a field (where |sdf| < epsilon)
(let sdf (field (p 2) (complex_shape p))
  ; Need to find min/max coordinates where abs(sdf(p)) < epsilon
  ; This is a reduction over space, not a pointwise operation

  (fold (fn (bbox pt)
          (if (< (abs (sample sdf pt)) 0.01)
              (expand-bbox bbox pt)
              bbox))
        empty-bbox
        sample-grid))

; Problems:
; 1. Explicit sampling grid needed
; 2. fold is a reduction, not pointwise
; 3. Result is a BBox, not a field
; 4. The field abstraction provides no help
```

**Analysis**: Spatial reductions (integration, optimization, statistics) don't fit the pointwise model.

---

## Summary: When It Works vs When It Doesn't

### ✅ Works Beautifully:
- CSG operations (min, max, smooth blend)
- Arithmetic transforms (offset, scale)
- Pointwise color/material assignment
- Conditionals based on field values
- Combining multiple fields algebraically

### ⚠️ Subtle/Awkward:
- Sampling (must break abstraction to get values)
- Field composition (warping, one field indexing another)
- Gradients/derivatives (change type)
- Extra dimensions (time, etc.)

### ❌ Gets in the Way:
- Spatial reductions (bounding boxes, integration)
- Root finding / optimization
- Iterative algorithms (marching cubes, ray marching)
- Custom sampling patterns
- Non-pointwise transformations

---

## Recommendation

**Hybrid approach:**

1. **Default to field lifting for user-level code**
   - Makes interactive editing intuitive
   - CSG and artistic operations "just work"

2. **Provide explicit primitives for meta-level operations**
   ```scheme
   sample : Field n α → Vec n → α
   gradient : Field n Scalar → Field n (Vec n)
   fold-over-region : (β → Vec n → α → β) → β → Field n α → Region → β
   ```

3. **Visual editor generates mostly pointwise code**
   - Node graph connections = field composition
   - Sampling/reduction hidden in rendering nodes

4. **Advanced users can break abstraction when needed**
   - For custom algorithms
   - When performance matters
   - When field model doesn't fit

This gives you the elegance of "everything is a field" for 90% of use cases, while acknowledging that some operations are fundamentally different.

**The ambiguity issue** is real but manageable: document clearly that operations lift pointwise, and meta-operations (sample, gradient) are explicitly named.
