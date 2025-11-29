# Shape Definition File Examples

This directory contains example shape definition files (`.sdf`) that demonstrate the capabilities of the solid modeling tool.

## File Format

Shape definition files use a simple, indented text format:

```
operation [parameters]
  child1
  child2
```

## Primitives

### Circle
```
circle <radius>
```

### Rectangle
```
rectangle <width> <height>
```

### Square
```
square <size>
```

## Transformations

### Translate
```
translate <x> <y>
  <shape>
```

### Rotate
```
rotate <angle_radians>
  <shape>
```

### Scale
```
scale <factor>
  <shape>
```

## CSG Operations

### Union
```
union
  <shape1>
  <shape2>
```

### Smooth Union (Metaballs)
```
smooth-union <smoothness>
  <shape1>
  <shape2>
```

### Intersection
```
intersect
  <shape1>
  <shape2>
```

### Subtraction
```
subtract
  <shape1>
  <shape2>
```

## Examples

- `circle.sdf` - Simple circle
- `union.sdf` - Two circles joined together
- `smooth_union.sdf` - Two circles smoothly blended (metaballs)
- `donut.sdf` - Ring shape created by subtracting circles
- `face.sdf` - Complex smiley face composition

## Usage

To view these examples in the viewer, you would load them via command line:

```bash
cargo run --release --bin solid_viewer examples/circle.sdf
```

(Note: File loading functionality would need to be implemented)
