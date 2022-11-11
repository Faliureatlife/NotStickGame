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
 const start_y: u16 = 0;
 const start_x: u16 = 0;

fn main() -> Result<(), pixels::Error> {
    //where event loop is created for the future event_loop.run
    let event_loop = EventLoop::new();

    //handle inputs with winit_input_helper
    let mut input = WinitInputHelper::new();

    //set env variable to give simple backtrace of broken runtime code
    let var = "RUST_BACKTRACE";
    env::set_var(var, "1");

    //Create window and give it Physical Size of 720 4:3
    let window = Window::new(&event_loop).unwrap();
    window.set_inner_size(PhysicalSize::new(720, 540));
    //let size = window.inner_size();

    //Create surface texture of given width and height with deref window
    let surface_texture = pixels::SurfaceTexture::new(720, 540, &window);

    //frame buffer "pixels"
    let mut pixels = Pixels::new(720, 540, surface_texture)?;

    //screen object that has the text.txt souce file
    let screen = Screen::new("WorldData/test.txt");

    //loop that runs program
    //todo: multithread to have game thinking and rendering at same time
    event_loop.run(move |event, _, control_flow| {
        //When it wants to redraw do this
        if let Event::RedrawRequested(_) = event {
            //framebuffer that we shall mut
            let pix = pixels.get_frame();
            screen.draw(pix);
            //do the thinking for the drawing process
            //render the frame buffer and panic if it has something passed to it
            if pixels
            .render()
            .map_err(|e| panic!("pixels.render() failed: {}",e))
            .is_err()
            {
                //after the panic close the process
                *control_flow = ControlFlow::Exit;
                return;
            }
        }
        //update part of code that handles keypresses and simple window things
        if input.update(&event) {
            //close on pressing esc
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            //after updates happen redraw the screen
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
struct Player {
    x_pos: u16,
    y_pos: u16,
}
impl Player {
    fn new() -> Self {
        Self {
            x_pos: start_x,
            y_pos: start_y,
        }
    }

}
struct Screen {
    player: Player,
    //baddies: Vec<Baddie>,
    area: Vec<u8>,
}
impl Screen {
    fn new(place: &str) -> Self {
        Self {
            player: Player::new(),
            //baddies: vec![],
            //check the types that are used if errors, maybe &str ?
            area: std::fs::read(place).unwrap(),

        }
    }
    fn draw(&self, pix: &mut [u8]){
        //read the entire pixel map with fs::read
        //unwrap to take from result<Vec[u8],e> to Vec[u8]
        //let colors = std::fs::read("WorldData/Houses").unwrap();
        //iterator var

        let mut it:usize = 0;
        for pixel in pix.chunks_exact_mut(4) {
            //i*6 is the byte chunk
            let pos = it*6;
            //i hate this part
            // let a = self.area.capacity();
            // println!("{}",a);

            //takes u8 at pos and turns into utf8
            //unwrap to take from Result to

            //kinda hacky workaround to turn two &str into a valid hex byte
            //takes two u8 and puts together
            let r: Vec<u8> = vec![self.area[pos], self.area[pos+1]];
            //takes it from u8 bytes to &str UTF-8
            let red = std::str::from_utf8(&r).unwrap();
            //sets red value in the thing into the hex value contained in red
            pixel[0] = u8::from_str_radix(red,16).unwrap(); // R

            let g: Vec<u8> = vec![self.area[pos+2], self.area[pos+3]];
            let green = std::str::from_utf8(&g).unwrap();
            pixel[1] = u8::from_str_radix(green,16).unwrap(); // G

            let b: Vec<u8> = vec![self.area[pos+4], self.area[pos+5]];
            let blue = std::str::from_utf8(&b).unwrap();
            pixel[2] = u8::from_str_radix(blue,16).unwrap(); // B
            //Sets transparency value to none because that is stupid
            pixel[3] = 0xFF; // A
            it += 1;
        }
    }
}
// fn _update(&mut self) {
//
// }
