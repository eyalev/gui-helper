use anyhow::Result;
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder, WindowLevel},
};
use pixels::{Pixels, SurfaceTexture};
use std::time::{Duration, Instant};

pub struct OverlayWindow {
    pub width: u32,
    pub height: u32,
    event_loop: Option<EventLoop<()>>,
    window: Option<Window>,
    pixels: Option<Pixels>,
}

impl OverlayWindow {
    pub fn new(width: u32, height: u32) -> Result<Self> {
        let event_loop = EventLoop::new()?;
        
        let window = WindowBuilder::new()
            .with_title("GUI Helper Overlay")
            .with_inner_size(PhysicalSize::new(width, height))
            .with_position(winit::dpi::PhysicalPosition::new(0, 0))
            .with_decorations(false)
            .with_transparent(true)
            .with_window_level(WindowLevel::AlwaysOnTop)
            .with_resizable(false)
            .build(&event_loop)?;
        
        let surface_texture = SurfaceTexture::new(width, height, &window);
        let pixels = Pixels::new(width, height, surface_texture)?;
        
        Ok(Self {
            width,
            height,
            event_loop: Some(event_loop),
            window: Some(window),
            pixels: Some(pixels),
        })
    }
    
    pub fn run_with_timeout<F>(mut self, duration_secs: u32, mut draw_callback: F) -> Result<()>
    where
        F: FnMut(&mut [u8], u32, u32) + 'static,
    {
        let event_loop = self.event_loop.take().unwrap();
        let window = self.window.take().unwrap();
        let mut pixels = self.pixels.take().unwrap();
        
        let start_time = Instant::now();
        let timeout_duration = Duration::from_secs(duration_secs as u64);
        
        event_loop.run(move |event, elwt| {
            elwt.set_control_flow(ControlFlow::Poll);
            
            if start_time.elapsed() >= timeout_duration {
                elwt.exit();
                return;
            }
            
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    elwt.exit();
                }
                Event::WindowEvent {
                    event: WindowEvent::RedrawRequested,
                    ..
                } => {
                    let frame = pixels.frame_mut();
                    draw_callback(frame, self.width, self.height);
                    
                    if let Err(err) = pixels.render() {
                        eprintln!("pixels.render() failed: {err}");
                        elwt.exit();
                    }
                }
                Event::AboutToWait => {
                    window.request_redraw();
                }
                _ => {}
            }
        })?;
        
        Ok(())
    }
}