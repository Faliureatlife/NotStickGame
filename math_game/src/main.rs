//extra functions for idiomadic code or wtv
mod render;
use pixels::Pixels;
//Dont just import all of pixels at some point
// use pixels::wgpu::Color;
use winit::{
    dpi::PhysicalSize,
    //dpi::PhysicalSize,
    event::*,
    event_loop::*,
    window::Window,
};
use winit_input_helper::WinitInputHelper;
use std::{
     env,
     u8,
//     fs::*,
//     time::Duration,
//     thread::sleep,
 };

fn main() -> Result<(), pixels::Error> {
    //where event loop is created for future event_loop.run
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let var = "RUST_BACKTRACE";
    env::set_var(var, "1");
    //Create window and give it Logical Size of 720 4:3
    let window = Window::new(&event_loop).unwrap();
    window.set_inner_size(PhysicalSize::new(720, 540));
    //let size = window.inner_size();

    //Create surface texture of given width and height with deref window
    let surface_texture = pixels::SurfaceTexture::new(720, 540, &window);

    //frame buffer "pixels"
    let mut pixels = Pixels::new(720, 540, surface_texture)?;

    let screen = Screen::new("WorldData/test.txt");

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            let pix = pixels.get_frame();
            screen.draw(pix);
            if pixels
            .render()
            .map_err(|e| panic!("pixels.render() failed: {}",e))
            .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }
        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            window.request_redraw();
        }

        // *control_flow = ControlFlow::Wait;
        // pixels.render().unwrap();
        // //close window
        // match event {
        //     Event::WindowEvent {
        //         event: WindowEvent::CloseRequested,
        //         window_id,
        //     } if window_id == window.id() => *control_flow = ControlFlow::Exit,
        //     _ => (),
        // }
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
        let e = pix.len();
        println!("the length of pix is {}, and area should be {}",e,(e*4));
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

            let r: Vec<u8> = vec![self.area[pos], self.area[pos+1]];
            let red = std::str::from_utf8(&r).unwrap();
            pixel[0] = u8::from_str_radix(red,16).unwrap(); // R

            let g: Vec<u8> = vec![self.area[pos+2], self.area[pos+3]];
            let green = std::str::from_utf8(&g).unwrap();
            pixel[1] = u8::from_str_radix(green,16).unwrap(); // G

            let b: Vec<u8> = vec![self.area[pos+4], self.area[pos+5]];
            let blue = std::str::from_utf8(&b).unwrap();
            pixel[2] = u8::from_str_radix(blue,16).unwrap(); // B
            //Shoves string pointer into u8 sized hole
            pixel[3] = 0xFF; // A
            it += 1;
        }
    }
}
// fn _update(&mut self) {
//
// }
