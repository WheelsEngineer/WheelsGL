use nalgebra::geometry::{Point2, Point3};

pub type Point2D = Point2<f32>;
pub type Point3D = Point3<f32>;

#[derive(Default, Debug, Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

impl Color {
    pub const WHITE: Self = Self::new(1.0, 1.0, 1.0, 1.0);
    pub const BLACK: Self = Self::new(0.0, 0.0, 0.0, 1.0);

    #[inline(always)]
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            r, g, b, a
        }
    }
}

pub trait FrameBuffer {
    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;

    fn set_pixel_raw(&mut self, x: i32, y: i32, color: Color);
    fn set_pixel(&mut self, x: i32, y: i32, color: Color) -> bool {
        if x < 0 || x >= self.get_width() as i32 || y < 0 || y >= self.get_height() as i32 {
            false
        } else {
            self.set_pixel_raw(x, y, color);
            true
        }
    }

    fn get_pixel_raw(&self, x: i32, y: i32) -> Color;
    fn get_pixel(&self, x: i32, y: i32) -> Option<Color> {
        if x < 0 || x >= self.get_width() as i32 || y < 0 || y >= self.get_height() as i32 {
            None
        } else {
            Some(self.get_pixel_raw(x, y))
        }
    }

    fn clear(&mut self, color: Color) {
        for y in 0..self.get_height() as i32 {
            for x in 0..self.get_width() as i32 {
                self.set_pixel_raw(x, y, color);
            }
        }
    }
}