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
    env::set_var(var, "0");

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
}

struct Screen {
    player: Player,
    //triggers: idk
    //baddies: Vec<Baddie>,
    area: Vec<u8>,
}

impl Screen {
    fn new(place: &str) -> Self {
        Self {
            player: Player::new("SpriteData/Nav/back_nav0.txt"),
            //baddies: vec![],
            //check the types that are used if errors, maybe &str ?
            area: std::fs::read(place).unwrap(),
        }
    }

    fn draw(&self, pix: &mut [u8]){
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
        for v in 0..27{   //0-26
            println!("{}",v);
            let p:u32 = ((6*720*self.player.y_pos)+ 6*(self.player.x_pos)) as u32 +(6*720*v);
            let (b4,l8) = fb.split_at(p as usize);
            println!{"the split point is {}",p}
            let (_,l8r) = l8.split_at(108 as usize);
            //THIS LINE IS NOT THE PROBLEM
            //jk lol it is
            let good = &(self.player.sprite.get((6*18*v) as usize..(6*18*v + 18*6) as usize).unwrap());
            fb = [b4,good,l8r].concat();
//test infodumps
//println!("the first part is {}, and the second is {}, and this is equal to {}",b4.len(),l8.len(),b4.len() + l8.len());
//println!("the discrepancy of the first part is {} \n",(b4.len() + l8.len())-(a.len() + l8r.len()));
        }
        std::fs::write("asdf", &fb).unwrap();


        for (it, pixel) in pix.chunks_exact_mut(4).enumerate() {
            //i*6 is the byte chunk
            let pos = it*6;
            //i hate this part
            // let a = self.area.capacity();
            // println!("{}",a);

            //takes u8 at pos and turns into utf8
            //unwrap to take from Result to

            //kinda hacky workaround to turn two &str into a valid hex byte
            //takes two u8 and puts together
            let r: Vec<u8> = vec![fb[pos], fb[pos+1]];
            //takes it from u8 bytes to &str UTF-8
            let red = std::str::from_utf8(&r).unwrap();
            //sets red value in the thing into the hex value contained in red
            pixel[0] =  u8::from_str_radix(red,16).unwrap(); // R

            let g: Vec<u8> = vec![fb[pos+2], fb[pos+3]];
            let green = std::str::from_utf8(&g).unwrap();
            pixel[1] = u8::from_str_radix(green,16).unwrap(); // G

            let b: Vec<u8> = vec![fb[pos+4], fb[pos+5]];
            let blue = std::str::from_utf8(&b).unwrap();
            pixel[2] = u8::from_str_radix(blue,16).unwrap(); // B
            //Sets transparency value to none because that is stupid
            pixel[3] = 0xFF;
            let st = format!("{}{}{}",red,green,blue);
            if st.as_str() == "000000" {
                pixel[0] = {let r2 = vec![self.area[pos],self.area[pos + 1]];
                            let red2 = std::str::from_utf8(&r2).unwrap();
                            u8::from_str_radix(red2,16).unwrap()};
                pixel[1] = {let b2 = vec![self.area[pos+2],self.area[pos + 3]];
                            let blu2 = std::str::from_utf8(&b2).unwrap();
                            u8::from_str_radix(blu2,16).unwrap()};
                pixel[2] = {let g2 = vec![self.area[pos+2],self.area[pos + 3]];
                            let gre2 = std::str::from_utf8(&g2).unwrap();
                            u8::from_str_radix(gre2,16).unwrap()};
            }
        }
    }
}
// fn _update(&mut self, sc) -> std::io::Result <()> {
//     std::fs::copy(self.place,"screen.txt");
//
// }
