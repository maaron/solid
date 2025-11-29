use crate::implicit::{ImplicitFunction2D, ImplicitFunction3D};
use crate::{Vec2, Vec3};

// ===== 2D Primitives =====

/// 2D Circle (signed distance field)
#[derive(Clone, Copy, Debug)]
pub struct Circle {
    pub radius: f32,
}

impl Circle {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
}

impl ImplicitFunction2D for Circle {
    fn evaluate(&self, point: Vec2) -> f32 {
        point.length() - self.radius
    }
}

/// 2D Rectangle (signed distance field)
#[derive(Clone, Copy, Debug)]
pub struct Rectangle {
    pub half_extents: Vec2,
}

impl Rectangle {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            half_extents: Vec2::new(width / 2.0, height / 2.0),
        }
    }
}

impl ImplicitFunction2D for Rectangle {
    fn evaluate(&self, point: Vec2) -> f32 {
        let d = point.abs() - self.half_extents;
        d.max(Vec2::ZERO).length() + d.x.max(d.y).min(0.0)
    }
}

// ===== 3D Primitives =====

/// 3D Sphere (signed distance field)
#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    pub radius: f32,
}

impl Sphere {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
}

impl ImplicitFunction3D for Sphere {
    fn evaluate(&self, point: Vec3) -> f32 {
        point.length() - self.radius
    }
}

/// 3D Box (signed distance field)
#[derive(Clone, Copy, Debug)]
pub struct Box {
    pub half_extents: Vec3,
}

impl Box {
    pub fn new(width: f32, height: f32, depth: f32) -> Self {
        Self {
            half_extents: Vec3::new(width / 2.0, height / 2.0, depth / 2.0),
        }
    }

    pub fn cube(size: f32) -> Self {
        Self::new(size, size, size)
    }
}

impl ImplicitFunction3D for Box {
    fn evaluate(&self, point: Vec3) -> f32 {
        let d = point.abs() - self.half_extents;
        d.max(Vec3::ZERO).length() + d.x.max(d.y).max(d.z).min(0.0)
    }
}

/// 3D Torus (signed distance field)
#[derive(Clone, Copy, Debug)]
pub struct Torus {
    pub major_radius: f32,
    pub minor_radius: f32,
}

impl Torus {
    pub fn new(major_radius: f32, minor_radius: f32) -> Self {
        Self {
            major_radius,
            minor_radius,
        }
    }
}

impl ImplicitFunction3D for Torus {
    fn evaluate(&self, point: Vec3) -> f32 {
        let q = Vec2::new(
            Vec2::new(point.x, point.z).length() - self.major_radius,
            point.y,
        );
        q.length() - self.minor_radius
    }
}

/// 3D Cylinder (signed distance field)
#[derive(Clone, Copy, Debug)]
pub struct Cylinder {
    pub radius: f32,
    pub height: f32,
}

impl Cylinder {
    pub fn new(radius: f32, height: f32) -> Self {
        Self { radius, height }
    }
}

impl ImplicitFunction3D for Cylinder {
    fn evaluate(&self, point: Vec3) -> f32 {
        let d = Vec2::new(
            Vec2::new(point.x, point.z).length() - self.radius,
            point.y.abs() - self.height / 2.0,
        );
        d.max(Vec2::ZERO).length() + d.x.max(d.y).min(0.0)
    }
}

/// 3D Capsule (signed distance field)
#[derive(Clone, Copy, Debug)]
pub struct Capsule {
    pub radius: f32,
    pub height: f32,
}

impl Capsule {
    pub fn new(radius: f32, height: f32) -> Self {
        Self { radius, height }
    }
}

impl ImplicitFunction3D for Capsule {
    fn evaluate(&self, point: Vec3) -> f32 {
        let half_height = self.height / 2.0;
        let y = point.y.clamp(-half_height, half_height);
        let to_line = point - Vec3::new(0.0, y, 0.0);
        to_line.length() - self.radius
    }
}

/// 3D Plane (signed distance field)
#[derive(Clone, Copy, Debug)]
pub struct Plane {
    pub normal: Vec3,
    pub distance: f32,
}

impl Plane {
    pub fn new(normal: Vec3, distance: f32) -> Self {
        Self {
            normal: normal.normalize(),
            distance,
        }
    }

    pub fn from_point_normal(point: Vec3, normal: Vec3) -> Self {
        let normal = normal.normalize();
        Self {
            normal,
            distance: point.dot(normal),
        }
    }
}

impl ImplicitFunction3D for Plane {
    fn evaluate(&self, point: Vec3) -> f32 {
        point.dot(self.normal) - self.distance
    }
}
