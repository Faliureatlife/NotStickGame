//extra functions for idiomadic code or wtv
mod render;

use pixels::Pixels;
//Dont just import all of pixels at some point
// use pixels::wgpu::Color;
use winit::{
    dpi::LogicalSize,
    //dpi::PhysicalSize,
    event::*,
    event_loop::*,
    window::Window,
};
use std::{
    fs::*,
//     time::Duration,
//     thread::sleep,
};
fn main() -> Result<(), pixels::Error> {
    //where event loop is created for future event_loop.run
    let event_loop = EventLoop::new();

    //Create window and give it Logical Size of 720 4:3
    let window = Window::new(&event_loop).unwrap();
    window.set_inner_size(LogicalSize::new(720, 540));
    let size = window.inner_size();

    //Create surface texture of given width and height with deref window
    let surface_texture = pixels::SurfaceTexture::new(size.width, size.height, &window);

    //frame buffer "pixels"
    let mut pixels = Pixels::new(size.width, size.height, surface_texture)?;

    let mut screen = Screen::new("WorldData/Houses");


    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        let pix = pixels.get_frame();
        screen.draw(pix);
        {if pixels
        .render()
        .map_err(|e| panic!("pixels.render() failed: {}", e))
        .is_err()
        {}}
        //close window
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
    //Ok(())
    //use to crash program safely
    //
}
struct Player {
    x_pos: u16,
    y_pos: u16,
}

struct Screen {
    //baddies: Vec<Baddie>,
    area: Vec<u8>,
    fn draw ()
}

impl Screen {
    fn new(place: &str) -> Self {
        Self {
            //baddies: vec![],
            //check the types that are used if errors, maybe &str ?
            area: std::fs::read(place).unwrap(),
        }
    }
}
// fn _update(&mut self) {
//
// }
