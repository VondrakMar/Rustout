extern crate sdl2;

use sdl2::rect::Point;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
// use sdl2::rect::Rect;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use std::time::Duration;


mod utils;
// use utils::utils::Button;
use utils::utils::MapGrid;
mod dweller;
use dweller::Dweller;
mod my_consts;
// use crate::my_consts::LENGHT_UNIT;
use crate::my_consts::LENGHT_ROOM;
use crate::my_consts::LENGHT_DWELLER;
mod building;
use building::Room;
use building::Floor;



fn is_dweller_in(dweller: &Dweller,ghost_dweller: &Dweller,room: &mut Room,canvas: &mut Canvas<Window>){
    if (dweller.x == room.x+10 && ghost_dweller.x == room.x-10) && (dweller.y > room.y && dweller.y < room.y + LENGHT_ROOM) {
        room.dweller_entered(canvas);
    }
    else if (dweller.x == room.x && ghost_dweller.x == room.x) && (dweller.y > room.y && dweller.y < room.y + LENGHT_ROOM) {
        room.dweller_left(canvas);
    }
}

fn is_dweller_on_the_floor(dweller: &Dweller, floors: &Vec<Floor>)->bool{
    for floor in floors{
        if dweller.x >= floor.x_start && dweller.x < floor.x_start + floor.x_units*LENGHT_DWELLER{
            if dweller.center_y+10 == floor.y{
                return true;
            }
        }
    }
    return false;
}

fn move_dweller_horizontlly(dweller: &mut Dweller, keycode: Option<Keycode>){
    dweller.move_dweller(keycode);
}

fn move_dweller_up(dweller: &mut Dweller, is_on_floor: bool, keycode: Option<Keycode>){
    dweller.move_dweller(keycode);
}

fn move_dweller_down(dweller: &mut Dweller, is_on_floor: bool, keycode: Option<Keycode>){
    if !is_on_floor{
        dweller.move_dweller(keycode);
    }
    else{
        ();
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("Super Duper game", 1200, 1000)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let (map_width, map_height) = canvas.output_size().unwrap();
    let mut my_grid = MapGrid::new(LENGHT_DWELLER as usize, map_width, map_height);
    my_grid.calculate_grid();
    // my_grid.save_map();
    let mut event_pump = sdl_context.event_pump()?;
    let mut list_of_floors: Vec<Floor> = vec![];
    let mut list_of_rooms: Vec<Room> = vec![];
    let mut list_of_dweller: Vec<Dweller> = vec![];
    my_grid.load_map(&mut canvas,&mut list_of_dweller,&mut list_of_rooms,&mut list_of_floors);
    let mut id_active_dweller: usize = 0;
    'running: loop {
        let is_floor = is_dweller_on_the_floor(&mut list_of_dweller[id_active_dweller],&list_of_floors);
        list_of_dweller[id_active_dweller].free_fall(is_floor,1);
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {break 'running;}
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                }
                Event::KeyDown{keycode: Some(keycode),..}=>{
                    if keycode == sdl2::keyboard::Keycode::Up{
                        move_dweller_up(&mut list_of_dweller[id_active_dweller],is_floor,Some(keycode));  
                    }
                    else if keycode == sdl2::keyboard::Keycode::Down{
                        move_dweller_down(&mut list_of_dweller[id_active_dweller],is_floor,Some(keycode));                    
                    }    
                    else if keycode == sdl2::keyboard::Keycode::Left||keycode == sdl2::keyboard::Keycode::Right{
                        // move_dweller_horizontlly(&mut main_dweller,Some(keycode));
                        move_dweller_horizontlly(&mut list_of_dweller[id_active_dweller],Some(keycode));
                    }
                    else if keycode == sdl2::keyboard::Keycode::R{
                        list_of_dweller[id_active_dweller].restart_position();
                    }
                }
                // Event::MouseMotion { x, y, .. } => {
                    // for (index, dweller) in list_of_dweller.iter_mut().enumerate() {
                        // dweller.is_hovered = dweller.dweller_body.contains_point((x, y));
                    // }
                // }
                Event::MouseButtonDown { x, y, mouse_btn, .. } => {
                    for (index, dweller) in list_of_dweller.iter_mut().enumerate() {
                        if mouse_btn == MouseButton::Left && dweller.dweller_body.contains_point((x, y)) {
                            id_active_dweller = index;
                        }
                    }
                }
                // Event::MouseButtonDown {x,y,...}{
                    // self.rect.contains_point((*x, *y));
                // } 
                        
                
                _ => {
                    // button.handle_event(&event);
                }
            }
        }
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        my_grid.render(&mut canvas,&list_of_dweller,&list_of_rooms,&list_of_floors);
        canvas.present();
        // std::thread::sleep(Duration::from_millis(16));
        std::thread::sleep(Duration::from_millis(50));
    }

    Ok(())
}
