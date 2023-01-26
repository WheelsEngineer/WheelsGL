use std::error::Error;
use pixels::{Pixels, SurfaceTexture};
use wheels_gl::{Color, FrameBuffer};
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

pub type RenderFunction = fn(&mut Box<&mut dyn FrameBuffer>);

struct MemoryBuffer<'a> {
    buffer: &'a mut [u8],
    width: u32,
    height: u32
}

impl<'a> FrameBuffer for MemoryBuffer<'a> {
    fn get_width(&self) -> u32 {
        self.width
    }

    fn get_height(&self) -> u32 {
        self.height
    }

    fn set_pixel_raw(&mut self, x: i32, y: i32, color: Color) {
        let index = 4 * (y * self.width as i32 + x) as usize;
        self.buffer[index] = (color.r * 255.0) as u8;
        self.buffer[index + 1] = (color.g * 255.0) as u8;
        self.buffer[index + 2] = (color.b * 255.0) as u8;
        self.buffer[index + 3] = (color.a * 255.0) as u8;
    }

    fn get_pixel_raw(&self, x: i32, y: i32) -> Color {
        let index = 4 * (y * self.width as i32 + x) as usize;

        Color::new(self.buffer[index] as f32 / 255.0, self.buffer[index + 1] as f32 / 255.0, self.buffer[index + 2] as f32 / 255.0, self.buffer[index + 3] as f32 / 255.0)
    }
}

pub fn run(title: &str, width: u32, height: u32, window_scale: u32, render_function: RenderFunction) -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(width * window_scale, height * window_scale);
        WindowBuilder::new()
            .with_title(title)
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(width, height, surface_texture)?
    };

    let buffer_size = PhysicalSize::new(width, height);

    event_loop.run(move |event, _, control_flow| {

        if let Event::RedrawRequested(_) = event {
            let mut memory_buffer = MemoryBuffer { width: buffer_size.width, height: buffer_size.height, buffer: pixels.get_frame_mut() };

            render_function(&mut Box::<&mut dyn FrameBuffer>::new(&mut memory_buffer));

            if pixels.render().is_err() {
                control_flow.set_exit();
            }
        }


        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                control_flow.set_exit();
                return;
            }

            window.request_redraw();
        }

    });

}