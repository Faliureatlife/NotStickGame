//extra functions for idiomatic code or wtv
//todo: make moving work

//jacob cringe rust book import
use rand::prelude::*;
//screen access
use pixels::{
    wgpu::{PowerPreference, RequestAdapterOptions,PresentMode},
    PixelsBuilder,
};
//audio
use rodio::{Decoder, OutputStream, source::Source, Sink};
//deserialize json files
use serde_json::{de, value::Value};
//whatever i need from the std library atm
use std::{
    env,
    fs::*,
    io::BufReader
    // thread::sleep,
};
//window management and input
use winit::event::VirtualKeyCode;
use winit::{dpi::PhysicalSize, event::*, event_loop::*, window::Window, };
use winit_input_helper::WinitInputHelper;
// use pixels::wgpu::Color;

// unused constants
// const START_Y: u16 = 10;
// const START_X: u16 = 0;
// const SCROLL_OFFSET:u16 = 10;
const WORLD: &str = "WorldData/";
const SCREEN_WIDTH: u16 = 720;
const SCREEN_HEIGHT: u16 = 540;
//real width + 1
const CHAR_WIDTH: u16 = 37;
const CHAR_HEIGHT: u16 = 54;
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
    window.set_inner_size(PhysicalSize::new(SCREEN_WIDTH, SCREEN_HEIGHT));
    //let size = window.inner_size();

    //Create surface texture of given width and height with deref window
    let surface_texture =
        pixels::SurfaceTexture::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, &window);

    //frame buffer "pixels"
    let mut pixels = PixelsBuilder::new(720, 540, surface_texture)
        .request_adapter_options(RequestAdapterOptions {
            power_preference: PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: None,
        })
        //Fifo
        .present_mode(PresentMode::Fifo)
        .build()?;

    //sets every fourth transparency pixel to 255
    for pixel in pixels.get_frame().chunks_exact_mut(4) {
        pixel[3] = 255;
    }

    //screen object made from the house page
    let mut screen = Screen::new("houses");
    let mut mvmt_dist: u16 = 5;

    //music initialization
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = File::open("music/Stroll_Around_Town.wav").unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap().repeat_infinite();
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(source);
    sink.play();

    //setting the distance to be the correct value (add in to new() function later)
    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
    //declaring the direction moved values with initial value of false
    let mut up: bool = false;
    let mut left: bool = false;
    let mut down: bool = false;
    let mut right: bool = false;

    // Pause menu variables
    let mut x_save: u16 = screen.player.x_pos;
    let mut y_save: u16 = screen.player.y_pos;
    let mut paused:bool = false;
    let mut last_scr: String = format!("houses");
    let mut last_scroll: u16 = 0;
    let mut track: u8 = 0;

    let mut battle:bool = false;
    let mut fight:bool = false;
    let mut run:bool = false;
    let mut run_did:bool = false;
    //todo: multithreading to have game thinking and rendering at same time
    //loop that runs program
    event_loop.run(move |event, _, control_flow| {
        //When it wants to redraw do this
        if let Event::RedrawRequested(_) = event {
            //framebuffer that we shall mut
            screen.draw(pixels.get_frame());
            //do the thinking for the drawing process
            //render the frame buffer and panic if it has something passed to it
            if pixels
                .render()
                .map_err(|e| panic!("pixels.render() failed: {:?}", e))
                .is_err()
            {
                //after the panic close the process
                *control_flow = ControlFlow::Exit;
                return;
            }
        }
        //update part of code that handles key-presses and simple window things
        if input.update(&event) && !paused{

            //debug key
            if input.key_pressed(VirtualKeyCode::U) {

            }

            //mute/unmute sound
            if input.key_pressed(VirtualKeyCode::M) {
                match sink.is_paused() {
                    true => {
                        sink.play();
                    }
                    false => {
                        sink.pause();
                    }
                }
            }

            //close on pressing esc
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            if input.key_pressed(VirtualKeyCode::Tab) {
                last_scr = screen.scr.clone();
                last_scroll = screen.scroll_dist;
                x_save = screen.player.x_pos;
                y_save = screen.player.y_pos;
                screen = Screen::new("pause-menu/pause-menu-a");
                screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                paused = !paused;
            }
            //When w or up arrow pressed flip value of upwards movement
            if input.key_released(VirtualKeyCode::W)
                || input.key_pressed(VirtualKeyCode::W)
                || input.key_released(VirtualKeyCode::Up)
                || input.key_pressed(VirtualKeyCode::Up)
            {
                up = !up;
            }
            //When A or Left arrow pressed flip value of leftwards movement
            if input.key_released(VirtualKeyCode::A)
                || input.key_pressed(VirtualKeyCode::A)
                || input.key_released(VirtualKeyCode::Left)
                || input.key_pressed(VirtualKeyCode::Left)
            {
                left = !left;
            }
            //When S or Down arrow pressed flip value of downwards movement
            if input.key_released(VirtualKeyCode::S)
                || input.key_pressed(VirtualKeyCode::S)
                || input.key_released(VirtualKeyCode::Down)
                || input.key_pressed(VirtualKeyCode::Down)
            {
                down = !down;
            }
            //when D or Right arrow pressed flip value of rightwards movement
            if input.key_released(VirtualKeyCode::D)
                || input.key_pressed(VirtualKeyCode::D)
                || input.key_released(VirtualKeyCode::Right)
                || input.key_pressed(VirtualKeyCode::Right)
            {
                right = !right;
            }
            if input.key_pressed(VirtualKeyCode::E) {
                let mut check_x = screen.player.x_pos + screen.scroll_dist;
                let mut check_y = screen.player.y_pos;
                match screen.player.direction {
                    1 => check_y -= 30,
                    2 => check_x -= 30,
                    3 => check_y += 30 + CHAR_WIDTH,
                    4 => check_x += 30 + CHAR_HEIGHT,
                    _ => {}
                }
                println!("checking for x {},{}",check_x,check_x + CHAR_WIDTH);
                println!("checking for y {},{}",check_y,check_y + CHAR_HEIGHT);
                for (i, it) in screen.interact_pos.clone().chunks_exact(2).enumerate() {
                    if check_x < it[0]
                        && it[0] < check_x + CHAR_WIDTH
                        && check_y < it[1]
                        && it[1] < check_y + CHAR_HEIGHT
                    {
                        match screen.interact[i].as_str() {
                            "move" => {
                                screen = Screen::new(&screen.interact_action[i]);
                                screen.screen_len =
                                    screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                                last_scr = screen.scr.clone();
                            }
                             "dialogue" => {

                                 screen.new_dialog(screen.interact_action[i].clone());
                             }
                             //add new dialogue section, take string and turn into csv of each char which are gotten from the premade alphabet
                            _ => {}
                        }
                    }
                }
            }

            match screen.player.change_screen {
                1 => {
                    // println!("up");
                    let x = screen.player.x_pos;
                    screen = Screen::new(&screen.player.mvmt_destinations[0]);
                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                    screen.player.x_pos = x;
                    //bottom of screen offset by player height + mvmt distance
                    screen.player.y_pos = 540 - (CHAR_HEIGHT as u16 + mvmt_dist + 1);
                    last_scr = screen.scr.clone();
                }
                2 => {
                    let y = screen.player.y_pos;
                    screen = Screen::new(&screen.player.mvmt_destinations[1]);
                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                    screen.player.y_pos = y;
                    //left side of screen offset by player height + mvmt distance
                    screen.player.x_pos = 720 - (CHAR_WIDTH + mvmt_dist + 1);
                    screen.scroll_dist = (screen.screen_len - 720) as u16;
                    last_scr = screen.scr.clone();
                }
                3 => {
                    let x = screen.player.x_pos;
                    let scroll = screen.scroll_dist;
                    screen = Screen::new(&screen.player.mvmt_destinations[2]);
                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                    screen.player.x_pos = x;
                    screen.scroll_dist = scroll;
                    screen.player.y_pos = 0 + (mvmt_dist + 1);
                    last_scr = screen.scr.clone();
                }
                4 => {
                    let y = screen.player.y_pos;
                    screen = Screen::new(&screen.player.mvmt_destinations[3]);
                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                    screen.player.y_pos = y;
                    screen.player.x_pos = 0 + (mvmt_dist + 1);
                    screen.scroll_dist = 0;
                    last_scr = screen.scr.clone();
                }
                _ => {}
            }

            if up {
                screen.player.mov(1, screen.scroll_dist,mvmt_dist);
            }
            //move down if down using the mov function
            if down {
                screen.player.mov(3, screen.scroll_dist,mvmt_dist);
            }
            //move left or scroll if the updated position will be past the bounds
            if left {
                if screen.player.x_pos < 360 && screen.scroll_dist > 0 + mvmt_dist + 1 {
                    screen.scroll_dist -= mvmt_dist;
                    screen.player.move_delay += 1;
                    screen.player.direction = 2;
                } else {
                    screen.player.mov(2, screen.scroll_dist,mvmt_dist);
                }
            }
            //move right or scroll right if moved pos would be past the bounds
            if right {
                //first checking where player will be next move
                //second checking to make sure no bad negative overflows
                if screen.player.x_pos + mvmt_dist > 360 && screen.screen_len > 720
                    && screen.scroll_dist + mvmt_dist < (screen.screen_len - 720) as u16
                {
                    screen.scroll_dist += mvmt_dist;
                    screen.player.move_delay += 1;
                    screen.player.direction = 3;
                } else {
                    screen.player.mov(4, screen.scroll_dist,mvmt_dist);
                }
            }
            //delay the player movement to every three ticks
            if screen.player.move_delay >= 2 {
                screen.player.move_delay -= 2;
                screen.player.move_state += 1;
            }
            //reset player movement state if at max
            if screen.player.move_state == 4 {
                screen.player.move_state -= 4;
            }
            //upon stopping movement reset the delay and move state to return to neutral
            if input.key_released(VirtualKeyCode::W)
                || input.key_released(VirtualKeyCode::A)
                || input.key_released(VirtualKeyCode::S)
                || input.key_released(VirtualKeyCode::D)
                || input.key_released(VirtualKeyCode::Up)
                || input.key_released(VirtualKeyCode::Left)
                || input.key_released(VirtualKeyCode::Down)
                || input.key_released(VirtualKeyCode::Right)
            {
                screen.player.move_state = 0;
                screen.player.move_delay = 0;
            }
            //after updates happen redraw the screen
            window.request_redraw();
        };
        if paused {
            // Switches screen based on choice selected
            // 0. Save Select
            // 1. Load Select
            // 2. Bag Select
            // 3. Settings Select
            // 4. Quit Select
            match track % 5 {
                0 => {
                    screen = Screen::new("pause-menu/pause-menu-a");
                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                }
                1 => {
                    screen = Screen::new("pause-menu/pause-menu-b");
                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                }
                2 => {
                    screen = Screen::new("pause-menu/pause-menu-c");
                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                }
                3 => {
                    screen = Screen::new("pause-menu/pause-menu-d");
                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                }
                4 => {
                    screen = Screen::new("pause-menu/pause-menu-e");
                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                }
                _ => {}
            }
            // Closes program on Escape
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            // Moves option selected to previous option
            if input.key_pressed(VirtualKeyCode::A) || input.key_pressed(VirtualKeyCode::Left) {
                if track == 0 {
                    track = 254;
                } else {
                    track = track - 1;
                }
            }
            // Moves option selected to following option
            if input.key_pressed(VirtualKeyCode::D) || input.key_pressed(VirtualKeyCode::Right) {
                if track == 254 {
                    track = 0;
                } else {
                    track = track + 1;
                }
            }
            if input.key_pressed(VirtualKeyCode::Tab) && !input.key_held(VirtualKeyCode::Tab) {
                println!("AAAAAAAAAAAAA");
                screen = Screen::new(&last_scr);
                screen.scroll_dist = last_scroll;
                screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                screen.player.x_pos = x_save;
                screen.player.y_pos = y_save;
                track = 0;
                paused = !paused;
            }

            // Selects choice and runs appropriate code
            if input.key_pressed(VirtualKeyCode::Return) {
                match track % 5 {
                    4 => {
                        screen = Screen::new(&last_scr);
                        screen.scroll_dist = last_scroll;
                        screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                        screen.player.x_pos = x_save;
                        screen.player.y_pos = y_save;
                        track = 0;
                        paused = !paused;
                    }
                    _ => {}
                }
            }

            //after updates happen redraw the screen
            window.request_redraw();
        }
    });
    //Ok(())
    //use to crash program safely
    //
}

struct Player {
    //horizontal position from right of screen to left of player
    x_pos: u16,
    //vertical position from top of screen to top of player
    y_pos: u16,
    //1-4 animation frames
    move_state: u8,
    //4 different arrays for direction with 4 sub-arrays for each frame
    sprite: [[Vec<u8>; 4]; 4],
    //direction the player is facing
    direction: u8,
    //state of movement delay in ticks
    move_delay: u8,
    //vector of pairs that determine the points at which the player will collide
    collision: Vec<u16>,
    //list of valid locations for the player to move
    mvmt_destinations: Vec<String>,
    //1 - W, 2 - A, 3 - S, 4 - D
    change_screen: u8,
}

impl Player {
    //horrific number of params i dont feel like mutilating into looking pretty
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
        x: u16,
        y: u16,
        collision_pts: Vec<u16>,
        mvmt_destinations: Vec<String>,
    ) -> Self {
        // giving all variables the default values
        Self {
            x_pos: x,
            y_pos: y,
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
            //set the collision points from the file
            collision: collision_pts,
            mvmt_destinations,
            change_screen: 0,
        }
    }

    //used for turning single animation frame into readable bytes
    fn gen_sprite(spr: &str) -> Vec<u8> {
        //vector containing the sprite data to be returned
        let mut data = vec![];
        //loop through the file in by byte
        for pix in read(spr).expect("Failed to read from file").chunks_exact(2) {
            //append each value taken from hex byte to single u8 value
            data.push(
                u8::from_str_radix(
                    std::str::from_utf8(pix).expect("Failed to convert to utf8"),
                    16,
                )
                .expect("Failed to convert to hex value"),
            );
        }
        //return the vector with the info
        data
    }
    fn mov(&mut self, dir: u8, scrolled: u16, mvmt_dist: u16) -> Option<()> {
        //variable for whether or not collision is taking place
        let mut colliding: bool = false;
        Option::from(match dir {
            //Move up W
            1 if self.y_pos - mvmt_dist >= 2 => {
                //loop through all possible collision points
                for colliders in self.collision.chunks_exact(2) {
                    //check to see if character is or will be within any of the bounds
                    if colliders[0] >= self.x_pos + scrolled {
                        if colliders[0] >= scrolled + self.x_pos
                            && colliders[0] <= scrolled + self.x_pos + CHAR_WIDTH
                            && colliders[1] >= (self.y_pos - mvmt_dist)
                            && colliders[1] <= (self.y_pos - mvmt_dist + CHAR_HEIGHT)
                        {
                            //flips collision to true and break from for loop
                            colliding = !colliding;
                            break;
                        }
                    }
                }
                //if collision is not taking place then move by amount mvmt_dist
                if !colliding {
                    self.y_pos -= mvmt_dist;
                }
                //increase delay and set direction
                self.move_delay += 1;
                self.direction = 1;
            }
            1 if self.mvmt_destinations[0] != "null" && self.y_pos - mvmt_dist <= mvmt_dist => {
                self.change_screen = 1;
            }
            1 => {}
            //Move left A
            2 if self.x_pos - mvmt_dist > mvmt_dist => {
                //loop through all possible collision points
                for colliders in self.collision.chunks_exact(2) {
                    //check to see if character is or will be within any of the bounds
                        if colliders[0] >= scrolled + self.x_pos - mvmt_dist
                            && colliders[0] <= scrolled + self.x_pos + CHAR_WIDTH - mvmt_dist
                            && colliders[1] >= (self.y_pos)
                            && colliders[1] <= (self.y_pos + CHAR_HEIGHT)
                        {
                            //flips collision to true and break from for loop
                            // println!("AAAAAA {:.?}",colliders);
                            colliding = !colliding;
                            break;
                        }
                }
                //if collision is not taking place then move by amount mvmt_dist
                if !colliding {
                    self.x_pos -= mvmt_dist;
                }
                //increase delay and set direction
                self.move_delay += 1;
                self.direction = 2;
            }
            2 if self.mvmt_destinations[1] != "null" && self.x_pos - mvmt_dist <= mvmt_dist => {
                self.change_screen = 2;
            }
            2 => {}
            //Move down S
            3 if self.y_pos + mvmt_dist < 540 - CHAR_HEIGHT - mvmt_dist => {
                //loop through all possible collision points
                for colliders in self.collision.chunks_exact(2) {
                    //check to see if character is or will be within any of the bounds
                    if colliders[0] >= self.x_pos + scrolled {
                        if colliders[0] >= scrolled + self.x_pos
                            && colliders[0] <= scrolled + self.x_pos + CHAR_WIDTH
                            && colliders[1] >= (self.y_pos + mvmt_dist)
                            && colliders[1] <= (self.y_pos + mvmt_dist + CHAR_HEIGHT)
                        {
                            //flips collision to true and break from for loop
                            colliding = !colliding;
                            break;
                        }
                    }
                }
                //if collision is not taking place then move by amount mvmt_dist
                if !colliding {
                    self.y_pos += mvmt_dist;
                }
                //increase delay and set direction
                self.move_delay += 1;
                self.direction = 0;
            }
            3 if self.mvmt_destinations[2] != "null"
                && self.y_pos + mvmt_dist >= 540 - CHAR_HEIGHT - mvmt_dist =>
            {
                self.change_screen = 3;
            }
            3 => {}
            //Move right D
            4 if self.x_pos + mvmt_dist < 720 - CHAR_WIDTH => {
                //loop through all possible collision points
                for colliders in self.collision.chunks_exact(2) {
                    //check to see if character is or will be within any of the bounds
                    if colliders[0] >= self.x_pos + scrolled {
                        if colliders[0] >= scrolled + self.x_pos + mvmt_dist
                            && colliders[0] <= scrolled + self.x_pos + CHAR_WIDTH + mvmt_dist
                            && colliders[1] >= (self.y_pos)
                            && colliders[1] <= (self.y_pos + CHAR_HEIGHT)
                        {
                            //flips collision to true and break from for loop
                            colliding = !colliding;
                            break;
                        }
                    }
                }
                //if collision is not taking place then move by amount mvmt_dist
                if !colliding {
                    self.x_pos += mvmt_dist;
                }
                //increase delay and set direction
                self.move_delay += 1;
                self.direction = 3;
            }
            4 if self.mvmt_destinations[3] != "null"
                && self.x_pos + mvmt_dist >= SCREEN_WIDTH - CHAR_WIDTH - mvmt_dist =>
            {
                self.change_screen = 4;
            }
            4 => {}
            _ => {}
        })
    }
}

struct Screen {
    //the player object
    player: Player,
    //the list of entities that will be used
    //unused
    entities: Vec<Entity>,
    //the data for the background screen
    area: Vec<u8>,
    //the distance that the screen has scrolled
    scroll_dist: u16,
    //the length of the screen
    screen_len: usize,
    scr: String,
    interact: Vec<String>,
    interact_pos: Vec<u16>,
    interact_action: Vec<String>,
    music: String,
}
impl Screen {
    fn new(place: &str) -> Self {
        Self {
            //creating the new player objects
            player: Player::new(
                "SpriteData/Nav/down/0.txt",
                "SpriteData/Nav/down/1.txt",
                "SpriteData/Nav/down/2.txt",
                "SpriteData/Nav/down/3.txt",
                "SpriteData/Nav/up/0.txt",
                "SpriteData/Nav/up/1.txt",
                "SpriteData/Nav/up/2.txt",
                "SpriteData/Nav/up/3.txt",
                "SpriteData/Nav/left/0.txt",
                "SpriteData/Nav/left/1.txt",
                "SpriteData/Nav/left/2.txt",
                "SpriteData/Nav/left/3.txt",
                "SpriteData/Nav/right/0.txt",
                "SpriteData/Nav/right/1.txt",
                "SpriteData/Nav/right/2.txt",
                "SpriteData/Nav/right/3.txt",
                Screen::read_from_file_u16(
                    format!("{}{}{}", WORLD, place, "/data.json"),
                    "start_x",
                )
                .expect("Failed to read x value from file"),
                Screen::read_from_file_u16(
                    format!("{}{}{}", WORLD, place, "/data.json"),
                    "start_y",
                )
                .expect("Failed to read y value from file"),
                Screen::read_from_file_vecu16(
                    format!("{}{}{}", WORLD, place, "/data.json"),
                    "collision",
                )
                .expect("Failed to read collision from file"),
                Screen::read_from_file_vecstr(
                    format!("{}{}{}", WORLD, place, "/data.json"),
                    "mvmt_dest",
                )
                .expect("failed to read values"),
            ),
            entities: {
                let mut v: Vec<Entity> = vec![];
                for ent in Screen::read_from_file_vecstr(
                    format!("{}{}{}", WORLD, place, "/data.json"),
                    "entities",
                )
                .unwrap()
                {
                    v.push(Entity::new(
                        &format!("{}{}{}", "SpriteData/", ent, "/0.txt"),
                        &format!("{}{}{}", "SpriteData/", ent, "/1.txt"),
                        &format!("{}{}{}", "SpriteData/", ent, "/2.txt"),
                        &format!("{}{}{}", "SpriteData/", ent, "/3.txt"),
                        &format!("{}{}{}", "SpriteData/", ent, "/4.txt"),
                        &ent,
                    ));
                }
                v
            },
            //getting the data for a new screen
            area: Screen::new_screen(format!("{}{}{}", WORLD, place, "/picture.txt")),
            //default scroll dist is read from file
            scroll_dist: Screen::read_from_file_u16(
                format!("{}{}{}", WORLD, place, "/data.json"),
                "default_scroll",
            )
            .expect("Failed to read the default scroll distance of the screen from file"),

            //default scroll len is 0
            screen_len: 0,
            scr: place.to_owned(),
            interact: Screen::read_from_file_vecstr(
                format!("{}{}{}", WORLD, place, "/data.json"),
                "interact",
            )
            .expect("Failed to read interaction types"),

            interact_pos: Screen::read_from_file_vecu16(
                format!("{}{}{}", WORLD, place, "/data.json"),
                "interact_pos",
            )
            .expect("Failed to read interaction pos from file"),

            interact_action: Screen::read_from_file_vecstr(
                format!("{}{}{}", WORLD, place, "/data.json"),
                "interact_actions",
            )
            .expect("Failed to read interaction types"),

            music: {
                Screen::read_from_file_str(
                    format!("{}{}{}", WORLD, place, "/data.json"),
                    "music",
                ).expect("Failed to read background music")
            }
        }
    }
    fn read_from_file_u16(path: String, get: &str) -> Result<u16, std::io::Error> {
        //opens the file
        let a = File::open(path)?;
        //opens the file in a buffered reader
        let b = std::io::BufReader::new(a);
        //reads from the file into Value enum
        let c: Value = de::from_reader(b).expect("File not a valid .json");
        //gets the desired u16 as a u64, then converts to u16
        let d = c
            .get(get)
            .expect("read_from_file_u16 failed to get value")
            .as_u64()
            .expect("read_from_file_u16 failed to convert to u64") as u16;
        //returns as Result
        Ok(d)
    }
    fn read_from_file_vecstr(path: String, get: &str) -> Result<Vec<String>, std::io::Error> {
        //opens the json file
        let a = File::open(path)?;
        //makes the file a buffered reader
        let b = std::io::BufReader::new(a);
        //reads from file into Value enum
        let c: Value = serde_json::from_reader(b).expect("File not a valid .json");
        //gets the list from the overall value
        let d = c
            .get(get)
            .expect("read_from_file_vec failed to get value")
            .as_array()
            .expect("read_from_file_vec failed to convert to array");
        //vector for conversion
        let mut e = vec![];
        //take out each value in array to u16
        for i in d {
            e.push(
                i.as_str()
                    .expect("read_from_file_vec failed to move Vec<value> to Vec<u16>")
                    .to_string(),
            )
        }
        //returns as result
        Ok(e)
    }

    fn read_from_file_str(path: String, get: &str) -> Result<String, std::io::Error> {
        //opens the json file
        let a = File::open(path)?;
        //makes the file a buffered reader
        let b = std::io::BufReader::new(a);
        //reads from file into Value enum
        let c: Value = serde_json::from_reader(b).expect("File not a valid .json");
        //gets the list from the overall value
        let d = c
            .get(get)
            .expect("read_from_file_vec failed to get value")
            .as_str()
            .expect("read_from_file_vec failed to convert to array")
            .to_string();
        //returns as result
        Ok(d)
    }

    fn read_from_file_vecu16(path: String, get: &str) -> Result<Vec<u16>, std::io::Error> {
        //opens the json file
        let a = File::open(path)?;
        //makes the file a buffered reader
        let b = std::io::BufReader::new(a);
        //reads from file into Value enum
        let c: Value = de::from_reader(b).expect("File not a valid .json");
        //gets the list from the overall value
        let d = c
            .get(get)
            .expect("read_from_file_vec failed to get value")
            .as_array()
            .expect("read_from_file_vec failed to convert to array");
        //vector for conversion
        let mut e = vec![];
        //take out each value in array to u16
        for i in d {
            e.push(
                i.as_u64()
                    .expect("read_from_file_vec failed to move Vec<value> to Vec<u16>")
                    as u16,
            )
        }
        //returns as result
        Ok(e)
    }
    fn new_screen(place: String) -> Vec<u8> {
        //makes vec to be returned
        let mut data = vec![];
        //goes through the whole file by byte
        for pix in read(place)
            .expect("Unable to read from file")
            .chunks_exact(2)
        {
            //gives a vec<u8> of all "valid" bits for the fb without the added opacity bits
            data.push(
                u8::from_str_radix(
                    std::str::from_utf8(pix).expect("Unable to convert to utf-6"),
                    16,
                )
                .expect("Unable to convert to to hex value"),
            );
        }
        data.shrink_to_fit();
        //returns vector
        data
    }

    //not getting comments because it works
    fn draw(&self, pix: &mut [u8]) {
        for (it,pixel) in pix.chunks_exact_mut(4).enumerate() {
            pixel[0] = self.area[3 * self.scroll_dist as usize
                + (3 * self.screen_len * ((3 * it) / (3 * SCREEN_WIDTH) as usize))
                + ((it * 3) % (3 * SCREEN_WIDTH as usize))];
            pixel[1] = self.area[3 * self.scroll_dist as usize
                + (3 * self.screen_len * ((3 * it + 1) / (3 * SCREEN_WIDTH) as usize))
                + ((it * 3 + 1) % (3 * SCREEN_WIDTH as usize))];
            pixel[2] = self.area[3 * self.scroll_dist as usize
                + (3 * self.screen_len * ((3 * it + 2) / (3 * SCREEN_WIDTH) as usize))
                + ((it * 3 + 2) % (3 * SCREEN_WIDTH as usize))];
        }
        //find a way to not have to cast to u16 if i ever care
        for (it, pixel) in self.player.sprite[self.player.direction as usize][self.player.move_state as usize].chunks_exact(3).enumerate() {
            if pixel[0] as u16 + pixel[1] as u16 + pixel[2] as u16 != 0 {
                pix[(((self.player.y_pos as usize + (it / (CHAR_WIDTH - 1) as usize)) * SCREEN_WIDTH as usize) + (self.player.x_pos as usize + (it % (CHAR_WIDTH - 1) as usize))) * 4] = pixel[0];
                pix[(((self.player.y_pos as usize + (it / (CHAR_WIDTH - 1) as usize)) * SCREEN_WIDTH as usize) + (self.player.x_pos as usize + (it % (CHAR_WIDTH - 1) as usize))) * 4 + 1] = pixel[1];
                pix[(((self.player.y_pos as usize + (it / (CHAR_WIDTH - 1) as usize)) * SCREEN_WIDTH as usize) + (self.player.x_pos as usize + (it % (CHAR_WIDTH - 1) as usize))) * 4 + 2] = pixel[2];
            }
        }
        for a in &self.entities {
            for (it,pixel) in a.sprite[a.move_state as usize].chunks_exact(3).enumerate(){
                if pixel[0] as u16 + pixel[1] as u16 + pixel[2] as u16 != 0 {
                    pix[(((a.y_pos as usize + (it / (a.width) as usize)) * SCREEN_WIDTH as usize) + (a.x_pos as usize + (it % (a.width) as usize))) * 4] = pixel[0];
                    pix[(((a.y_pos as usize + (it / (a.width) as usize)) * SCREEN_WIDTH as usize) + (a.x_pos as usize + (it % (a.width) as usize))) * 4 + 1] = pixel[1];
                    pix[(((a.y_pos as usize + (it / (a.width) as usize)) * SCREEN_WIDTH as usize) + (a.x_pos as usize + (it % (a.width) as usize))) * 4 + 2] = pixel[2];
                }
            }
        }
    }

    fn new_dialog(&mut self, text: String) {
        let mut x:u16 = 30;
        let mut y:u16 = 360;
        let mut lett: Entity;
        for letter in text.chars() {
            x += 38;
            if x >=630 {
                x = 68;
                y += 40;
            }
            if letter == ' '{
                continue
            } else {
                println!("{}",format!("{}{}", "letras/", letter));
                lett = Entity::new(
                    &format!("{}{}{}{}.txt", "SpriteData/letras/", letter, "/", letter),
                    &format!("{}{}{}{}.txt", "SpriteData/letras/", letter, "/", letter),
                    &format!("{}{}{}{}.txt", "SpriteData/letras/", letter, "/", letter),
                    &format!("{}{}{}{}.txt", "SpriteData/letras/", letter, "/", letter),
                    &format!("{}{}{}{}.txt", "SpriteData/letras/", letter, "/", letter),
                    &format!("{}{}", "letras/", letter,),
                );
                lett.x_pos = x;
                lett.y_pos = y;
                self.entities.push(lett);
            }
        }
    }
}

struct Entity {
    //horizontal position from right of screen to left of player
    x_pos: u16,
    //vertical position from top of screen to top of player
    y_pos: u16,
    //1-4 animation frames
    sprite: [Vec<u8>; 5],
    //direction the player is facing
    // direction: u8,
    height: u8,
    width: u8,
    move_state: u8,
}
impl Entity {
    fn new(
        spr0: &str,
        spr1: &str,
        spr2: &str,
        spr3: &str,
        spr4: &str,
        idd: &str,
    ) -> Self {
        // giving all variables the default values
        Self {
            height: Entity::read_from_file_u8(
                format!("{}{}{}", "SpriteData/", idd, "/data.json"),
                "height",
            )
                .expect("failed to get height"),
            width: Entity::read_from_file_u8(
                format!("{}{}{}", "SpriteData/", idd, "/data.json"),
                "width",
            )
                .unwrap(),
            x_pos: 1,
            y_pos: 1,
            move_state: 0,
            sprite: [
                Entity::gen_sprite(spr0),
                Entity::gen_sprite(spr1),
                Entity::gen_sprite(spr2),
                Entity::gen_sprite(spr3),
                Entity::gen_sprite(spr4),
            ],
            // direction: 0,
        }
    }
    //used for turning single animation frame into readable bytes
    fn gen_sprite(spr: &str) -> Vec<u8> {
        //vector containing the sprite data to be returned
        let mut data = vec![];
        //loop through the file in by byte
        for pix in read(spr).expect("Failed to read from file").chunks_exact(2) {
            //append each value taken from hex byte to single u8 value
            data.push(
                u8::from_str_radix(
                    std::str::from_utf8(pix).expect("Failed to convert to utf8"),
                    16,
                )
                    .expect("Failed to convert to hex value"),
            );
        }
        //return the vector with the info
        data
    }
    fn read_from_file_u8(path: String, get: &str) -> Result<u8, std::io::Error> {
        //opens the file
        let a = File::open(path)?;
        //opens the file in a buffered reader
        let b = std::io::BufReader::new(a);
        //reads from the file into Value enum
        let c: Value = de::from_reader(b).expect("File not a valid .json");
        //gets the desired u16 as a u64, then converts to u16
        let d = c
            .get(get)
            .expect("read_from_file_u16 failed to get value")
            .as_u64()
            .expect("read_from_file_u16 failed to convert to u64") as u8;
        //returns as Result
        Ok(d)
    }
}