use crate::implicit::{ImplicitFunction2D, ImplicitFunction3D};
use crate::{Vec2, Vec3, Mat3};

// ===== 2D Transformations =====

/// Translation for 2D implicit functions
pub struct Translate2D<F> {
    pub inner: F,
    pub offset: Vec2,
}

impl<F> Translate2D<F> {
    pub fn new(inner: F, offset: Vec2) -> Self {
        Self { inner, offset }
    }
}

impl<F: ImplicitFunction2D> ImplicitFunction2D for Translate2D<F> {
    fn evaluate(&self, point: Vec2) -> f32 {
        self.inner.evaluate(point - self.offset)
    }
}

/// Rotation for 2D implicit functions
pub struct Rotate2D<F> {
    pub inner: F,
    pub angle: f32,
}

impl<F> Rotate2D<F> {
    pub fn new(inner: F, angle: f32) -> Self {
        Self { inner, angle }
    }
}

impl<F: ImplicitFunction2D> ImplicitFunction2D for Rotate2D<F> {
    fn evaluate(&self, point: Vec2) -> f32 {
        let cos = self.angle.cos();
        let sin = self.angle.sin();
        let rotated = Vec2::new(
            point.x * cos + point.y * sin,
            -point.x * sin + point.y * cos,
        );
        self.inner.evaluate(rotated)
    }
}

/// Uniform scale for 2D implicit functions
pub struct Scale2D<F> {
    pub inner: F,
    pub scale: f32,
}

impl<F> Scale2D<F> {
    pub fn new(inner: F, scale: f32) -> Self {
        Self { inner, scale }
    }
}

impl<F: ImplicitFunction2D> ImplicitFunction2D for Scale2D<F> {
    fn evaluate(&self, point: Vec2) -> f32 {
        self.inner.evaluate(point / self.scale) * self.scale
    }
}

// ===== 3D Transformations =====

/// Translation for 3D implicit functions
pub struct Translate3D<F> {
    pub inner: F,
    pub offset: Vec3,
}

impl<F> Translate3D<F> {
    pub fn new(inner: F, offset: Vec3) -> Self {
        Self { inner, offset }
    }
}

impl<F: ImplicitFunction3D> ImplicitFunction3D for Translate3D<F> {
    fn evaluate(&self, point: Vec3) -> f32 {
        self.inner.evaluate(point - self.offset)
    }
}

/// Rotation around arbitrary axis for 3D implicit functions
pub struct Rotate3D<F> {
    pub inner: F,
    pub rotation_matrix: Mat3,
    pub inverse_matrix: Mat3,
}

impl<F> Rotate3D<F> {
    pub fn from_axis_angle(inner: F, axis: Vec3, angle: f32) -> Self {
        let rotation_matrix = Mat3::from_axis_angle(axis.normalize(), angle);
        let inverse_matrix = rotation_matrix.transpose();
        Self {
            inner,
            rotation_matrix,
            inverse_matrix,
        }
    }

    pub fn from_rotation_x(inner: F, angle: f32) -> Self {
        Self::from_axis_angle(inner, Vec3::X, angle)
    }

    pub fn from_rotation_y(inner: F, angle: f32) -> Self {
        Self::from_axis_angle(inner, Vec3::Y, angle)
    }

    pub fn from_rotation_z(inner: F, angle: f32) -> Self {
        Self::from_axis_angle(inner, Vec3::Z, angle)
    }
}

impl<F: ImplicitFunction3D> ImplicitFunction3D for Rotate3D<F> {
    fn evaluate(&self, point: Vec3) -> f32 {
        let rotated = self.inverse_matrix * point;
        self.inner.evaluate(rotated)
    }
}

/// Uniform scale for 3D implicit functions
pub struct Scale3D<F> {
    pub inner: F,
    pub scale: f32,
}

impl<F> Scale3D<F> {
    pub fn new(inner: F, scale: f32) -> Self {
        Self { inner, scale }
    }
}

impl<F: ImplicitFunction3D> ImplicitFunction3D for Scale3D<F> {
    fn evaluate(&self, point: Vec3) -> f32 {
        self.inner.evaluate(point / self.scale) * self.scale
    }
}
