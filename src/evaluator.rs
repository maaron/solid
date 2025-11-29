use crate::implicit::ImplicitFunction2D;
use crate::Vec2;

/// Evaluate a 2D implicit function over a rectangular region and generate RGBA pixel data
pub struct Evaluator2D {
    width: u32,
    height: u32,
    pub view_min: Vec2,
    pub view_max: Vec2,
}

impl Evaluator2D {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            view_min: Vec2::new(-5.0, -5.0),
            view_max: Vec2::new(5.0, 5.0),
        }
    }

    pub fn with_view(width: u32, height: u32, min: Vec2, max: Vec2) -> Self {
        Self {
            width,
            height,
            view_min: min,
            view_max: max,
        }
    }

    pub fn set_view(&mut self, min: Vec2, max: Vec2) {
        self.view_min = min;
        self.view_max = max;
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    /// Evaluate an implicit function and return RGBA8 pixel data
    /// Colors the shape based on signed distance
    pub fn evaluate<F: ImplicitFunction2D>(&self, function: &F) -> Vec<u8> {
        let mut pixels = vec![0u8; (self.width * self.height * 4) as usize];

        for y in 0..self.height {
            for x in 0..self.width {
                // Map pixel coordinates to world coordinates
                let u = x as f32 / self.width as f32;
                let v = y as f32 / self.height as f32;

                let world_x = self.view_min.x + u * (self.view_max.x - self.view_min.x);
                let world_y = self.view_min.y + v * (self.view_max.y - self.view_min.y);

                let point = Vec2::new(world_x, world_y);
                let distance = function.evaluate(point);

                let pixel_index = ((y * self.width + x) * 4) as usize;

                // Color based on distance
                let color = self.distance_to_color(distance);
                pixels[pixel_index] = color.0;
                pixels[pixel_index + 1] = color.1;
                pixels[pixel_index + 2] = color.2;
                pixels[pixel_index + 3] = color.3;
            }
        }

        pixels
    }

    /// Convert signed distance to RGBA color
    /// Negative distance (inside) = blue
    /// Zero distance (boundary) = white
    /// Positive distance (outside) = black gradient
    fn distance_to_color(&self, distance: f32) -> (u8, u8, u8, u8) {
        if distance < 0.0 {
            // Inside - blue with intensity based on distance
            let intensity = (1.0 - (-distance * 0.5).min(1.0)) * 255.0;
            let blue = 200 + ((255 - 200) as f32 * (1.0 - intensity / 255.0)) as u8;
            (intensity as u8, intensity as u8, blue, 255)
        } else if distance.abs() < 0.02 {
            // Boundary - white
            (255, 255, 255, 255)
        } else {
            // Outside - gradient to dark gray
            let intensity = (1.0 - (distance * 0.5).min(1.0)) * 128.0;
            (intensity as u8, intensity as u8, intensity as u8, 255)
        }
    }

    /// Alternative coloring: simple binary (inside/outside)
    pub fn evaluate_binary<F: ImplicitFunction2D>(&self, function: &F) -> Vec<u8> {
        let mut pixels = vec![0u8; (self.width * self.height * 4) as usize];

        for y in 0..self.height {
            for x in 0..self.width {
                let u = x as f32 / self.width as f32;
                let v = y as f32 / self.height as f32;

                let world_x = self.view_min.x + u * (self.view_max.x - self.view_min.x);
                let world_y = self.view_min.y + v * (self.view_max.y - self.view_min.y);

                let point = Vec2::new(world_x, world_y);
                let distance = function.evaluate(point);

                let pixel_index = ((y * self.width + x) * 4) as usize;

                if distance < 0.0 {
                    // Inside - white
                    pixels[pixel_index] = 255;
                    pixels[pixel_index + 1] = 255;
                    pixels[pixel_index + 2] = 255;
                    pixels[pixel_index + 3] = 255;
                } else {
                    // Outside - black
                    pixels[pixel_index] = 0;
                    pixels[pixel_index + 1] = 0;
                    pixels[pixel_index + 2] = 0;
                    pixels[pixel_index + 3] = 255;
                }
            }
        }

        pixels
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}
