# Solid Modeling - Implicit Function 2D/3D Tool

A high-performance solid modeling tool based on implicit function representations, built with Rust and wgpu for cross-platform support.

## Features

### Current (2D)
- **Implicit Function Framework**: Clean trait-based system for defining signed distance fields
- **Primitive Shapes**: Circle, Rectangle, and more
- **CSG Operations**: Union, Intersection, Difference with smooth variants
- **Transformations**: Translate, Rotate, Scale
- **Fluent DSL**: Easy-to-use builder pattern for composing shapes
- **GPU Rendering**: wgpu-based renderer for cross-platform support

### Planned (3D)
- 3D primitives: Sphere, Box, Torus, Cylinder, Capsule, Plane
- 3D CSG operations
- Ray marching renderer
- Marching cubes mesh extraction
- Export to STL/OBJ formats

## Installation

```bash
cargo build --release
```

## Usage

### Running the Viewer

```bash
cargo run --release --bin solid_viewer
```

**Controls:**
- `SPACE` - Cycle through demo scenes
- `1-5` - Jump to specific scene
- `ESC` - Quit

### Using the Library

```rust
use solid_modeling::prelude::*;

// Create shapes using the fluent DSL
let shape = circle(2.0)
    .translate(-1.0, 0.0)
    .smooth_union(
        circle(2.0).translate(1.0, 0.0),
        0.8  // smoothness factor
    );

// Evaluate at a point
let distance = shape.evaluate(Vec2::new(0.0, 0.0));

// Render to pixels
let evaluator = Evaluator2D::new(800, 800);
let pixels = evaluator.evaluate(&shape);
```

## DSL Examples

### Simple Shapes

```rust
// Circle with radius 1.0
let c = circle(1.0);

// Rectangle 2.0 x 3.0
let r = rectangle(2.0, 3.0);

// Square 2.0 x 2.0
let s = square(2.0);
```

### Transformations

```rust
// Translate a circle
let shape = circle(1.0).translate(2.0, 3.0);

// Rotate a rectangle (angle in radians)
let shape = rectangle(2.0, 1.0).rotate(std::f32::consts::PI / 4.0);

// Scale a shape
let shape = circle(1.0).scale(2.0);
```

### CSG Operations

```rust
// Union (OR)
let shape = circle(1.0)
    .translate(-1.0, 0.0)
    .union(circle(1.0).translate(1.0, 0.0));

// Intersection (AND)
let shape = circle(1.5)
    .intersect(rectangle(2.0, 2.0));

// Difference (subtraction)
let donut = circle(2.0).subtract(circle(1.0));

// Smooth union (metaballs effect)
let shape = circle(1.0)
    .translate(-1.0, 0.0)
    .smooth_union(circle(1.0).translate(1.0, 0.0), 0.5);
```

### Complex Compositions

```rust
// Smiley face
let body = circle(2.0);
let eye_left = circle(0.3).translate(-0.7, 0.7);
let eye_right = circle(0.3).translate(0.7, 0.7);
let mouth = rectangle(1.2, 0.3)
    .translate(0.0, -0.5)
    .intersect(circle(1.8).translate(0.0, -1.0));

let face = body
    .subtract(eye_left)
    .subtract(eye_right)
    .subtract(mouth);
```

## Architecture

### Core Modules

- **`implicit`**: Core trait definitions for 2D and 3D implicit functions
- **`primitives`**: Built-in primitive shapes (circles, rectangles, spheres, boxes, etc.)
- **`csg`**: Constructive Solid Geometry operations (union, intersection, difference)
- **`transform`**: Transformation operations (translate, rotate, scale)
- **`dsl`**: Fluent builder API for easy shape composition
- **`evaluator`**: CPU-side evaluation and pixel generation
- **`renderer`**: wgpu-based GPU rendering

### Key Concepts

**Signed Distance Fields (SDF)**: Each shape is represented as a function that returns:
- Negative values inside the shape
- Zero at the boundary
- Positive values outside the shape

This representation enables:
- Easy CSG operations
- Smooth blending
- Efficient collision detection
- GPU-accelerated rendering

## Dependencies

- `wgpu` - Cross-platform graphics API
- `winit` - Window creation and event handling
- `glam` - Mathematics library for vectors and matrices
- `bytemuck` - Safe type casting for GPU buffers
- `pollster` - Async runtime for initialization

## Future Enhancements

- [ ] Interactive camera controls (pan, zoom)
- [ ] Real-time shape editing
- [ ] 3D ray marching renderer
- [ ] Marching cubes implementation
- [ ] Export to mesh formats (STL, OBJ)
- [ ] Custom user-defined implicit functions
- [ ] Animation support
- [ ] GPU compute shaders for evaluation
- [ ] Multi-material support
- [ ] Texture mapping

## License

MIT
