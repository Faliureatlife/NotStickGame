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
    // u8,
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
const START_Y: u16 = 10;
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
    for pixel in pixels.get_frame().chunks_exact_mut(4) {
        pixel[3] = 255;
    }
    //screen object that has the text.txt source file
    let mut screen = Screen::new("WorldData/dots.txt");
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
            //Todo: Diagonal movement
            if input.key_pressed_os(VirtualKeyCode::W) || input.key_pressed_os(VirtualKeyCode::Up) {
                screen.player.mov(1);
                return;
            }
            if input.key_pressed_os(VirtualKeyCode::A) || input.key_pressed_os(VirtualKeyCode::Left){
                screen.player.mov(2);
                return;
            }
            if input.key_pressed_os(VirtualKeyCode::S) || input.key_pressed_os(VirtualKeyCode::Down){
                screen.player.mov(3);
                return;
            }
            if input.key_pressed_os(VirtualKeyCode::D) || input.key_pressed_os(VirtualKeyCode::Right){
                screen.player.mov(4);
                return;
            }
            if input.key_released(VirtualKeyCode::W)
                || input.key_released(VirtualKeyCode::A)
                || input.key_released(VirtualKeyCode::S)
                || input.key_released(VirtualKeyCode::D)
                || input.key_released(VirtualKeyCode::Up)
                || input.key_released(VirtualKeyCode::Left)
                || input.key_released(VirtualKeyCode::Down)
                || input.key_released(VirtualKeyCode::Right){
                screen.player.move_state = 0;
                screen.player.move_delay = 0;
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
    move_state: u8,
    // data:
    sprite: [[Vec<u8>; 4]; 4],
    direction: u8,
    move_delay: u8,
    collision: Vec<u16>,
}
impl Player {
    //make &string into directory not file
    fn new(
        spr0: &str,
        spr1: &str,
        spr2: &str,
        spr3: &str,
        spr4: &str,
        spr5: &str,
        spr6: &str,
        spr7: &str,
        spr8: &str,
        spr9: &str,
        spr10: &str,
        spr11: &str,
        spr12: &str,
        spr13: &str,
        spr14: &str,
        spr15: &str,
    ) -> Self {
        Self {
            x_pos: START_X,
            y_pos: START_Y,
            move_state: 0,
            sprite: [
                [
                    Player::gen_sprite(spr0),
                    Player::gen_sprite(spr1),
                    Player::gen_sprite(spr2),
                    Player::gen_sprite(spr3),
                ],
                [
                    Player::gen_sprite(spr4),
                    Player::gen_sprite(spr5),
                    Player::gen_sprite(spr6),
                    Player::gen_sprite(spr7),
                ],
                [
                    Player::gen_sprite(spr8),
                    Player::gen_sprite(spr9),
                    Player::gen_sprite(spr10),
                    Player::gen_sprite(spr11),
                ],
                [
                    Player::gen_sprite(spr12),
                    Player::gen_sprite(spr13),
                    Player::gen_sprite(spr14),
                    Player::gen_sprite(spr15),
                ],
            ],
            direction: 0,
            move_delay: 0,
            collision: vec![60,60],
        }
    }

    fn gen_sprite(spr: &str) -> Vec<u8> {
        let mut data = vec![];
        for pix in std::fs::read(spr).unwrap().chunks_exact(2) {
            data.push(u8::from_str_radix(std::str::from_utf8(pix).unwrap(), 16).unwrap());
        }
        data
    }
    fn mov(&mut self, dir: u8) {
        const MVMT_D:u16 = 2;
        let mut bad:bool = false;
        println!("x1 {}", self.x_pos);
        println!("x2 {}", self.x_pos+18);
        println!("y1 {}", self.y_pos);
        println!("y2 {}", self.y_pos+27);
        match dir {
            //TODO: make the movement flush with edges
            //TODO: use different sprites for movement
            //Move up W
            1 if self.y_pos > 1 => {
                for colliders in self.collision.chunks_exact(2) {
                    if colliders[0] > self.x_pos && colliders[0] < self.x_pos+18 && colliders[1] > (self.y_pos - MVMT_D) && colliders[1] < (self.y_pos - MVMT_D + 27) {
                        bad = !bad;
                        break
                    }
                }
                if !bad {
                    self.y_pos -= 2;
                }
                self.move_delay += 1;
                self.direction = 1;
            }
            1 => {}
            //Move left A
            2 if self.x_pos > 1 => {
                for colliders in self.collision.chunks_exact(2) {
                    if colliders[0] > self.x_pos - MVMT_D && colliders[0] < self.x_pos + 18 - MVMT_D && colliders[1] > self.y_pos && colliders[1] < self.y_pos + 27 {
                        bad = !bad;
                        break
                    }
                }
                if !bad {
                    self.x_pos -=2;
                }
                self.move_delay += 1;
                self.direction = 2;
            }
            2 => {}
            //Move down S
            3 if self.y_pos < 511 => {
                for colliders in self.collision.chunks_exact(2) {
                    if colliders[0] > self.x_pos && colliders[0] < self.x_pos+18 && colliders[1] > (self.y_pos + MVMT_D) && colliders[1] < (self.y_pos + MVMT_D + 27) {
                        bad = !bad;
                        break
                    }
                }
                if !bad {
                    self.y_pos += 2;
                }

                self.move_delay += 1;
                self.direction = 0;
            }
            3 => {}
            //Move right D
            4 if self.x_pos < 700 => {
                for colliders in self.collision.chunks_exact(2) {
                    if colliders[0] > self.x_pos + MVMT_D && colliders[0] < self.x_pos+18+MVMT_D && colliders[1] > self.y_pos && colliders[1] < self.y_pos + 27 {
                        bad = !bad;
                        break
                    }
                }
                if !bad {
                    self.x_pos += 2;
                }
                self.move_delay += 1;
                self.direction = 3;
            }
            4 => {}
            _ => panic!("Invalid movement"),
        }
        if self.move_delay == 3 {
            self.move_delay -= 3;
            self.move_state += 1;
        }
        if self.move_state == 4 {
            self.move_state -= 4;
        }
    }
}

struct Screen {
    player: Player,
    // collision:Vec<u16>,
    //triggers: idk
    //baddies: Vec<Baddie>,
    area: Vec<u8>,
}

impl Screen {
    fn new(place: &str) -> Self {
        Self {
            player: Player::new(
                "SpriteData/Nav/down/front_nav0.txt",
                "SpriteData/Nav/down/front_nav1.txt",
                "SpriteData/Nav/down/front_nav2.txt",
                "SpriteData/Nav/down/front_nav3.txt",
                "SpriteData/Nav/up/back_nav0.txt",
                "SpriteData/Nav/up/back_nav1.txt",
                "SpriteData/Nav/up/back_nav2.txt",
                "SpriteData/Nav/up/back_nav3.txt",
                "SpriteData/Nav/left/L0.txt",
                "SpriteData/Nav/left/L1.txt",
                "SpriteData/Nav/left/L2.txt",
                "SpriteData/Nav/left/L3.txt",
                "SpriteData/Nav/right/R0.txt",
                "SpriteData/Nav/right/R1.txt",
                "SpriteData/Nav/right/R2.txt",
                "SpriteData/Nav/right/R3.txt",
            ),
            // collision: vec![],
            //baddies: vec![],
            area: Screen::new_screen(place),
            // i hate myself
        }
    }
    fn new_screen(place: &str) -> Vec<u8> {
        let mut data = vec![];
        for pix in std::fs::read(place).unwrap().chunks_exact(2) {
            //std::str::from_utf8(&g).unwrap()
            //u8::from_str_radix(blu2, 16).unwrap()
            //gives a vec<u8> of all "valid" bits for the fb without the added opacity bits
            data.push(u8::from_str_radix(std::str::from_utf8(pix).unwrap(), 16).unwrap());
        }
        data
    }
    fn draw(&self, pix: &mut [u8]) {
        //TODO: Update in chunks
        //TODO: Use premade transparency values
        let mut it2: usize = 0;
        for (it, pixel) in pix.chunks_exact_mut(4).enumerate() {
            /*Four checks:
            it % 720 > x_pos
            it % 720 < x_pos + 19
            it / 720 > y_pos
            it / 720 < y_pos + 28
            */
            if it % 720 > self.player.x_pos as usize
                && it % 720 < (self.player.x_pos + 19) as usize
                && it / 720 > self.player.y_pos as usize
                && it / 720 < (self.player.y_pos + 28) as usize
            {
                if (self.player.sprite[self.player.direction as usize]
                    [self.player.move_state as usize][(it2) * 3] as u16)
                    + (self.player.sprite[self.player.direction as usize]
                        [self.player.move_state as usize][(it2) * 3 + 1]
                        as u16)
                    + (self.player.sprite[self.player.direction as usize]
                        [self.player.move_state as usize][(it2) * 3 + 2]
                        as u16)
                    == 0
                {
                    pixel[0] = self.area[it * 3];
                    pixel[1] = self.area[it * 3 + 1];
                    pixel[2] = self.area[it * 3 + 2];
                } else {
                    // println!("char {}",it);
                    pixel[0] = self.player.sprite[self.player.direction as usize]
                        [self.player.move_state as usize][(it2) * 3];
                    //do the expect thing tomorrow
                    pixel[1] = self.player.sprite[self.player.direction as usize]
                        [self.player.move_state as usize][(it2) * 3 + 1];
                    pixel[2] = self.player.sprite[self.player.direction as usize]
                        [self.player.move_state as usize][(it2) * 3 + 2];
                    // pixel[3] = 255;
                }
                it2 += 1;
            } else {
                // println!("b {}",it);
                pixel[0] = self.area[it * 3];
                pixel[1] = self.area[it * 3 + 1];
                pixel[2] = self.area[it * 3 + 2];
                // pixel[3] = 255;
            }
        }

        //0-388799 it, should be right amt
        //testing the fb contents
        // let a = format!("{:?}",&pix);
        // std::fs::write("framebuffer.txt", a).unwrap();
        // println!("File created");
    }
}
// fn _update(&mut self, sc) -> std::io::Result <()> {
//     std::fs::copy(self.place,"screen.txt");
//
// }
