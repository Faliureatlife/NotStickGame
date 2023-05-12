//extra functions for idiomatic code or wtv
//todo: make moving work
//todo: replace serde with miniserde (maybe)
//todo: multithreading
//todo: pause when move off tab
use rand::prelude::*;
use pixels::{
    wgpu::{PowerPreference, RequestAdapterOptions,PresentMode},
    PixelsBuilder,
};
//Dont just import all of pixels at some point
use serde_json::{de, value::Value};
use std::{
    env,
    fs::*,
    // time::SystemTime,
    // mem,
    // io::Write,
    // time::Duration,
    // thread::sleep,
    // u8,
    // error::Error;
};
use winit::event::VirtualKeyCode;
use winit::{
    dpi::PhysicalSize,
    event::*,
    event_loop::*,
    window::Window,
    //dpi::PhysicalSize,
};
use winit_input_helper::WinitInputHelper;
// use pixels::wgpu::Color;
// use rayon::prelude::*;

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
    let mut track: u8 = 0;

    // Battle Variables
    let mut battle:bool = false;
    let mut fight:bool = false;
    let mut time_count: u16 = 0;
    let mut player_health: u8 = 4;

    // Run Variables
    let mut run:bool = false;
    let mut run_did:bool = false;
    let mut rng = rand::thread_rng();
    let mut run_good: u8 = 0;
    let mut try_run:bool = false;

    // Fight Variables
    
    let task = vec!["Expand", "Expand", "Expand", "Expand", "Expand", "Expand", "Expand", "Expand", 
                    "Simplify", "Simplify", "Simplify", "Simplify"];
    let problems = vec!["x@+4x+4", "6x@+5x+1", "12x@+4x-1", "4x@+9x-9", "x@-49", "x@-25", "x@+2x-80", "9x@+6x-80", 
                    "(4x+7)(2x+3)", "(-5y-5)(4y-2)", "(3l-7)(3l-5)", "(j+7)(j-7)"];
    let options = vec![vec!["(x+3)(x-5)".to_string(), "(x+2)(x-2)".to_string(), "(x+2)(x+2)".to_string(), "(x+2)(x+1)".to_string()], 
                       vec!["(3x+1)(2x+1)".to_string(), "(3x-1)(2x-1)".to_string(), "(3x+1)(2x-1)".to_string(), "(3x-1)(2x+1)".to_string()],
                       vec!["(6x-1)(2x-1)".to_string(), "(6x+1)(2x-1)".to_string(), "(6x+1)(2x+1)".to_string(), "(6x-1)(2x+1)".to_string()],
                       vec!["(4x-3)(x-3)".to_string(), "(4x-3)(x+3)".to_string(), "(4x+3)(x+3)".to_string(), "(4x+3)(x-3)".to_string()],
                       vec!["(x+7)(x+7)".to_string(), "(x-7)(x+9)".to_string(), "(x-7)(x-7)".to_string(), "(x-7)(x+7)".to_string()],
                       vec!["(x-5)(x-5)".to_string(), "(x-5)(x+7)".to_string(), "(x-5)(x+5)".to_string(), "(x+5)(x+5)".to_string()],
                       vec!["(x-10)(x-8)".to_string(), "(x+10)(x+8)".to_string(), "(x-10)(x+8)".to_string(), "(x+10)(x-8)".to_string()],
                       vec!["(3x-8)(3x+10)".to_string(), "(3x+8)(3x-10)".to_string(), "(-3x-8)(-3x-10)".to_string(), "(3x-3)(3x+10)".to_string()],
                       vec!["8x@+2x-21".to_string(), "8x@-26x+21".to_string(), "8x@-2x-21".to_string(), "8x@+26x+21".to_string()],
                       vec!["-20y@-30y-10".to_string(), "-20y@+30y-10".to_string(), "-20y@-10y+10".to_string(), "-20y@+10y+10".to_string()],
                       vec!["9l@-36l-35".to_string(), "9l@+36l+35".to_string(), "9l@-36l+35".to_string(), "9l@+6l+35".to_string()],
                       vec!["j@-14j-49".to_string(), "j@+14j-49".to_string(), "j@-49".to_string(), "j@+14j-49".to_string()]];
    let answer = vec!["(x+2)(x+2)", "(3x+1)(2x+1)", "(6x-1)(2x+1)", "(4x-3)(g+3)", "(x-7)(x+7)", "(x-5)(x+5)", "(x+10)(x-8)", "(3x-8)(3x+10)", 
                      "8x@+26x+21", "-20y@-10y+10", "9l@-36l+35", "j@-49"];
    let mut problem_choose: usize = 0;
    let mut problem_generate: bool = false;
    let mut submit: bool = false;
    let mut fight_tracker: usize = 0;
    let mut total_correct: u8 = 0;
    let mut battle_won: bool = false;
    //todo: multithreading to have game thinking and rendering at same time
    //loop that runs program
    event_loop.run(move |event, _, control_flow| {
        //When it wants to redraw do this
        if let Event::RedrawRequested(_) = event {
            //framebuffer that we shall mut
            screen.draw(pixels.get_frame());
            //screen.draw_dialog(pix.get_frane());
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
        if input.update(&event) && !paused && !battle{
            //make into a match statement at some point maybe
            //close on pressing esc
            if input.key_pressed(VirtualKeyCode::U) {
                println!("{:?}", screen.interact_pos);
            }
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            if input.key_pressed(VirtualKeyCode::Tab) {
                track = 0;
                last_scr = screen.scr.clone();
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

            if screen.player.move_state != 0 {
                let encounter:u16 = rng.gen_range(0..500);
                if encounter <= 2 {
                    track = 0;
                    last_scr = screen.scr.clone();
                    x_save = screen.player.x_pos;
                    y_save = screen.player.y_pos;
                    match last_scr.as_str() {
                        "houses" => {
                            screen = Screen::new("BattleScene/Full-Health/Select/Fight/houses");
                        }
                        "stoor" => {
                            screen = Screen::new("BattleScene/Full-Health/Select/Fight/stoor");
                        }
                        "school-cafeteria" => {}
                        "school-math" => {}
                        "school-english" => {}
                        "pond" => {
                            screen = Screen::new("BattleScene/Full-Health/Select/Fight/pond");
                        }
                        "library-tables" => {
                            screen = Screen::new("BattleScene/Full-Health/Select/Fight/library");
                        }
                        "house-living" => {}
                        "school-hall" => {}
                        "school" => {
                            screen = Screen::new("BattleScene/Full-Health/Select/Fight/school");
                        }
                        "lhouses" => {}
                        _ => {}
                    }
                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                    battle = !battle;
                    screen.player.move_state = 0;
                    left = false;
                    right = true;
                    up = false;
                    down = false;
                }
            }
            //after updates happen redraw the screen
            window.request_redraw();
        };
        if paused && input.update(&event) {
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
            // Selects choice and runs appropriate code
            if input.key_pressed(VirtualKeyCode::Return) {
                match track % 5 {
                    4 => {
                        screen = Screen::new(&last_scr);
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

        // Use match statements to determine what scenes to load
        // match last_scr {
        //      "stoor" =>{
        //          screen = Screen::new("StoorBattleScene/Full-Health/Select/Fight");
        //          screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
        //      }...
        // }
        if battle && input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) {
                *control_flow = ControlFlow::Exit;
                return;
            }
            
            if !fight && !run {
                match track {
                    0 => {
                        // Fight select
                        match player_health {
                            4 => {
                                match last_scr.as_str() {
                                    "houses" => {
                                        screen = Screen::new("BattleScene/Full-Health/Select/Fight/houses");
                                    }
                                    "stoor" => {
                                        screen = Screen::new("BattleScene/Full-Health/Select/Fight/stoor");
                                    }
                                    "school-cafeteria" => {
                                        screen = Screen::new("BattleScene/Full-Health/Select/Fight/school-cafeteria");
                                    }
                                    "school-math" => {
                                        screen = Screen::new("BattleScene/Full-Health/Select/Fight/school-math");
                                    }
                                    "school-english" => {
                                        screen = Screen::new("BattleScene/Full-Health/Select/Fight/school-english");
                                    }
                                    "pond" => {
                                        screen = Screen::new("BattleScene/Full-Health/Select/Fight/pond");
                                    }
                                    "library-tables" => {
                                        screen = Screen::new("BattleScene/Full-Health/Select/Fight/library");
                                    }
                                    "house-living" => {
                                        screen = Screen::new("BattleScene/Full-Health/Select/Fight/house-living");
                                    }
                                    "school-hall" => {
                                        screen = Screen::new("BattleScene/Full-Health/Select/Fight/school-hall");
                                    }
                                    "school" => {
                                        screen = Screen::new("BattleScene/Full-Health/Select/Fight/school");
                                    }
                                    "lhouses" => {
                                        screen = Screen::new("BattleScene/Full-Health/Select/Fight/lhouses");
                                    }
                                    _ => {}
                                }
                            }
                            3 => {
                                match last_scr.as_str() {
                                    "houses" => {
                                        screen = Screen::new("BattleScene/75-Health/Select/Fight/houses");
                                    }
                                    "stoor" => {
                                        screen = Screen::new("BattleScene/75-Health/Select/Fight/stoor");
                                    }
                                    "school-cafeteria" => {
                                        screen = Screen::new("BattleScene/75-Health/Select/Fight/school-cafeteria");
                                    }
                                    "school-math" => {
                                        screen = Screen::new("BattleScene/75-Health/Select/Fight/school-math");
                                    }
                                    "school-english" => {
                                        screen = Screen::new("BattleScene/75-Health/Select/Fight/school-english");
                                    }
                                    "pond" => {
                                        screen = Screen::new("BattleScene/75-Health/Select/Fight/pond");
                                    }
                                    "library-tables" => {
                                        screen = Screen::new("BattleScene/75-Health/Select/Fight/library-tables");
                                    }
                                    "house-living" => {
                                        screen = Screen::new("BattleScene/75-Health/Select/Fight/house-living");
                                    }
                                    "school-hall" => {
                                        screen = Screen::new("BattleScene/75-Health/Select/Fight/school-hall");
                                    }
                                    "school" => {
                                        screen = Screen::new("BattleScene/75-Health/Select/Fight/school");
                                    }
                                    "lhouses" => {
                                        screen = Screen::new("BattleScene/75-Health/Select/Fight/lhouses");
                                    }
                                    _ => {}
                                }
                            }
                            2 => {
                                match last_scr.as_str() {
                                    "houses" => {
                                        screen = Screen::new("BattleScene/50-Health/Select/Fight/houses");
                                    }
                                    "stoor" => {
                                        screen = Screen::new("BattleScene/50-Health/Select/Fight/stoor");
                                    }
                                    "school-cafeteria" => {
                                        screen = Screen::new("BattleScene/50-Health/Select/Fight/school-cafeteria");
                                    }
                                    "school-math" => {
                                        screen = Screen::new("BattleScene/50-Health/Select/Fight/school-math");
                                    }
                                    "school-english" => {
                                        screen = Screen::new("BattleScene/50-Health/Select/Fight/school-english");
                                    }
                                    "pond" => {
                                        screen = Screen::new("BattleScene/50-Health/Select/Fight/pond");
                                    }
                                    "library-tables" => {
                                        screen = Screen::new("BattleScene/50-Health/Select/Fight/library-tables");
                                    }
                                    "house-living" => {
                                        screen = Screen::new("BattleScene/50-Health/Select/Fight/house-living");
                                    }
                                    "school-hall" => {
                                        screen = Screen::new("BattleScene/50-Health/Select/Fight/school-hall");
                                    }
                                    "school" => {
                                        screen = Screen::new("BattleScene/50-Health/Select/Fight/school");
                                    }
                                    "lhouses" => {
                                        screen = Screen::new("BattleScene/50-Health/Select/Fight/lhouses");
                                    }
                                    _ => {}
                                }
                            }
                            1 => {
                                match last_scr.as_str() {
                                    "houses" => {
                                        screen = Screen::new("BattleScene/25-Health/Select/Fight/houses");
                                    }
                                    "stoor" => {
                                        screen = Screen::new("BattleScene/25-Health/Select/Fight/stoor");
                                    }
                                    "school-cafeteria" => {
                                        screen = Screen::new("BattleScene/25-Health/Select/Fight/school-cafeteria");
                                    }
                                    "school-math" => {
                                        screen = Screen::new("BattleScene/25-Health/Select/Fight/school-math");
                                    }
                                    "school-english" => {
                                        screen = Screen::new("BattleScene/25-Health/Select/Fight/school-english");
                                    }
                                    "pond" => {
                                        screen = Screen::new("BattleScene/25-Health/Select/Fight/pond");
                                    }
                                    "library-tables" => {
                                        screen = Screen::new("BattleScene/25-Health/Select/Fight/library-tables");
                                    }
                                    "house-living" => {
                                        screen = Screen::new("BattleScene/25-Health/Select/Fight/house-living");
                                    }
                                    "school-hall" => {
                                        screen = Screen::new("BattleScene/25-Health/Select/Fight/school-hall");
                                    }
                                    "school" => {
                                        screen = Screen::new("BattleScene/25-Health/Select/Fight/school");
                                    }
                                    "lhouses" => {
                                        screen = Screen::new("BattleScene/25-Health/Select/Fight/lhouses");
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    }
                    1 => {
                        match player_health {
                            4 => {
                                match last_scr.as_str() {
                                    "houses" => {
                                        screen = Screen::new("BattleScene/Full-Health/Select/Run/houses");
                                    }
                                    "stoor" => {
                                        screen = Screen::new("BattleScene/Full-Health/Select/Run/stoor");
                                    }
                                    "school-cafeteria" => {
                                        screen = Screen::new("BattleScene/Full-Health/Select/Run/school-cafeteria");
                                    }
                                    "school-math" => {
                                        screen = Screen::new("BattleScene/Full-Health/Select/Run/school-math");
                                    }
                                    "school-english" => {
                                        screen = Screen::new("BattleScene/Full-Health/Select/Run/school-english");
                                    }
                                    "pond" => {
                                        screen = Screen::new("BattleScene/Full-Health/Select/Run/pond");
                                    }
                                    "library-tables" => {
                                        screen = Screen::new("BattleScene/Full-Health/Select/Run/library");
                                    }
                                    "house-living" => {
                                        screen = Screen::new("BattleScene/Full-Health/Select/Run/house-living");
                                    }
                                    "school-hall" => {
                                        screen = Screen::new("BattleScene/Full-Health/Select/Run/school-hall");
                                    }
                                    "school" => {
                                        screen = Screen::new("BattleScene/Full-Health/Select/Run/school");
                                    }
                                    "lhouses" => {
                                        screen = Screen::new("BattleScene/Full-Health/Select/Run/lhouses");
                                    }
                                    _ => {}
                                }
                            }
                            3 => {
                                match last_scr.as_str() {
                                    "houses" => {
                                        screen = Screen::new("BattleScene/75-Health/Select/Run/houses");
                                    }
                                    "stoor" => {
                                        screen = Screen::new("BattleScene/75-Health/Select/Run/stoor");
                                    }
                                    "school-cafeteria" => {
                                        screen = Screen::new("BattleScene/75-Health/Select/Run/school-cafeteria");
                                    }
                                    "school-math" => {
                                        screen = Screen::new("BattleScene/75-Health/Select/Run/school-math");
                                    }
                                    "school-english" => {
                                        screen = Screen::new("BattleScene/75-Health/Select/Run/school-english");
                                    }
                                    "pond" => {
                                        screen = Screen::new("BattleScene/75-Health/Select/Run/pond");
                                    }
                                    "library-tables" => {
                                        screen = Screen::new("BattleScene/75-Health/Select/Run/library-tables");
                                    }
                                    "house-living" => {
                                        screen = Screen::new("BattleScene/75-Health/Select/Run/house-living");
                                    }
                                    "school-hall" => {
                                        screen = Screen::new("BattleScene/75-Health/Select/Run/school-hall");
                                    }
                                    "school" => {
                                        screen = Screen::new("BattleScene/75-Health/Select/Run/school");
                                    }
                                    "lhouses" => {
                                        screen = Screen::new("BattleScene/75-Health/Select/Run/lhouses");
                                    }
                                    _ => {}
                                }
                            }
                            2 => {
                                match last_scr.as_str() {
                                    "houses" => {
                                        screen = Screen::new("BattleScene/50-Health/Select/Run/houses");
                                    }
                                    "stoor" => {
                                        screen = Screen::new("BattleScene/50-Health/Select/Run/stoor");
                                    }
                                    "school-cafeteria" => {
                                        screen = Screen::new("BattleScene/50-Health/Select/Run/school-cafeteria");
                                    }
                                    "school-math" => {
                                        screen = Screen::new("BattleScene/50-Health/Select/Run/school-math");
                                    }
                                    "school-english" => {
                                        screen = Screen::new("BattleScene/50-Health/Select/Run/school-english");
                                    }
                                    "pond" => {
                                        screen = Screen::new("BattleScene/50-Health/Select/Run/pond");
                                    }
                                    "library-tables" => {
                                        screen = Screen::new("BattleScene/50-Health/Select/Run/library-tables");
                                    }
                                    "house-living" => {
                                        screen = Screen::new("BattleScene/50-Health/Select/Run/house-living");
                                    }
                                    "school-hall" => {
                                        screen = Screen::new("BattleScene/50-Health/Select/Run/school-hall");
                                    }
                                    "school" => {
                                        screen = Screen::new("BattleScene/50-Health/Select/Run/school");
                                    }
                                    "lhouses" => {
                                        screen = Screen::new("BattleScene/50-Health/Select/Run/lhouses");
                                    }
                                    _ => {}
                                }
                            }
                            1 => {
                                match last_scr.as_str() {
                                    "houses" => {
                                        screen = Screen::new("BattleScene/25-Health/Select/Run/houses");
                                    }
                                    "stoor" => {
                                        screen = Screen::new("BattleScene/25-Health/Select/Run/stoor");
                                    }
                                    "school-cafeteria" => {
                                        screen = Screen::new("BattleScene/25-Health/Select/Run/school-cafeteria");
                                    }
                                    "school-math" => {
                                        screen = Screen::new("BattleScene/25-Health/Select/Run/school-math");
                                    }
                                    "school-english" => {
                                        screen = Screen::new("BattleScene/25-Health/Select/Run/school-english");
                                    }
                                    "pond" => {
                                        screen = Screen::new("BattleScene/25-Health/Select/Run/pond");
                                    }
                                    "library-tables" => {
                                        screen = Screen::new("BattleScene/25-Health/Select/Run/library-tables");
                                    }
                                    "house-living" => {
                                        screen = Screen::new("BattleScene/25-Health/Select/Run/house-living");
                                    }
                                    "school-hall" => {
                                        screen = Screen::new("BattleScene/25-Health/Select/Run/school-hall");
                                    }
                                    "school" => {
                                        screen = Screen::new("BattleScene/25-Health/Select/Run/school");
                                    }
                                    "lhouses" => {
                                        screen = Screen::new("BattleScene/25-Health/Select/Run/lhouses");
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
                screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                // Moves option selected to previous option
                if input.key_pressed(VirtualKeyCode::A) || input.key_pressed(VirtualKeyCode::Left) {
                    if track == 0 {
                        track = 1;
                    } else {
                        track = 0;
                    }
                }
                // Moves option selected to following option
                if input.key_pressed(VirtualKeyCode::D) || input.key_pressed(VirtualKeyCode::Right) {
                    if track == 1 {
                        track = 0;
                    } else {
                        track = 1;
                    }
                }
                if input.key_pressed(VirtualKeyCode::Return) {
                    match track{
                        0 => {
                            fight = true;
                            run = false;
                            track = 0;
                        }   
                        1 => {
                            run = true;
                            fight = false;
                            track = 0;
                        }
                        _ => {}
                    }
                }
            }

            if fight {
                if !problem_generate {
                    problem_choose = rng.gen_range(0..12);
                    problem_generate = true;
                }

                if !submit {
                    match last_scr.as_str(){
                        "houses" => {
                            match fight_tracker {
                                0 => {
                                    screen = Screen::new("BattleScene/General-Use/houses/fight/fight1");
                                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                                }
                                1 => {
                                    screen = Screen::new("BattleScene/General-Use/houses/fight/fight2");
                                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                                }
                                2 => {
                                    screen = Screen::new("BattleScene/General-Use/houses/fight/fight3");
                                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                                }
                                3 => {
                                    screen = Screen::new("BattleScene/General-Use/houses/fight/fight4");
                                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                                }
                                _ => {}
                            }
                        }
                        "stoor" => {
                            match fight_tracker {
                                0 => {
                                    screen = Screen::new("BattleScene/General-Use/stoor/fight/fight1");
                                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                                }
                                1 => {
                                    screen = Screen::new("BattleScene/General-Use/stoor/fight/fight2");
                                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                                }
                                2 => {
                                    screen = Screen::new("BattleScene/General-Use/stoor/fight/fight3");
                                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                                }
                                3 => {
                                    screen = Screen::new("BattleScene/General-Use/stoor/fight/fight4");
                                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                                }
                                _ => {}
                            }
                        }
                        "school-cafeteria" => {
                            match fight_tracker {
                                0 => {
                                    screen = Screen::new("BattleScene/General-Use/school-cafeteria/fight/fight1");
                                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                                }
                                1 => {
                                    screen = Screen::new("BattleScene/General-Use/school-cafeteria/fight/fight2");
                                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                                }
                                2 => {
                                    screen = Screen::new("BattleScene/General-Use/school-cafeteria/fight/fight3");
                                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                                }
                                3 => {
                                    screen = Screen::new("BattleScene/General-Use/school-cafeteria/fight/fight4");
                                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                                }
                                _ => {}
                            }
                        }
                        "school-math" => {
                            match fight_tracker {
                                0 => {
                                    screen = Screen::new("BattleScene/General-Use/school-math/fight/fight1");
                                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                                }
                                1 => {
                                    screen = Screen::new("BattleScene/General-Use/school-math/fight/fight2");
                                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                                }
                                2 => {
                                    screen = Screen::new("BattleScene/General-Use/school-math/fight/fight3");
                                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                                }
                                3 => {
                                    screen = Screen::new("BattleScene/General-Use/school-math/fight/fight4");
                                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                                }
                                _ => {}
                            }
                        }
                        "school-english" => {
                            match fight_tracker {
                                0 => {
                                    screen = Screen::new("BattleScene/General-Use/school-english/fight/fight1");
                                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                                }
                                1 => {
                                    screen = Screen::new("BattleScene/General-Use/school-english/fight/fight2");
                                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                                }
                                2 => {
                                    screen = Screen::new("BattleScene/General-Use/school-english/fight/fight3");
                                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                                }
                                3 => {
                                    screen = Screen::new("BattleScene/General-Use/school-english/fight/fight4");
                                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                                }
                                _ => {}
                            }
                        }
                        "pond" => {
                            match fight_tracker {
                                0 => {
                                    screen = Screen::new("BattleScene/General-Use/pond/fight/fight1");
                                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                                }
                                1 => {
                                    screen = Screen::new("BattleScene/General-Use/pond/fight/fight2");
                                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                                }
                                2 => {
                                    screen = Screen::new("BattleScene/General-Use/pond/fight/fight3");
                                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                                }
                                3 => {
                                    screen = Screen::new("BattleScene/General-Use/pond/fight/fight4");
                                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                                }
                                _ => {}
                            }
                        }
                        "library-tables" => {
                            match fight_tracker {
                                0 => {
                                    screen = Screen::new("BattleScene/General-Use/library/fight/fight1");
                                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                                }
                                1 => {
                                    screen = Screen::new("BattleScene/General-Use/library/fight/fight2");
                                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                                }
                                2 => {
                                    screen = Screen::new("BattleScene/General-Use/library/fight/fight3");
                                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                                }
                                3 => {
                                    screen = Screen::new("BattleScene/General-Use/library/fight/fight4");
                                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                                }
                                _ => {}
                            }
                        }
                        "house-living" => {
                            match fight_tracker {
                                0 => {
                                    screen = Screen::new("BattleScene/General-Use/house-living/fight/fight1");
                                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                                }
                                1 => {
                                    screen = Screen::new("BattleScene/General-Use/house-living/fight/fight2");
                                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                                }
                                2 => {
                                    screen = Screen::new("BattleScene/General-Use/house-living/fight/fight3");
                                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                                }
                                3 => {
                                    screen = Screen::new("BattleScene/General-Use/house-living/fight/fight4");
                                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                                }
                                _ => {}
                            }
                        }
                        "school-hall" => {
                            match fight_tracker {
                                0 => {
                                    screen = Screen::new("BattleScene/General-Use/school-hall/fight/fight1");
                                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                                }
                                1 => {
                                    screen = Screen::new("BattleScene/General-Use/school-hall/fight/fight2");
                                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                                }
                                2 => {
                                    screen = Screen::new("BattleScene/General-Use/school-hall/fight/fight3");
                                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                                }
                                3 => {
                                    screen = Screen::new("BattleScene/General-Use/school-hall/fight/fight4");
                                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                                }
                                _ => {}
                            }
                        }
                        "school" => {
                            match fight_tracker {
                                0 => {
                                    screen = Screen::new("BattleScene/General-Use/school/fight/fight1");
                                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                                }
                                1 => {
                                    screen = Screen::new("BattleScene/General-Use/school/fight/fight2");
                                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                                }
                                2 => {
                                    screen = Screen::new("BattleScene/General-Use/school/fight/fight3");
                                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                                }
                                3 => {
                                    screen = Screen::new("BattleScene/General-Use/school/fight/fight4");
                                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                                }
                                _ => {}
                            }
                        }
                        "lhouses" => {
                            match fight_tracker {
                                0 => {
                                    screen = Screen::new("BattleScene/General-Use/lhouses/fight/fight1");
                                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                                }
                                1 => {
                                    screen = Screen::new("BattleScene/General-Use/lhouses/fight/fight2");
                                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                                }
                                2 => {
                                    screen = Screen::new("BattleScene/General-Use/lhouses/fight/fight3");
                                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                                }
                                3 => {
                                    screen = Screen::new("BattleScene/General-Use/lhouses/fight/fight4");
                                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                    screen.fight_write(task[problem_choose].to_string() + ";" + problems[problem_choose], 75, 455);
                    screen.fight_write(options[problem_choose][0].to_string(), 150, 66);
                    screen.fight_write(options[problem_choose][1].to_string(), 150, 150);
                    screen.fight_write(options[problem_choose][2].to_string(), 150, 234);
                    screen.fight_write(options[problem_choose][3].to_string(), 150, 318);
                    if input.key_pressed(VirtualKeyCode::W) {
                        if fight_tracker == 0 {
                            fight_tracker = 3;
                        } else {
                            fight_tracker = fight_tracker - 1;
                        }
                        println!("{}", fight_tracker);
                    }
                    if input.key_pressed(VirtualKeyCode::S) {
                        if fight_tracker == 3 {
                            fight_tracker = 0;
                        } else {
                            fight_tracker = fight_tracker + 1;
                        }
                        println!("{}", fight_tracker);
                    }
                    if input.key_pressed(VirtualKeyCode::Return) && time_count > 5{
                        submit = true;
                        time_count = 0;
                        println!("{}", options[problem_choose][fight_tracker].to_string());
                        println!("{}", answer[problem_choose].to_string());
                    } else {
                        time_count = time_count + 1;
                    }
                }
                if submit {
                    if options[problem_choose][fight_tracker] != answer[problem_choose] {
                        match last_scr.as_str() {
                            "houses" => {
                                screen = Screen::new("BattleScene/General-Use/houses/end");
                            }
                            "stoor" => {
                                screen = Screen::new("BattleScene/General-Use/stoor/end");
                            }
                            "school-cafeteria" => {
                                screen = Screen::new("BattleScene/General-Use/school-cafeteria/end");
                            }
                            "school-math" => {
                                screen = Screen::new("BattleScene/General-Use/school-math/end");
                            }
                            "school-english" => {
                                screen = Screen::new("BattleScene/General-Use/school-english/end");
                            }
                            "pond" => {
                                screen = Screen::new("BattleScene/General-Use/pond/end");
                            }
                            "library-tables" => {
                                screen = Screen::new("BattleScene/General-Use/library/end");
                            }
                            "house-living" => {
                                screen = Screen::new("BattleScene/General-Use/house-living/end");
                            }
                            "school-hall" => {
                                screen = Screen::new("BattleScene/General-Use/school-hall/end");
                            }
                            "school" => {
                                screen = Screen::new("BattleScene/General-Use/school/end");
                            }
                            "lhouses" => {
                                screen = Screen::new("BattleScene/General-Use/lhouses/end");
                            }
                            _ => {}
                        }
                        screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                        if time_count <= 60 {
                            screen.fight_write("Your answer is".to_string(), 75, 455);
                            screen.fight_write("incorrect".to_string(), 75, 490);
                            time_count = time_count + 1;
                        } else if time_count <= 110{
                            screen.fight_write("Nav is hit".to_string(), 75, 455);
                            time_count = time_count + 1;
                        } else {
                            player_health = player_health - 1;
                            fight = false;
                            time_count = 0;
                            submit = false;
                            problem_generate = false;
                            fight_tracker = 0; 
                        }
                    } else {
                        match last_scr.as_str() {
                            "houses" => {
                                screen = Screen::new("BattleScene/General-Use/houses/end");
                            }
                            "stoor" => {
                                screen = Screen::new("BattleScene/General-Use/stoor/end");
                            }
                            "school-cafeteria" => {
                                screen = Screen::new("BattleScene/General-Use/school-cafeteria/end");
                            }
                            "school-math" => {
                                screen = Screen::new("BattleScene/General-Use/school-math/end");
                            }
                            "school-english" => {
                                screen = Screen::new("BattleScene/General-Use/school-english/end");
                            }
                            "pond" => {
                                screen = Screen::new("BattleScene/General-Use/pond/end");
                            }
                            "library-tables" => {
                                screen = Screen::new("BattleScene/General-Use/library/end");
                            }
                            "house-living" => {
                                screen = Screen::new("BattleScene/General-Use/house-living/end");
                            }
                            "school-hall" => {
                                screen = Screen::new("BattleScene/General-Use/school-hall/end");
                            }
                            "school" => {
                                screen = Screen::new("BattleScene/General-Use/school/end");
                            }
                            "lhouses" => {
                                screen = Screen::new("BattleScene/General-Use/lhouses/end");
                            }
                            _ => {}
                        }
                        screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                        if time_count <= 60 {
                            screen.fight_write("You are enemy".to_string(), 75, 450);
                            time_count = time_count + 1;
                        } else if time_count <= 120 {
                            screen.fight_write("You attack the".to_string(),75, 450);
                            screen.fight_write("enemy".to_string(), 75, 490);
                            time_count = time_count + 1;
                        } else if time_count <= 175 {
                            match total_correct {
                                0 => {
                                    screen.fight_write("The enemy has been".to_string(),75, 450);
                                    screen.fight_write("injured".to_string(), 75, 490);
                                }
                                1 => {
                                    screen.fight_write("The enemy is nearly".to_string(),75, 450);
                                    screen.fight_write("defeated".to_string(), 75, 490);
                                }
                                2 => {
                                    screen.fight_write("The enemy faints".to_string(),75, 450);
                                }
                                _ => {}
                            }
                            time_count = time_count + 1;
                        } else {
                            total_correct = total_correct + 1;
                            if total_correct == 3{
                                battle_won = true;
                            }
                            time_count = 0;
                            fight = false;
                            submit = false;
                            problem_generate = false;
                            fight_tracker = 0;
                        }
                    }
                }
            }

            if run {
                if !try_run {
                    run_good = rng.gen_range(0..100);
                    if run_good > 50 {
                        run_did = false;
                    } else {
                        run_did = true;
                    }
                    try_run = true;
                }

                if !run_did {
                    if time_count < 65 {
                        match last_scr.as_str() {
                            "houses" => {
                                screen = Screen::new("BattleScene/General-Use/houses/end");
                            }
                            "stoor" => {
                                screen = Screen::new("BattleScene/General-Use/stoor/end");
                            }
                            "school-cafeteria" => {
                                screen = Screen::new("BattleScene/General-Use/school-cafeteria/end");
                            }
                            "school-math" => {
                                screen = Screen::new("BattleScene/General-Use/school-math/end");
                            }
                            "school-english" => {
                                screen = Screen::new("BattleScene/General-Use/school-english/end");
                            }
                            "pond" => {
                                screen = Screen::new("BattleScene/General-Use/pond/end");
                            }
                            "library-tables" => {
                                screen = Screen::new("BattleScene/General-Use/library/end");
                            }
                            "house-living" => {
                                screen = Screen::new("BattleScene/General-Use/house-living/end");
                            }
                            "school-hall" => {
                                screen = Screen::new("BattleScene/General-Use/school-hall/end");
                            }
                            "school" => {
                                screen = Screen::new("BattleScene/General-Use/school/end");
                            }
                            "lhouses" => {
                                screen = Screen::new("BattleScene/General-Use/lhouses/end");
                            }
                            _ => {}
                        }
                        screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                        screen.fight_write("Nav cant run".to_string(), 75, 455);
                        time_count = time_count + 1;
                    }
                    if time_count < 130 && time_count >= 65 {
                        match last_scr.as_str() {
                            "houses" => {
                                screen = Screen::new("BattleScene/General-Use/houses/end");
                            }
                            "stoor" => {
                                screen = Screen::new("BattleScene/General-Use/stoor/end");
                            }
                            "school-cafeteria" => {
                                screen = Screen::new("BattleScene/General-Use/school-cafeteria/end");
                            }
                            "school-math" => {
                                screen = Screen::new("BattleScene/General-Use/school-math/end");
                            }
                            "school-english" => {
                                screen = Screen::new("BattleScene/General-Use/school-english/end");
                            }
                            "pond" => {
                                screen = Screen::new("BattleScene/General-Use/pond/end");
                            }
                            "library-tables" => {
                                screen = Screen::new("BattleScene/General-Use/library/end");
                            }
                            "house-living" => {
                                screen = Screen::new("BattleScene/General-Use/house-living/end");
                            }
                            "school-hall" => {
                                screen = Screen::new("BattleScene/General-Use/school-hall/end");
                            }
                            "school" => {
                                screen = Screen::new("BattleScene/General-Use/school/end");
                            }
                            "lhouses" => {
                                screen = Screen::new("BattleScene/General-Use/lhouses/end");
                            }
                            _ => {}
                        }
                        screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                        screen.fight_write("Nav is hit".to_string(), 75, 455);
                        time_count = time_count + 1;
                    }
                    if time_count >= 130 {
                        run = false;
                        time_count = 0;
                        player_health = player_health - 1;
                        track = 0;
                    }
                }

                if run_did {
                    
                    if time_count < 65 {
                        match last_scr.as_str() {
                            "houses" => {
                                screen = Screen::new("BattleScene/General-Use/houses/end");
                            }
                            "stoor" => {
                                screen = Screen::new("BattleScene/General-Use/stoor/end");
                            }
                            "school-cafeteria" => {
                                screen = Screen::new("BattleScene/General-Use/school-cafeteria/end");
                            }
                            "school-math" => {
                                screen = Screen::new("BattleScene/General-Use/school-math/end");
                            }
                            "school-english" => {
                                screen = Screen::new("BattleScene/General-Use/school-english/end");
                            }
                            "pond" => {
                                screen = Screen::new("BattleScene/General-Use/pond/end");
                            }
                            "library-tables" => {
                                screen = Screen::new("BattleScene/General-Use/library/end");
                            }
                            "house-living" => {
                                screen = Screen::new("BattleScene/General-Use/house-living/end");
                            }
                            "school-hall" => {
                                screen = Screen::new("BattleScene/General-Use/school-hall/end");
                            }
                            "school" => {
                                screen = Screen::new("BattleScene/General-Use/school/end");
                            }
                            "lhouses" => {
                                screen = Screen::new("BattleScene/General-Use/lhouses/end");
                            }
                            _ => {}
                        }
                        screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                        screen.fight_write("Nav runs away".to_string(), 75, 455);
                        time_count = time_count + 1;
                    }

                    if time_count >= 65 {
                        player_health = 4;
                        run = false;
                        try_run = false;
                        run_did = false;
                        fight = false;
                        time_count = 0;
                        battle = false;
                        up = false;
                        left = false;
                        right = false;
                        down = false;
                        screen = Screen::new(&last_scr);
                        screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                        screen.player.x_pos = x_save;
                        screen.player.y_pos = y_save;
                        track = 0;
                        total_correct = 0;
                    }
                }
            }

            if player_health == 0 {
                match last_scr.as_str() {
                    "houses" => {
                        screen = Screen::new("BattleScene/General-Use/houses/end");
                    }
                    "stoor" => {
                        screen = Screen::new("BattleScene/General-Use/stoor/end");
                    }
                    "school-cafeteria" => {
                        screen = Screen::new("BattleScene/General-Use/school-cafeteria/end");
                    }
                    "school-math" => {
                        screen = Screen::new("BattleScene/General-Use/school-math/end");
                    }
                    "school-english" => {
                        screen = Screen::new("BattleScene/General-Use/school-english/end");
                    }
                    "pond" => {
                        screen = Screen::new("BattleScene/General-Use/pond/end");
                    }
                    "library-tables" => {
                        screen = Screen::new("BattleScene/General-Use/library/end");
                    }
                    "house-living" => {
                        screen = Screen::new("BattleScene/General-Use/house-living/end");
                    }
                    "school-hall" => {
                        screen = Screen::new("BattleScene/General-Use/school-hall/end");
                    }
                    "school" => {
                        screen = Screen::new("BattleScene/General-Use/school/end");
                    }
                    "lhouses" => {
                        screen = Screen::new("BattleScene/General-Use/lhouses/end");
                    }
                    _ => {}
                }
                screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                screen.fight_write("you lost".to_string(), 90, 455);
                if time_count < 100 {
                    time_count = time_count + 1;
                } else {
                    battle = !battle;
                    screen = Screen::new(&last_scr);
                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                    screen.player.x_pos = x_save;
                    screen.player.y_pos = y_save;
                    fight_tracker = 0;
                    try_run = false;
                    run = false;
                    fight = false;
                    time_count = 0;
                    player_health = 4;
                    submit = false;

                }
            }
            if total_correct == 3 {
                match last_scr.as_str() {
                    "houses" => {
                        screen = Screen::new("BattleScene/General-Use/houses/end");
                    }
                    "stoor" => {
                        screen = Screen::new("BattleScene/General-Use/stoor/end");
                    }
                    "school-cafeteria" => {
                        screen = Screen::new("BattleScene/General-Use/school-cafeteria/end");
                    }
                    "school-math" => {
                        screen = Screen::new("BattleScene/General-Use/school-math/end");
                    }
                    "school-english" => {
                        screen = Screen::new("BattleScene/General-Use/school-english/end");
                    }
                    "pond" => {
                        screen = Screen::new("BattleScene/General-Use/pond/end");
                    }
                    "library-tables" => {
                        screen = Screen::new("BattleScene/General-Use/library/end");
                    }
                    "house-living" => {
                        screen = Screen::new("BattleScene/General-Use/house-living/end");
                    }
                    "school-hall" => {
                        screen = Screen::new("BattleScene/General-Use/school-hall/end");
                    }
                    "school" => {
                        screen = Screen::new("BattleScene/General-Use/school/end");
                    }
                    "lhouses" => {
                        screen = Screen::new("BattleScene/General-Use/lhouses/end");
                    }
                    _ => {}
                }
                screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                if time_count < 50 {
                    screen.fight_write("Nav wins the".to_string(), 75, 455);
                    screen.fight_write("battle".to_string(), 75, 490);
                } else {
                    battle = !battle;
                    screen = Screen::new(&last_scr);
                    screen.screen_len = screen.area.len() / (SCREEN_HEIGHT * 3) as usize;
                    screen.player.x_pos = x_save;
                    screen.player.y_pos = y_save;
                    fight_tracker = 0;
                    try_run = false;
                    run = false;
                    fight = false;
                    time_count = 0;
                    player_health = 4;
                    submit = false;
                    
                }
            }
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
                    if colliders[0] >= self.x_pos + scrolled {
                        if colliders[0] >= scrolled + self.x_pos - mvmt_dist
                            && colliders[0] <= scrolled + self.x_pos + CHAR_WIDTH - mvmt_dist
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
    fn fight_write(&mut self, text: String, x_pos: u16, y_pos: u16) {
        let mut x:u16 = x_pos;
        let mut y:u16 = y_pos;
        let mut lett: Entity;
        for letter in text.chars() {
            x += 30;
            if x >=630 {
                x = 68;
                y += 40;
            }
            if letter == ' '{
                continue
            } else {
                lett = Entity::new(
                    &format!("{}{}{}{}.txt", "SpriteData/battle letras/", letter, "/", letter),
                    &format!("{}{}{}{}.txt", "SpriteData/battle letras/", letter, "/", letter),
                    &format!("{}{}{}{}.txt", "SpriteData/battle letras/", letter, "/", letter),
                    &format!("{}{}{}{}.txt", "SpriteData/battle letras/", letter, "/", letter),
                    &format!("{}{}{}{}.txt", "SpriteData/battle letras/", letter, "/", letter),
                    &format!("{}{}", "battle letras/", letter,),
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
        // x: u16,
        // y: u16,
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

// fn _update(&mut self, sc) -> std::io::Result <()> {
//     std::fs::copy(self.place,"screen.txt");
//
// }
