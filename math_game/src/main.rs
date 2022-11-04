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
     env,
//     fs::*,
//     time::Duration,
//     thread::sleep,
 };
 use hex::FromHex;

fn main() -> Result<(), pixels::Error> {
    //where event loop is created for future event_loop.run
    let event_loop = EventLoop::new();
    let var = "RUST_BACKTRACE";
    env::set_var(var, "1");
    //Create window and give it Logical Size of 720 4:3
    let window = Window::new(&event_loop).unwrap();
    window.set_inner_size(LogicalSize::new(720, 540));
    let size = window.inner_size();

    //Create surface texture of given width and height with deref window
    let surface_texture = pixels::SurfaceTexture::new(size.width, size.height, &window);

    //frame buffer "pixels"
    let mut pixels = Pixels::new(size.width, size.height, surface_texture)?;

    let screen = Screen::new("WorldData/houses.txt");


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
struct _Player {
    x_pos: u16,
    y_pos: u16,
}

struct Screen {
    //baddies: Vec<Baddie>,
    area: Vec<u8>,
}
impl Screen {
    fn new(place: &str) -> Self {
        Self {
            //baddies: vec![],
            //check the types that are used if errors, maybe &str ?
            area: std::fs::read(place).unwrap(),
        }
    }
    fn draw(&self, pix: &mut [u8]){
        // let e = pix.len();
        // println!("{}",e);
        //read the entire pixel map with fs::read
        //unwrap to take from result<Vec[u8],e> to Vec[u8]
        //let colors = std::fs::read("WorldData/Houses").unwrap();
        //iterator var
        let mut it:usize = 0;
        for pixel in pix.chunks_exact_mut(4) {
            //i*6 is the byte chunk
            let pos = it*6;
            //i hate this part
            //takes u8 at pos and turns into utf8
            //unwrap to take from Result to
            // let a = self.area.capacity();
            // println!("{}",a);

            println!("pixel is {:?}",pixel);
            let r: Vec<u8> = vec![self.area[pos], self.area[pos+1]];
            println!("r is {:?}",r);
            let red = std::str::from_utf8(&r).unwrap();
            println!("red is {:?}",red);
            let g: Vec<u8> = vec![self.area[pos+2], self.area[pos+3]];
            println!("g is {:?}",g);
            let green = std::str::from_utf8(&g).unwrap();
            println!("green is {:?}",green);
            let b: Vec<u8> = vec![self.area[pos+4], self.area[pos+5]];
            println!("b is {:?}",b);
            let blue = std::str::from_utf8(&b).unwrap();
            println!("blue is {:?}",blue);

            let asd = FromHex(red);

            //Shoves string pointer into u8 sized hole
            pixel[0] = red.parse().unwrap(); // R
            pixel[1] = green.parse().unwrap(); // G
            pixel[2] = blue.parse().unwrap(); // B
            pixel[3] = 0xFF; // A
            it += 1;
        }
    }
}
// fn _update(&mut self) {
//
// }
