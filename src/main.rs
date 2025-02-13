use std::sync::Arc;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

struct WindowManager {
    window: Window,
    pixels: Pixels<'static>
}

impl WindowManager {
    pub fn new(window: Window, pixels: Pixels<'static>) -> Self {
        Self{
            window,
            pixels,
        }
    }
}

#[derive(Default)]
struct App {
    // window_manager: Option<WindowManager>
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
}

impl App {
    

    fn create_window(&mut self, event_loop: &ActiveEventLoop) {
        let window = {
            let size = LogicalSize::new(WIDTH, HEIGHT);
            let window = event_loop.create_window(Window::default_attributes()
                .with_title("Mandelbrot")
                .with_inner_size(size)
                .with_min_inner_size(size)
            ).unwrap();
            Arc::new(window)
        };

        self.window = Some(window);
        
        // Create another reference-counted pointer to the same Window
        let window_clone = Arc::clone(&self.window.as_ref().unwrap());

        let pixels = {
            let window_size =  self.window.as_ref().unwrap().inner_size();
            let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, window_clone);

            Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap()
        };

        self.pixels = Some(pixels);
        

        // self.window = Some(window);
    }

    fn draw(frame: &mut [u8], size: PhysicalSize<u32>) {
        let width = size.width;
        let height = size.height;
        println!("({},{})", width, height);
        let ratio = height as f32 / width as f32;
        let center = Point {x: -0.5, y: 0.0};
        let width_plot = 4.0 as f32;
        let height_plot = width_plot * ratio;
        let init_x = center.x - (width_plot / 2 as f32);
        let init_y = center.y - (height_plot / 2 as f32);
        let inc = width_plot / (width as f32);

        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % width as usize) as i16;
            let y = (i / width as usize) as i16;

            let u = init_x + (x as f32 * inc);
            let v = init_y + (y as f32 * inc);
            let t = mandelbrot(u, v);
            let color = color((2.0 * t + 0.5) % 1.0);

            pixel[0] = color[0]; // Red
            pixel[1] = color[1];   // Green
            pixel[2] = color[2];   // Blue
            pixel[3] = 255; // Alpha (fully opaque)
        }
    }
}

impl ApplicationHandler for App {

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.create_window(&event_loop);
        self.window.as_ref().unwrap().request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::Resized(new_size) => {
                println!("Window resized: {}x{}", new_size.width, new_size.height);
                
                if let Some(pixels) = self.pixels.as_mut() {
                    pixels.resize_surface(new_size.width, new_size.height).unwrap();
                    pixels.resize_buffer(new_size.width, new_size.height).unwrap();
                }
    
                self.window.as_ref().unwrap().request_redraw(); // Trigger redraw after resize
            }
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
                // First, get a mutable reference to `pixels`
                let width = self.pixels.as_ref().unwrap().texture().width();
                let height = self.pixels.as_ref().unwrap().texture().height();
                println!("Pixels Frame Size: {}x{}", width, height);

                let pixels = self.pixels.as_mut().unwrap();
                let frame = pixels.frame_mut(); // Borrow the frame buffer

                
                println!("Pixels Frame Size: {}x{}", width, height);

                // Now, call draw WITHOUT borrowing `self` again
                Self::draw(frame, self.window.as_ref().unwrap().inner_size());

                pixels.render().expect("Failed to render frame");
                self.window.as_ref().unwrap().request_redraw();
                // self.window_manager.as_ref().unwrap().window.request_redraw();
            }
            _ => (),
        }
    }
}

fn main() -> Result<(), Error> {
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


    return Ok(());
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
