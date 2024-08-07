extern crate sdl2;

use sdl2::rect::Point;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
// use sdl2::rect::Rect;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;


mod utils;
// use utils::utils::Button;
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
    if (dweller.center_x == room.x+10 && ghost_dweller.center_x == room.x-10) && (dweller.center_y > room.y && dweller.center_y < room.y + LENGHT_ROOM) {
        room.dweller_entered(canvas);
    }
    else if (dweller.center_x == room.x-10 && ghost_dweller.center_x == room.x+10) && (dweller.center_y > room.y && dweller.center_y < room.y + LENGHT_ROOM) {
        room.dweller_left(canvas);
    }
}

fn is_dweller_on_the_floor(dweller: &Dweller, floors: &Vec<Floor>)->bool{
    for floor in floors{
        if dweller.center_x > floor.x_start && dweller.center_x < floor.x_start + floor.x_units*LENGHT_DWELLER{
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
    let mut event_pump = sdl_context.event_pump()?;
    let mut main_dweller = Dweller::new(150,150,0.0,0.0,Color::RGB(5,5,5));
    // let mut second_dweller = Dweller::new(150,150,0.0,0.0,Color::RGB(5,5,5));
    let mut main_room = Room::new(200,200,4,2,Color::RGB(200,0,0));
    let mut test_floor = Floor::new(80,10,100);
    let mut test_floor2 = Floor::new(160,50,700);
    // let mut button = Button::new(350, 250, 100, 50);
    let mut list_of_floors: Vec<Floor> = vec![];
    list_of_floors.push(test_floor);
    list_of_floors.push(test_floor2);

    'running: loop {
        let is_floor = is_dweller_on_the_floor(&main_dweller,&list_of_floors);
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {break 'running;}
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                }
                Event::KeyDown{keycode: Some(keycode),..}=>{
                    if keycode == sdl2::keyboard::Keycode::Up{
                        move_dweller_up(&mut main_dweller,is_floor,Some(keycode));
                    }
                    else if keycode == sdl2::keyboard::Keycode::Down{
                        move_dweller_down(&mut main_dweller,is_floor,Some(keycode));                    
                    }    
                    else if keycode == sdl2::keyboard::Keycode::Left||keycode == sdl2::keyboard::Keycode::Right{
                        move_dweller_horizontlly(&mut main_dweller,Some(keycode));
                        // let ghost_dweller = Dweller::new(main_dweller.center_x,main_dweller.center_y,0.0,0.0,Color::RGB(25,25,25));
                        // main_dweller.move_dweller(Some(keycode));
                        // is_dweller_in(&main_dweller,&ghost_dweller,&mut main_room,&mut canvas);
                    }
                }
                        
                
                _ => {
                    // button.handle_event(&event);
                }
            }
        }
        println!("{}",is_floor);
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        if true{
            let (width, height) = canvas.output_size().unwrap();
            let grid_spacing = LENGHT_DWELLER as usize;
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            for x in (0..width).step_by(grid_spacing) {
                canvas.draw_line(Point::new(x as i32, 0), Point::new(x as i32, height as i32)).unwrap();
            }
            for y in (0..height).step_by(grid_spacing) {
                canvas.draw_line(Point::new(0, y as i32), Point::new(width as i32, y as i32)).unwrap();
            }
        }

        for floor in &list_of_floors{
            floor.render(&mut canvas);
        }
        main_dweller.render(&mut canvas);
        main_room.render(&mut canvas);
        // test_floor.render(&mut canvas);
        // button.render(&mut canvas);
        canvas.present();
        // Sleep for a short duration to avoid high CPU usage
        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
