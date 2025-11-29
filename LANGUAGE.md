# FieldCalc: Language Design and Implementation

## Overview

FieldCalc is a simply-typed lambda calculus with parametric polymorphism for defining implicit solid models. It takes inspiration from Conal Elliott's Pan system and uses denotational semantics where fields are first-class functions.

## Reference Implementation

### What We Built

**Location**: `src/lang/`

**Components**:
- `types.rs` - Type system (Scalar, Vec n, Color, Field n α, etc.)
- `ast.rs` - Abstract syntax tree (expressions, not syntax!)
- `env.rs` - Runtime values and evaluation environment
- `eval.rs` - Strict evaluator with lazy fields
- `builtins.rs` - Built-in operations (scalar math, vectors, etc.)

**Key Design Decisions**:

1. **Strict evaluation** everywhere except fields
   - Fields are closures that only evaluate when sampled
   - This is **critical** for performance

2. **Curried built-ins**
   - `+` is really `λa. λb. a + b`
   - Enables partial application naturally

3. **Type erasure at runtime**
   - Types are for static checking only
   - Dimension polymorphism erased

4. **Structural recursion only**
   - `fold` is the only recursion primitive
   - Guarantees termination

## Examples

```rust
// Simple arithmetic
(+ 2.0 3.0) → 5.0

// Lambda function
(λx. x + 1.0) 5.0 → 6.0

// Field construction (NOT evaluated until sampled!)
field (p : Vec 2) ↦ length p

// Circle SDF
circle = λr. field (p : Vec 2) ↦ length p − r

// Sampling
sample (circle 2.0) ⟨3.0, 0.0⟩ → 1.0
```

---

## Practical Implementation Recommendations

### Phase 1: Parser + Type Checker (Next)

**Goal**: Let users write actual code, not Rust AST construction

```
src/lang/
  parser.rs        - Text → AST (use nom or pest)
  typechecker.rs   - AST → Typed AST (Hindley-Milner with extensions)
  pretty.rs        - AST → Text (for debugging)
```

**Parser priorities**:
1. Infix operators: `2 + 3` instead of `(+ 2 3)`
2. Let bindings: `let x = 5 in x + 1`
3. Field syntax: `field p => length p` (shorter!)
4. Type annotations: optional but helpful

**Example syntax**:
```haskell
-- Simple circle
circle r = field p => length p - r

-- Union of two circles
two_circles =
  let c1 = translate (vec2 -1 0) (circle 1.5)
      c2 = translate (vec2 1 0) (circle 1.5)
  in union c1 c2
```

### Phase 2: Optimization (Critical for Performance)

The reference evaluator is **very slow** because:
1. Tree-walking interpreter
2. Closure allocation on every field sample
3. No inlining or specialization

**Optimization Strategy: Compile to Bytecode**

```rust
src/lang/
  bytecode.rs      - Bytecode instruction set
  compiler.rs      - AST → Bytecode
  vm.rs            - Bytecode VM (register-based)
```

**Why bytecode**:
- 10-100x faster than tree-walking
- Still portable (no LLVM needed)
- Easy to debug
- Can JIT later if needed

**Bytecode instructions** (sketch):
```rust
enum Instruction {
    // Stack operations
    LoadConst(f64),
    LoadVar(usize),      // Variable index
    Store(usize),

    // Arithmetic
    Add, Sub, Mul, Div,

    // Vector ops
    VecLength,
    VecDot,
    VecAdd,

    // Field sampling
    SampleField(usize),  // Field ID

    // Control flow
    Jump(usize),
    JumpIf(usize),
    Call(usize),         // Function call
    Return,
}
```

**Compilation example**:
```haskell
-- Source
λx. x + 1.0

-- Bytecode
0: LoadVar 0      // x
1: LoadConst 1.0
2: Add
3: Return
```

### Phase 3: GPU Acceleration (For Real Performance)

**Problem**: Evaluating fields pixel-by-pixel on CPU is slow

**Solution**: Compile fields to GPU shaders (WGSL/GLSL)

```rust
src/lang/
  codegen_wgsl.rs  - AST → WGSL shader code
  gpu_cache.rs     - Cache compiled shaders
```

**Architecture**:
```
User Code (FieldCalc)
    ↓
AST (type-checked)
    ↓
WGSL Shader (one per field)
    ↓
wgpu Pipeline
    ↓
GPU Evaluation (parallel!)
```

**Example codegen**:
```haskell
-- FieldCalc
circle 2.0

-- Generated WGSL
@fragment
fn evaluate_field(p: vec2<f32>) -> f32 {
    return length(p) - 2.0;
}
```

**Challenges**:
1. Not all operations map to WGSL (e.g., fold)
2. Need to detect "GPU-compatible" subset
3. Fallback to CPU for complex fields

**Hybrid approach**:
- Primitives → GPU shaders
- CSG operations → GPU (min/max)
- Complex logic → CPU evaluation
- User-defined fold → CPU only

### Phase 4: Automatic Differentiation

**Why**: For computing normals, curvature, etc.

**Approaches**:

**A. Forward-mode AD (simpler)**
```rust
// Dual numbers: (value, derivative)
struct Dual { val: f64, deriv: f64 }

impl Add for Dual {
    fn add(self, other: Dual) -> Dual {
        Dual {
            val: self.val + other.val,
            deriv: self.deriv + other.deriv,  // Chain rule!
        }
    }
}
```

Advantage: Easy to implement
Disadvantage: Need n passes for ∇f (one per dimension)

**B. Reverse-mode AD (harder but better)**
- Build computation graph
- Backpropagate gradients
- Get all derivatives in one pass

Advantage: Fast for f: ℝⁿ → ℝ (our case!)
Disadvantage: More complex implementation

**Recommendation**: Start with forward-mode, add reverse-mode later

**Implementation**:
```rust
src/lang/
  ad.rs            - Automatic differentiation transform
  dual.rs          - Dual number arithmetic
```

### Phase 5: Visual Editor Integration

**Key insight**: The AST IS the graph!

```rust
// Each Expr node → Visual node
match expr {
    Expr::Builtin("+") => AddNode { inputs: 2 },
    Expr::Field { param, body, .. } => FieldNode {
        input_dim: dim,
        body: body_graph,
    },
    Expr::App { func, arg } => ConnectionEdge {
        from: func,
        to: arg,
    },
}
```

**Node representation**:
```rust
struct VisualNode {
    id: NodeId,
    expr: Expr,              // The actual code!
    position: (f32, f32),    // Canvas position
    inputs: Vec<PortId>,
    outputs: Vec<PortId>,
}

struct Port {
    id: PortId,
    ty: Type,                // For type-checking connections
    connected: Option<PortId>,
}
```

**Workflow**:
1. User drags nodes → Creates AST
2. User connects ports → Creates App nodes
3. User edits values → Updates Expr literals
4. Evaluate → Runs our VM/GPU pipeline

**Serialization**:
```rust
// Save as JSON (for portability)
{
  "nodes": [...],
  "edges": [...],
  "types": {...}
}

// OR save as FieldCalc source code (human-readable!)
let my_shape = union (circle 1) (sphere 2)
```

---

## Architectural Recommendations

### Layered Architecture

```
┌─────────────────────────────────────┐
│   Visual Editor (egui/iced)        │  ← User interaction
├─────────────────────────────────────┤
│   Parser / Pretty Printer           │  ← Text ↔ AST
├─────────────────────────────────────┤
│   Type Checker                      │  ← Static analysis
├─────────────────────────────────────┤
│   AST (Core language)               │  ← Shared IR
├─────────────────────────────────────┤
│   ┌─────────┬─────────┬──────────┐ │
│   │ Interp. │   VM    │   GPU    │ │  ← Multiple backends
│   └─────────┴─────────┴──────────┘ │
├─────────────────────────────────────┤
│   Automatic Differentiation         │  ← Transform layer
├─────────────────────────────────────┤
│   Rendering (wgpu)                  │  ← Display
└─────────────────────────────────────┘
```

### Performance Targets

Based on typical use:

| Operation | Target | Method |
|-----------|--------|--------|
| Parse text | <100ms | Nom parser |
| Type check | <50ms | Bidirectional |
| Compile to bytecode | <100ms | Single pass |
| Evaluate field (CPU, 1px) | <1µs | VM |
| Render 800x800 (CPU) | <50ms | Parallel + VM |
| Render 800x800 (GPU) | <5ms | WGSL shader |
| Compute gradient | <2x field eval | Forward AD |

### Data Flow Example

```
User types: "circle 2"
    ↓
Parser: Text → AST
    Expr::App(Expr::Var("circle"), Expr::Scalar(2.0))
    ↓
Type Checker: AST → Typed AST
    (circle : Scalar → SDF 2)
    Application type checks ✓
    ↓
Compiler: AST → Bytecode
    [LoadConst 2.0, LoadGlobal "circle", Call 1, Return]
    ↓
Visual Editor: AST → Graph
    [CircleNode(radius_port) ← ConstNode(2.0)]
    ↓
Renderer: Evaluate per pixel
    GPU path:  Compile to WGSL → wgpu pipeline → 5ms
    CPU path:  VM execution → 50ms
    ↓
Display: Texture → Screen
```

---

## Implementation Timeline

**Week 1-2**: Parser + Type Checker
- Write parser for basic syntax
- Implement type inference
- Add error messages

**Week 3-4**: Bytecode VM
- Design instruction set
- Write compiler
- Benchmark vs interpreter

**Week 5-6**: GPU Code generation
- WGSL codegen for primitives
- Shader caching
- Hybrid CPU/GPU dispatch

**Week 7-8**: Visual Editor
- Node graph from AST
- Interactive editing
- Serialization

**Week 9-10**: Automatic Differentiation
- Forward-mode AD
- Gradient visualization
- Normal field computation

**Week 11-12**: Polish
- Error messages
- Documentation
- Example gallery

---

## Critical Design Choices

### 1. Lazy vs Strict Fields

**Current (Correct)**: Fields are closures, lazy until sampled
```rust
Value::Field { param, dim, body, env }
// NOT evaluated until sample_field() is called
```

**Why this matters**:
- Transformations compose without evaluation
- `translate (circle 1)` doesn't evaluate the circle
- Only evaluate at render time

### 2. Type Erasure vs Dependent Types

**Current (Simpler)**: Erase types at runtime
- Dimension `n` is runtime value
- Type variables don't exist at runtime

**Alternative**: Dependent types (dimension as type-level nat)
- More type safety
- Much more complex implementation
- May revisit later

**Recommendation**: Stick with type erasure for now

### 3. Pure vs Memoized Evaluation

**Current (Pure)**: Re-evaluate every time
```rust
sample field point → eval body with point
```

**Alternative**: Memoization cache
```rust
if cache.contains(point) {
    return cache[point]
} else {
    let result = eval(...)
    cache.insert(point, result)
    return result
}
```

**Recommendation**: Add memoization as optimization later
- Profile first
- Most fields aren't sampled at same point multiple times
- GPU evaluation doesn't benefit from memoization

### 4. Single-Pass vs Multi-Pass Type Checking

**Current**: Need to implement

**Options**:
- **Bidirectional**: Type checking + inference
- **Constraint-based**: Generate constraints, solve
- **Hindley-Milner**: Classic ML-style

**Recommendation**: Bidirectional type checking
- Simpler than constraint-based
- More explicit than HM (user provides some types)
- Natural for parametric polymorphism

---

## Next Steps

1. **Implement parser**
   - Start with S-expression syntax (easier to parse)
   - Add infix operators later

2. **Add type checker**
   - Bidirectional type checking
   - Error messages with source locations

3. **Build bytecode VM**
   - Design instruction set
   - Write compiler pass
   - Benchmark

4. **Test with real examples**
   - Port current Rust SDF code to FieldCalc
   - Compare performance
   - Iterate on design

Would you like me to start on any of these phases?
