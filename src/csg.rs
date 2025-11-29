use crate::implicit::{ImplicitFunction2D, ImplicitFunction3D};
use crate::{Vec2, Vec3};

// ===== 2D CSG Operations =====

/// Union of two 2D implicit functions
pub struct Union2D<A, B> {
    pub a: A,
    pub b: B,
}

impl<A, B> Union2D<A, B> {
    pub fn new(a: A, b: B) -> Self {
        Self { a, b }
    }
}

impl<A: ImplicitFunction2D, B: ImplicitFunction2D> ImplicitFunction2D for Union2D<A, B> {
    fn evaluate(&self, point: Vec2) -> f32 {
        self.a.evaluate(point).min(self.b.evaluate(point))
    }
}

/// Intersection of two 2D implicit functions
pub struct Intersection2D<A, B> {
    pub a: A,
    pub b: B,
}

impl<A, B> Intersection2D<A, B> {
    pub fn new(a: A, b: B) -> Self {
        Self { a, b }
    }
}

impl<A: ImplicitFunction2D, B: ImplicitFunction2D> ImplicitFunction2D for Intersection2D<A, B> {
    fn evaluate(&self, point: Vec2) -> f32 {
        self.a.evaluate(point).max(self.b.evaluate(point))
    }
}

/// Difference of two 2D implicit functions (A - B)
pub struct Difference2D<A, B> {
    pub a: A,
    pub b: B,
}

impl<A, B> Difference2D<A, B> {
    pub fn new(a: A, b: B) -> Self {
        Self { a, b }
    }
}

impl<A: ImplicitFunction2D, B: ImplicitFunction2D> ImplicitFunction2D for Difference2D<A, B> {
    fn evaluate(&self, point: Vec2) -> f32 {
        self.a.evaluate(point).max(-self.b.evaluate(point))
    }
}

/// Smooth union (smooth minimum) for 2D shapes
pub struct SmoothUnion2D<A, B> {
    pub a: A,
    pub b: B,
    pub smoothness: f32,
}

impl<A, B> SmoothUnion2D<A, B> {
    pub fn new(a: A, b: B, smoothness: f32) -> Self {
        Self { a, b, smoothness }
    }
}

impl<A: ImplicitFunction2D, B: ImplicitFunction2D> ImplicitFunction2D for SmoothUnion2D<A, B> {
    fn evaluate(&self, point: Vec2) -> f32 {
        let d1 = self.a.evaluate(point);
        let d2 = self.b.evaluate(point);
        let h = (0.5 + 0.5 * (d2 - d1) / self.smoothness).clamp(0.0, 1.0);
        d2 * (1.0 - h) + d1 * h - self.smoothness * h * (1.0 - h)
    }
}

// ===== 3D CSG Operations =====

/// Union of two 3D implicit functions
pub struct Union3D<A, B> {
    pub a: A,
    pub b: B,
}

impl<A, B> Union3D<A, B> {
    pub fn new(a: A, b: B) -> Self {
        Self { a, b }
    }
}

impl<A: ImplicitFunction3D, B: ImplicitFunction3D> ImplicitFunction3D for Union3D<A, B> {
    fn evaluate(&self, point: Vec3) -> f32 {
        self.a.evaluate(point).min(self.b.evaluate(point))
    }
}

/// Intersection of two 3D implicit functions
pub struct Intersection3D<A, B> {
    pub a: A,
    pub b: B,
}

impl<A, B> Intersection3D<A, B> {
    pub fn new(a: A, b: B) -> Self {
        Self { a, b }
    }
}

impl<A: ImplicitFunction3D, B: ImplicitFunction3D> ImplicitFunction3D for Intersection3D<A, B> {
    fn evaluate(&self, point: Vec3) -> f32 {
        self.a.evaluate(point).max(self.b.evaluate(point))
    }
}

/// Difference of two 3D implicit functions (A - B)
pub struct Difference3D<A, B> {
    pub a: A,
    pub b: B,
}

impl<A, B> Difference3D<A, B> {
    pub fn new(a: A, b: B) -> Self {
        Self { a, b }
    }
}

impl<A: ImplicitFunction3D, B: ImplicitFunction3D> ImplicitFunction3D for Difference3D<A, B> {
    fn evaluate(&self, point: Vec3) -> f32 {
        self.a.evaluate(point).max(-self.b.evaluate(point))
    }
}

/// Smooth union (smooth minimum) for 3D shapes
pub struct SmoothUnion3D<A, B> {
    pub a: A,
    pub b: B,
    pub smoothness: f32,
}

impl<A, B> SmoothUnion3D<A, B> {
    pub fn new(a: A, b: B, smoothness: f32) -> Self {
        Self { a, b, smoothness }
    }
}

impl<A: ImplicitFunction3D, B: ImplicitFunction3D> ImplicitFunction3D for SmoothUnion3D<A, B> {
    fn evaluate(&self, point: Vec3) -> f32 {
        let d1 = self.a.evaluate(point);
        let d2 = self.b.evaluate(point);
        let h = (0.5 + 0.5 * (d2 - d1) / self.smoothness).clamp(0.0, 1.0);
        d2 * (1.0 - h) + d1 * h - self.smoothness * h * (1.0 - h)
    }
}

/// Smooth intersection for 3D shapes
pub struct SmoothIntersection3D<A, B> {
    pub a: A,
    pub b: B,
    pub smoothness: f32,
}

impl<A, B> SmoothIntersection3D<A, B> {
    pub fn new(a: A, b: B, smoothness: f32) -> Self {
        Self { a, b, smoothness }
    }
}

impl<A: ImplicitFunction3D, B: ImplicitFunction3D> ImplicitFunction3D for SmoothIntersection3D<A, B> {
    fn evaluate(&self, point: Vec3) -> f32 {
        let d1 = self.a.evaluate(point);
        let d2 = self.b.evaluate(point);
        let h = (0.5 - 0.5 * (d2 - d1) / self.smoothness).clamp(0.0, 1.0);
        d2 * (1.0 - h) + d1 * h + self.smoothness * h * (1.0 - h)
    }
}

/// Smooth difference for 3D shapes
pub struct SmoothDifference3D<A, B> {
    pub a: A,
    pub b: B,
    pub smoothness: f32,
}

impl<A, B> SmoothDifference3D<A, B> {
    pub fn new(a: A, b: B, smoothness: f32) -> Self {
        Self { a, b, smoothness }
    }
}

impl<A: ImplicitFunction3D, B: ImplicitFunction3D> ImplicitFunction3D for SmoothDifference3D<A, B> {
    fn evaluate(&self, point: Vec3) -> f32 {
        let d1 = self.a.evaluate(point);
        let d2 = self.b.evaluate(point);
        let h = (0.5 - 0.5 * (d2 + d1) / self.smoothness).clamp(0.0, 1.0);
        d2 * h - d1 * (1.0 - h) + self.smoothness * h * (1.0 - h)
    }
}
