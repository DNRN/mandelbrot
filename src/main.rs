use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;

#[derive(Default)]
struct App {
    window: Option<Window>,
}

impl ApplicationHandler for App {

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let mut window_attr = Window::default_attributes();
        window_attr.title = "Mandelbrot".to_owned();
        window_attr.inner_size = Some(LogicalSize::new(WIDTH, HEIGHT).into());
        self.window = Some(event_loop.create_window(window_attr).unwrap());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                // Draw.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);

    // ControlFlow::Wait pauses the event loop if no events are available to process.
    // This is ideal for non-game applications that only update in response to user
    // input, and uses significantly less power/CPU time than ControlFlow::Poll.
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::default();
    event_loop.run_app(&mut app);
}

fn draw_png (image_width: u32, image_height: u32, ) {
    let ratio = image_height as f32 / image_width as f32;
    let center = Point {x: 0.0, y: 0.0};
    let width = 4.0 as f32;
    let height = width * ratio;
    let init_x = center.x - (width / 2 as f32);
    let init_y = center.y - (height / 2 as f32);
    let inc = width / (image_width as f32);

    let mut image_buffer = image::ImageBuffer::new(
        image_width, image_height);

    for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
        // let u = x as f32 / image_height as f32;
        // let v = y as f32 / image_height as f32;
        // let t = mandelbrot(2.5 * (u - 0.5) - 1.4, 2.5 * (v - 0.5));
        let u = init_x + (x as f32 * inc);
        let v = init_y + (y as f32 * inc);
        let t = mandelbrot(u, v);
        *pixel = image::Rgb(color((2.0 * t + 0.5) % 1.0));
    }

    image_buffer.save("mandelbrot.png").unwrap();
}

struct Point {
    pub x: f32,
    pub y: f32
}

#[derive(Clone, Copy)]
struct Complex {
    pub a: f32,
    pub b: f32,
}

impl std::ops::Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            a: self.a + rhs.a,
            b: self.b + rhs.b,
        }
    }
}

impl std::ops::Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Complex { 
            a: self.a * rhs.a - self.b * rhs.b, 
            b: self.a * rhs.b + self.b * rhs.a,
        }
    }
}

impl Complex {
    fn arg_sq(self) -> f32 {
        self.a * self.a + self.b * self.b
    }
}

fn mandelbrot(x: f32, y: f32) -> f32 {
    let mut z = Complex { a: 0.0, b: 0.0 };
    let c = Complex { a: x, b: y };
    let max = 256;
    let mut i = 0;
    while i < max && z.arg_sq() < 32.0 {
        z = z * z + c;
        i += 1;
    }
    // let result = (i as f32 - z.arg_sq().log2().log2()) / (max as f32);
    // println!("({},{})={}",x, y, result);
    return (i as f32 - z.arg_sq().log2().log2()) / (max as f32);
}

fn color(t: f32) -> [u8; 3] {
    let a = (0.5, 0.5, 0.5);
    let b = (0.5, 0.5, 0.5);
    let c = (1.0, 1.0, 1.0);
    let d = (0.0, 0.10, 0.20);
    let r = b.0 * (6.28318 * (c.0 * t + d.0)).cos() + a.0;
    let g = b.1 * (6.28318 * (c.1 * t + d.1)).cos() + a.1;
    let b = b.2 * (6.28318 * (c.2 * t + d.2)).cos() + a.2;
    [(255.0 * r) as u8, (255.0 * g) as u8, (255.0 * b) as u8]
}
