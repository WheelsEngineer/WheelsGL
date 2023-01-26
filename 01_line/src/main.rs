use std::error::Error;
use std::mem::swap;
use wheels_gl::{Color, FrameBuffer, Point2D};
use wheels_platform::run;

struct PointI32 {
    pub x: i32,
    pub y: i32
}

impl From<Point2D> for PointI32 {
    fn from(value: Point2D) -> Self {
        Self {
            x: value.x.round() as i32,
            y: value.y.round() as i32
        }
    }
}

fn line_method_1(frame_buffer: &mut Box<&mut dyn FrameBuffer>, point_a: Point2D, point_b: Point2D, color: Color) {
    let mut start_point = PointI32::from(point_a);
    let mut end_point = PointI32::from(point_b);

    if start_point.x == end_point.x {
        return
    }

    if start_point.x > end_point.x {
        swap(&mut start_point, &mut end_point);
    }

    let slope = (point_b.y - point_a.y) / (point_b.x - point_a.x);

    for x in start_point.x..end_point.x {
        let y = start_point.y + (slope * (x - start_point.x) as f32).round() as i32;
        frame_buffer.set_pixel(x, y, color);
    }
}

fn line_method_2(frame_buffer: &mut Box<&mut dyn FrameBuffer>, point_a: Point2D, point_b: Point2D, color: Color) {
    let mut start_point = PointI32::from(point_a);
    let mut end_point = PointI32::from(point_b);

    let mut x_y_swapped = false;

    if (start_point.x - end_point.x).abs() < (start_point.y - end_point.y).abs() {
        swap(&mut start_point.x, & mut start_point.y);
        swap(&mut end_point.x, & mut end_point.y);
        x_y_swapped = true;
    }

    if start_point.x == end_point.x {
        return
    }

    if start_point.x > end_point.x {
        swap(&mut start_point, &mut end_point);
    }

    let slope = if x_y_swapped {
        (point_b.x - point_a.x) / (point_b.y - point_a.y)
    } else {
        (point_b.y - point_a.y) / (point_b.x - point_a.x)
    };


    for x in start_point.x..end_point.x {
        let y = start_point.y + (slope * (x - start_point.x) as f32).round() as i32;
        if x_y_swapped {
            frame_buffer.set_pixel(y, x, color);
        } else {
            frame_buffer.set_pixel(x, y, color);
        }
    }
}

// Bresenham’s Line Drawing Algorithm
fn line_method_3(frame_buffer: &mut Box<&mut dyn FrameBuffer>, point_a: Point2D, point_b: Point2D, color: Color) {
    let mut start_point = PointI32::from(point_a);
    let mut end_point = PointI32::from(point_b);

    let mut x_y_swapped = false;

    if (start_point.x - end_point.x).abs() < (start_point.y - end_point.y).abs() {
        swap(&mut start_point.x, & mut start_point.y);
        swap(&mut end_point.x, & mut end_point.y);
        x_y_swapped = true;
    }

    if start_point.x == end_point.x {
        return
    }

    if start_point.x > end_point.x {
        swap(&mut start_point, &mut end_point);
    }

    let delta_x = end_point.x - start_point.x;
    let delta_y = end_point.y - start_point.y;

    let delta_x_2 = delta_x * 2;
    let delta_y_2_abs = (delta_y * 2).abs();
    let mut error = 0;

    let mut y = start_point.y;
    let step = if end_point.y > start_point.y { 1 }  else { -1 };


    for x in start_point.x..end_point.x {
        if x_y_swapped {
            frame_buffer.set_pixel(y, x, color);
        } else {
            frame_buffer.set_pixel(x, y, color);
        }

        error += delta_y_2_abs;
        if error > delta_x {
            y += step;
            error -= delta_x_2;
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    run("Line", 160, 120, 1, |frame_buffer| {
        frame_buffer.clear(Color::WHITE);
        frame_buffer.set_pixel(10, 10, Color::BLACK);

        line_method_1(frame_buffer, Point2D::from_slice(&[ 10.0, 10.0 ]), Point2D::from_slice(&[ 100.0, 20.0 ]), Color::BLACK);
        line_method_2(frame_buffer, Point2D::from_slice(&[ 10.0, 20.0 ]), Point2D::from_slice(&[ 100.0, 30.0 ]), Color::BLACK);
        line_method_3(frame_buffer, Point2D::from_slice(&[ 10.0, 30.0 ]), Point2D::from_slice(&[ 100.0, 40.0 ]), Color::BLACK);

        line_method_1(frame_buffer, Point2D::from_slice(&[ 120.0, 10.0 ]), Point2D::from_slice(&[ 110.0, 100.0 ]), Color::BLACK);
        line_method_2(frame_buffer, Point2D::from_slice(&[ 130.0, 10.0 ]), Point2D::from_slice(&[ 120.0, 100.0 ]), Color::BLACK);
        line_method_3(frame_buffer, Point2D::from_slice(&[ 140.0, 10.0 ]), Point2D::from_slice(&[ 130.0, 100.0 ]), Color::BLACK);
    })
}
