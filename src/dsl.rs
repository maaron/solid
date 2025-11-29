/// DSL module for easy composition of 2D implicit shapes
use crate::implicit::{ImplicitFunction2D, ImplicitObject2D};
use crate::primitives::*;
use crate::csg::*;
use crate::transform::*;
use crate::Vec2;

/// Builder trait for fluent API
pub trait ShapeBuilder2D: ImplicitFunction2D + Sized + 'static {
    /// Translate this shape by an offset
    fn translate(self, x: f32, y: f32) -> Translate2D<Self> {
        Translate2D::new(self, Vec2::new(x, y))
    }

    /// Rotate this shape by an angle (in radians)
    fn rotate(self, angle: f32) -> Rotate2D<Self> {
        Rotate2D::new(self, angle)
    }

    /// Scale this shape uniformly
    fn scale(self, scale: f32) -> Scale2D<Self> {
        Scale2D::new(self, scale)
    }

    /// Union with another shape
    fn union<B: ImplicitFunction2D>(self, other: B) -> Union2D<Self, B> {
        Union2D::new(self, other)
    }

    /// Smooth union with another shape
    fn smooth_union<B: ImplicitFunction2D>(self, other: B, smoothness: f32) -> SmoothUnion2D<Self, B> {
        SmoothUnion2D::new(self, other, smoothness)
    }

    /// Intersection with another shape
    fn intersect<B: ImplicitFunction2D>(self, other: B) -> Intersection2D<Self, B> {
        Intersection2D::new(self, other)
    }

    /// Difference with another shape (subtract other from self)
    fn subtract<B: ImplicitFunction2D>(self, other: B) -> Difference2D<Self, B> {
        Difference2D::new(self, other)
    }

    /// Convert to a type-erased object for dynamic dispatch
    fn into_object(self) -> ImplicitObject2D {
        ImplicitObject2D::new(self)
    }
}

// Implement the builder trait for all 2D implicit functions
impl<T: ImplicitFunction2D + 'static> ShapeBuilder2D for T {}

// ===== Convenience constructors =====

/// Create a circle at the origin
pub fn circle(radius: f32) -> Circle {
    Circle::new(radius)
}

/// Create a rectangle at the origin
pub fn rectangle(width: f32, height: f32) -> Rectangle {
    Rectangle::new(width, height)
}

/// Create a square at the origin
pub fn square(size: f32) -> Rectangle {
    Rectangle::new(size, size)
}

// ===== Example DSL usage in tests =====
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dsl_composition() {
        // Create a shape using the fluent API
        let shape = circle(1.0)
            .translate(2.0, 0.0)
            .union(circle(1.0).translate(-2.0, 0.0))
            .union(rectangle(3.0, 0.5));

        // Test that points inside are negative
        assert!(shape.evaluate(Vec2::new(2.0, 0.0)) < 0.0);
        assert!(shape.evaluate(Vec2::new(-2.0, 0.0)) < 0.0);
    }

    #[test]
    fn test_smooth_union() {
        let shape = circle(1.0)
            .translate(-0.5, 0.0)
            .smooth_union(circle(1.0).translate(0.5, 0.0), 0.5);

        // The blend should create smooth transitions
        assert!(shape.evaluate(Vec2::new(0.0, 0.0)) < 0.0);
    }

    #[test]
    fn test_subtraction() {
        // Create a donut by subtracting a small circle from a large one
        let donut = circle(2.0).subtract(circle(1.0));

        // Inside the hole should be positive (outside)
        assert!(donut.evaluate(Vec2::new(0.0, 0.0)) > 0.0);

        // In the ring should be negative (inside)
        assert!(donut.evaluate(Vec2::new(1.5, 0.0)) < 0.0);
    }
}
