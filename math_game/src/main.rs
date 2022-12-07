//extra functions for idiomatic code or wtv
mod render;

use pixels::Pixels;
//Dont just import all of pixels at some point
// use pixels::wgpu::Color;
use std::{
    env,
    time::SystemTime,
    mem,
    //     fs::*,
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
use rayon::prelude::*;
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
    sprite: Vec<u8>,
    //direction: u8
}
impl Player {
    //make &string into directory not file
    fn new(spr: &str) -> Self {
        Self {
            x_pos: START_X,
            y_pos: START_Y,
            sprite: std::fs::read(spr).unwrap(),
        }
    }
    fn mov(&mut self, dir: u8) {
        match dir {
            //Move up W
            1 if self.y_pos > 3 => self.y_pos -= 2,
            1 => {}
            //Move left A
            2 if self.x_pos > 3 => self.x_pos -= 2,
            2 => {}
            //Move down S
            3 if self.y_pos < 511 => self.y_pos += 2,
            3 => {}
            //Move right D
            4 if self.x_pos < 700 => self.x_pos += 2,
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
            area: std::fs::read(place).unwrap(),
        }
    }
    fn draw(&self, pix: &mut [u8]) {
        // let times = SystemTime::now();
        //iterator var
        let mut fb = self.area.clone();
        //fb.get_mut(((720*self.player.x_pos + self.player.y_pos) as usize)..(((720*self.player.x_pos + self.player.y_pos) as usize )+self.player.sprite.len())) = &self.player.sprite;
        //for (i, bit) in fb.get_mut(((720*self.player.x_pos + self.player.y_pos) as usize)..(((720*self.player.x_pos + self.player.y_pos) as usize )+self.player.sprite.len())).into_iter().enumerate(){
        //     bit = &mut [self.player.sprite[i]];
        // }
        //^^ failed ideas that im keeping because they could be useful

        //below here needs to be recommented
        // let (b4,l8) = fb.split_at_mut((720*self.player.x_pos + self.player.y_pos) as usize);
        // let (_,l8r) = l8.split_at_mut((720*self.player.x_pos + self.player.y_pos) as usize + self.player.sprite.len());
        // let good = &mut self.player.sprite
        // .as_slice()
        // .to_owned();
        // fb = [b4,good,l8r].concat();

        //entities are 18x27
        //whole thing takes about 8ms
        //each iteration is about 300 microseconds

        const BYTE_LEN: u64 = 6;
        const SCREEN_WIDTH: u64 = 720;
        for v in 0..27 {
            //0-26
            //println!("{}",v);
            //about 130 nanoseconds
            let x = ((BYTE_LEN * SCREEN_WIDTH * self.player.y_pos as u64) + (BYTE_LEN * (self.player.x_pos as u64)) + (BYTE_LEN * SCREEN_WIDTH * v))
                as u64;
            // let a = mem::size_of_val(&x);
            // println!("{}",x);
            // println!("{}",a);
            let (b4, l8) = fb.split_at(x as usize);
            //println!{"the split point is {}",p}
            //either 70 or 200 nanoseconds
            let (_, l8r) = l8.split_at(108);

            //about 100 nanoseconds again with 400 spikes
            let good = &(self
                .player
                .sprite
                .get((BYTE_LEN * 18 * v) as usize..(BYTE_LEN * 18 * (v + 1)) as usize)
                .unwrap());

            //400-500 microseconds
            fb = [b4, good, l8r].concat();


            //test infodumps
            //println!("the first part is {}, and the second is {}, and this is equal to {}",b4.len(),l8.len(),b4.len() + l8.len());
            //println!("the discrepancy of the first part is {} \n",(b4.len() + l8.len())-(a.len() + l8r.len()));
        }
        //until this point is about 8ms, too long id say
        //framebuffer dump for errors
        //std::fs::write("asdf", &fb).unwrap();


        //ENTIRE thing takes 1.8 microseconds
        let mut r:[u8;2];
        let mut g:[u8;2];
        let mut b:[u8;2];
        // let mut it:usize = 0;
        // let pix_iter = pix.as_parallel_slice_mut();
        // pix_iter.par_chunks_exact_mut(4).for_each( |pixel| {
        for (it, pixel) in pix.chunks_exact_mut(4).enumerate() {
            pixel[3] = 0xFF;
            //i*6 is the byte chunk
            //either 0 or 100 ns, avg about 25
            let pos = it * 6;

            //takes two u8 and puts together
            //either 100 or 200ns, avg about 150
            r = [fb[pos], fb[pos + 1]];
            //kinda hacky workaround to turn two &str into a valid hex byte
            //again either 0 or 100ns averaging about 25
            let red = std::str::from_utf8(&r).unwrap();
            //this whole bit takes about 200ns reliably

            g = [fb[pos + 2], fb[pos + 3]];
            let green = std::str::from_utf8(&g).unwrap();

            b = [fb[pos + 4], fb[pos + 5]];
            let blue = std::str::from_utf8(&b).unwrap();

            //test for transparency in image
            //takes 100-200ns
            let st = format!("{}{}{}", red, green, blue);
            if st.as_str() == "000000" {
                pixel[0] = {
                    let r2 = [self.area[pos], self.area[pos + 1]];
                    let red2 = std::str::from_utf8(&r2).unwrap();
                    u8::from_str_radix(red2, 16).unwrap()
                };
                pixel[1] = {
                    let b2 = [self.area[pos + 2], self.area[pos + 3]];
                    let blu2 = std::str::from_utf8(&b2).unwrap();
                    u8::from_str_radix(blu2, 16).unwrap()
                };
                pixel[2] = {
                    let g2 = [self.area[pos + 2], self.area[pos + 3]];
                    let gre2 = std::str::from_utf8(&g2).unwrap();
                    u8::from_str_radix(gre2, 16).unwrap()
                };
                continue;
            }

            //takes it from u8 bytes to &str UTF-8
            //sets red value in the thing into the hex value contained in red
            pixel[0] = u8::from_str_radix(red, 16).unwrap(); // R
            pixel[1]  = u8::from_str_radix(green, 16).unwrap(); // G
            pixel[2] = u8::from_str_radix(blue, 16).unwrap(); // B

            // let blue = std::str::from_utf8().unwrap();
            //Sets transparency value to none because that isnt needed
            // let time_n = SystemTime::now();
            // let diff = time_n.duration_since(times).unwrap();
            // println!("{:?}", diff);
        }
    }
}
// fn _update(&mut self, sc) -> std::io::Result <()> {
//     std::fs::copy(self.place,"screen.txt");
//
// }
