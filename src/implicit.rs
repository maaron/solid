use crate::{Vec2, Vec3};

/// Trait for 2D implicit functions
/// An implicit surface is defined by f(x, y) = 0
/// - f(x, y) < 0: inside the shape
/// - f(x, y) = 0: on the boundary
/// - f(x, y) > 0: outside the shape
pub trait ImplicitFunction2D: Send + Sync {
    /// Evaluate the signed distance function at point (x, y)
    fn evaluate(&self, point: Vec2) -> f32;

    /// Compute the gradient (normal) at a point using central differences
    fn gradient(&self, point: Vec2) -> Vec2 {
        let h = 0.0001;
        let dx = (self.evaluate(point + Vec2::new(h, 0.0)) -
                  self.evaluate(point - Vec2::new(h, 0.0))) / (2.0 * h);
        let dy = (self.evaluate(point + Vec2::new(0.0, h)) -
                  self.evaluate(point - Vec2::new(0.0, h))) / (2.0 * h);
        Vec2::new(dx, dy)
    }
}

/// Trait for 3D implicit functions (Signed Distance Fields)
/// An implicit surface is defined by f(x, y, z) = 0
/// - f(x, y, z) < 0: inside the shape
/// - f(x, y, z) = 0: on the boundary
/// - f(x, y, z) > 0: outside the shape
pub trait ImplicitFunction3D: Send + Sync {
    /// Evaluate the signed distance function at point (x, y, z)
    fn evaluate(&self, point: Vec3) -> f32;

    /// Compute the gradient (normal) at a point using central differences
    fn gradient(&self, point: Vec3) -> Vec3 {
        let h = 0.0001;
        let dx = (self.evaluate(point + Vec3::new(h, 0.0, 0.0)) -
                  self.evaluate(point - Vec3::new(h, 0.0, 0.0))) / (2.0 * h);
        let dy = (self.evaluate(point + Vec3::new(0.0, h, 0.0)) -
                  self.evaluate(point - Vec3::new(0.0, h, 0.0))) / (2.0 * h);
        let dz = (self.evaluate(point + Vec3::new(0.0, 0.0, h)) -
                  self.evaluate(point - Vec3::new(0.0, 0.0, h))) / (2.0 * h);
        Vec3::new(dx, dy, dz)
    }
}

/// Type-erased wrapper for 2D implicit functions
pub struct ImplicitObject2D {
    evaluator: Box<dyn ImplicitFunction2D>,
}

impl ImplicitObject2D {
    pub fn new<F: ImplicitFunction2D + 'static>(func: F) -> Self {
        Self {
            evaluator: Box::new(func),
        }
    }
}

impl ImplicitFunction2D for ImplicitObject2D {
    fn evaluate(&self, point: Vec2) -> f32 {
        self.evaluator.evaluate(point)
    }

    fn gradient(&self, point: Vec2) -> Vec2 {
        self.evaluator.gradient(point)
    }
}

/// Type-erased wrapper for 3D implicit functions
pub struct ImplicitObject3D {
    evaluator: Box<dyn ImplicitFunction3D>,
}

impl ImplicitObject3D {
    pub fn new<F: ImplicitFunction3D + 'static>(func: F) -> Self {
        Self {
            evaluator: Box::new(func),
        }
    }
}

impl ImplicitFunction3D for ImplicitObject3D {
    fn evaluate(&self, point: Vec3) -> f32 {
        self.evaluator.evaluate(point)
    }

    fn gradient(&self, point: Vec3) -> Vec3 {
        self.evaluator.gradient(point)
    }
}
