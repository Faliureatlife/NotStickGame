//extra functions for idiomatic code or wtv
mod render;

use pixels::Pixels;
//Dont just import all of pixels at some point
// use pixels::wgpu::Color;
use std::{
    env,
    // time::SystemTime,
    // mem,
    // io::Write,
    // fs::*,
    //     time::Duration,
    //     thread::sleep,
    u8,
};
use winit::{
    dpi::PhysicalSize,
    //dpi::PhysicalSize,
    event::*,
    event_loop::*,
    window::Window,
};
// use rayon::prelude::*;
use winit_input_helper::WinitInputHelper;

//starting position of player
const START_Y: u16 = 0;
const START_X: u16 = 0;

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

    //screen object that has the text.txt source file
    let mut screen = Screen::new("WorldData/test.txt");
    //loop that runs program
    //todo: multithreading to have game thinking and rendering at same time
    event_loop.run(move |event, _, control_flow| {
        //When it wants to redraw do this
        if let Event::RedrawRequested(_) = event {
            //framebuffer that we shall mut
            screen.draw(pixels.get_frame());
            //do the thinking for the drawing process
            //render the frame buffer and panic if it has something passed to it
            if pixels
                .render()
                .map_err(|e| panic!("pixels.render() failed: {}", e))
                .is_err()
            {
                //after the panic close the process
                *control_flow = ControlFlow::Exit;
                return;
            }
        }
        //update part of code that handles key-presses and simple window things
        if input.update(&event) {
            //make into a match statement at some point maybe
            //close on pressing esc
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            if input.key_pressed_os(VirtualKeyCode::W) {
                screen.player.mov(1);
                return;
            }
            if input.key_pressed_os(VirtualKeyCode::A) {
                screen.player.mov(2);
                return;
            }
            if input.key_pressed_os(VirtualKeyCode::S) {
                screen.player.mov(3);
                return;
            }
            if input.key_pressed_os(VirtualKeyCode::D) {
                screen.player.mov(4);
                return;
            }
            //after updates happen redraw the screen
            window.request_redraw();

        };
    });
    //Ok(())
    //use to crash program safely
    //
}
struct Player {
    //top right of player
    x_pos: u16,
    y_pos: u16,
    move_state:u8,
    // data:
    sprite: Vec<u8>,
    //direction: u8
}
impl Player {
    //make &string into directory not file
    fn new(spr: &str) -> Self {
        Self {
            x_pos: START_X,
            y_pos: START_Y,
            move_state: 0,

            sprite: std::fs::read(spr).unwrap(),
        }
    }
    fn mov(&mut self, dir: u8) {
        match dir {
            //Move up W
            1 if self.y_pos > 3 => {self.y_pos -= 2;
                                    self.move_state +=1;},
            1 => {}
            //Move left A
            2 if self.x_pos > 3 => {self.x_pos -= 2;
                                    self.move_state +=1;},
            2 => {}
            //Move down S
            3 if self.y_pos < 511 => {self.y_pos += 2;
                                    self.move_state +=1;},
            3 => {}
            //Move right D
            4 if self.x_pos < 700 => {self.x_pos += 2;
                                    self.move_state +=1;},
            4 => {}
            _ => panic!("Invalid movement"),
        }
    }
}

struct Screen {
    player: Player,
    //player_pos: enum
    //triggers: idk
    //baddies: Vec<Baddie>,
    area: Vec<u8>,
}

impl Screen {
   fn new(place: &str) -> Self {
       Self {
           player: Player::new("SpriteData/Nav/up/back_nav0.txt"),
           //baddies: vec![],
           //check the types that are used if errors, maybe &str ?
		   //area maybe as an array, convert to slice later on?
           area: Screen::new_screen(place),

       // add in the whole framebuffer thing and just copy it later
       // render adds in just the characters on top
       // i hate myself
	   //render all of place into a vec<u8> same as pixels
       // }
       }
   }
    fn new_screen(place: &str) -> Vec<u8> {

        let mut data: Vec<u8> = vec![];
        let mut real_data: Vec<u8> = vec![];
        for pix in std::fs::read(place).unwrap().chunks_exact_mut(2){
            //std::str::from_utf8(&g).unwrap()
            //u8::from_str_radix(blu2, 16).unwrap()
            //gives a vec<u8> of all "valid" bits for the fb without the added opacity bits
            data.push(u8::from_str_radix(std::str::from_utf8(pix).unwrap(),16).unwrap());
            // write!(a,"{:.?}", "{b}")
        }

        for (it,x) in data.into_iter().enumerate() {
            if it % 4 == 0 {
                real_data.push(0);
            }
            real_data.push(x);
        }
        let a = format!("{:?}",&real_data);
        std::fs::write("with_opacity.txt", a).unwrap();
        println!("File created");
        //output the whole thing
        real_data
    }
//pix never used but needed in order to draw to framebuffer
    fn draw(&self, _pix: &mut [u8]) {
        // let times = SystemTime::now();
        //iterator var
        let mut fb = self.area.clone();
        //entities are 18x27
        //whole thing takes about 8ms
        //each iteration is about 300 microseconds

        const BYTE_LEN: u64 = 6;
        const SCREEN_WIDTH: u64 = 720;
        for v in 0..27 {
            // for it in 0 .. 18 {
            //     if std::str::from_utf8(self.player.sprite.get((BYTE_LEN*v+it) as usize .. (BYTE_LEN*v+it+BYTE_LEN) as usize).unwrap()).unwrap() == "000000" {
            //
            //     }
            // }
            //0-26
            //println!("{}",v);
            //about 130 nanoseconds
            let x = (BYTE_LEN * SCREEN_WIDTH * self.player.y_pos as u64) + (BYTE_LEN * (self.player.x_pos as u64)) + (BYTE_LEN * SCREEN_WIDTH * v);
            // let a = mem::size_of_val(&x);
            // println!("{}",x);
            // println!("{}",a);
            let (b4, l8) = fb.split_at(x as usize);
            //println!{"the split point is {}",p}
            //either 70 or 200 nanoseconds
            let (_, l8r) = l8.split_at(108);

            //about 100 nanoseconds again with 400 spikes
            let good = self
                .player
                .sprite
                .get((BYTE_LEN * 18 * v) as usize..(BYTE_LEN * 18 * (v + 1)) as usize)
                .unwrap();

            //400-500 microseconds
            fb = [b4, good, l8r].concat();

            //test infodumps
            //println!("the first part is {}, and the second is {}, and this is equal to {}",b4.len(),l8.len(),b4.len() + l8.len());
            //println!("the discrepancy of the first part is {} \n",(b4.len() + l8.len())-(a.len() + l8r.len()));
        }
        // for (it, pixel) in pix.chunks_exact_mut(4).enumerate() {
        //     pixel[0] = fb[it*6];
        //     pixel[1] = fb[it*6+2];
        //     pixel[2] = fb[it*6+4];
        //     pixel[3] = 0xFF;
        // }
        // let time_n = SystemTime::now();
        // let diff = time_n.duration_since(times).unwrap();
        // println!("{:?}", diff);
    }
}
// fn _update(&mut self, sc) -> std::io::Result <()> {
//     std::fs::copy(self.place,"screen.txt");
//
// }
