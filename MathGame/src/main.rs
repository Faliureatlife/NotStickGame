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
// use std::{
//     time::Duration,
//     thread::sleep,
// };
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

    //Have mutable frame buffer
    let frame = pixels.get_frame(); // returns slice called pixels
    //pixel is the iterator variable
    //frame is the slice
    //chunks_exact_mut allows muting group in slice, 4 is the aomunt that will be changed at once
    let mut r:u8 = 0; let mut g:u8 = 0; let mut b:u8 = 0;

    for pixel in frame.chunks_exact_mut(4) {
        pixel[0] = r; // R
        pixel[1] = g; // G
        pixel[2] = b; // B
        pixel[3] = 0x11; // A
    }


    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        pixels.render().unwrap();
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
