extern crate sdl2;

use sdl2::rect::Point;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::rect::Rect;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;


mod utils;
use utils::utils::Button;

const LENGHT_UNIT: i32 = 5;
const LENGHT_DWELLER: i32 = 4*LENGHT_UNIT;
const LENGHT_ROOM: i32 = 20*LENGHT_UNIT;

pub struct Dweller{
    pub dweller_body: Rect,
    pub x: i32,
    pub y: i32,
    pub center_x: i32,
    pub center_y: i32,
    pub dweller_color: Color
}


impl Dweller{
    pub fn new(x:i32,y:i32,v_x:f64,v_y:f64,col: Color)->Self{
        Self{
            dweller_body: Rect::new(x - 2*LENGHT_UNIT,y - 2*LENGHT_UNIT,LENGHT_DWELLER as u32,LENGHT_DWELLER as u32),
            center_x: x,
            center_y: y,
            x:x - 2*LENGHT_UNIT,
            y:y - 2*LENGHT_UNIT,
            dweller_color:col
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>){
        canvas.set_draw_color(self.dweller_color);
        canvas.fill_rect(self.dweller_body).ok().unwrap_or_default();
    }
    pub fn move_dweller(&mut self, keycode: Option<Keycode>){
        if keycode.unwrap() == sdl2::keyboard::Keycode::Right {
            self.x += LENGHT_DWELLER;
            self.center_x += LENGHT_DWELLER;
            self.dweller_body.set_x(self.x);
        }
        else if keycode.unwrap() == sdl2::keyboard::Keycode::Left {
            self.x -= LENGHT_DWELLER;
            self.center_x -= LENGHT_DWELLER;
            self.dweller_body.set_x(self.x);
        }
        else if keycode.unwrap() == sdl2::keyboard::Keycode::Up {
            self.y -= LENGHT_DWELLER;
            self.center_y -= LENGHT_DWELLER;
            self.dweller_body.set_y(self.y);
        }
        else if keycode.unwrap() == sdl2::keyboard::Keycode::Down {
            self.y += LENGHT_DWELLER;
            self.center_y += LENGHT_DWELLER;
            self.dweller_body.set_y(self.y);
        }
    }
}

pub struct Room{
    pub room: Rect,
    pub x:i32,
    pub x_lenght: i16,
    pub y: i32,
    pub y_lenght: i16,
    pub color: Color,
    pub dweller_in: bool
}

impl Room{
    pub fn new(x:i32,y:i32,x_lenght:i16,y_lenght:i16,color:Color)->Self{
        Self{
            room: Rect::new(x,y,LENGHT_ROOM as u32,LENGHT_ROOM as u32),
            x: x,
            y: y,
            x_lenght: x_lenght,
            y_lenght: y_lenght,
            color: color,
            dweller_in: false
        }
    }
    pub fn render(&self, canvas: &mut Canvas<Window>){
        canvas.set_draw_color(self.color);
        canvas.fill_rect(self.room).ok().unwrap_or_default();
    }

    pub fn dweller_entered(&mut self, canvas: &mut Canvas<Window>){
        self.dweller_in = true;
        let g = self.color.g.saturating_add(50);
        self.color = Color::RGB(self.color.r,g,self.color.b);
        canvas.set_draw_color(self.color);
        canvas.fill_rect(self.room).ok().unwrap_or_default();
    }
    pub fn dweller_left(&mut self, canvas: &mut Canvas<Window>){
        self.dweller_in = false;
        let mut g = self.color.g.saturating_sub(50);
        self.color = Color::RGB(self.color.r,g,self.color.b);
        canvas.set_draw_color(self.color);
        canvas.fill_rect(self.room).ok().unwrap_or_default();
    }

}

fn is_dweller_in(dweller: &Dweller,room: &mut Room,canvas: &mut Canvas<Window>){
    println!("room.x {} dweller.x {} dweller.center_x {} dweller.y {} dweller.center_y {} room.y {}",room.x,dweller.x,dweller.center_x,dweller.y,dweller.center_y,room.y);
    if (dweller.center_x == room.x-10 || dweller.center_x == room.x + LENGHT_ROOM-10) && (dweller.center_y > room.y && dweller.center_y < room.y + LENGHT_ROOM) {
        if room.dweller_in{
            room.dweller_left(canvas)
        }
        else {
            room.dweller_entered(canvas)
        }
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("SDL2 Window", 1200, 1000)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;
    let mut main_dweller = Dweller::new(150,150,0.0,0.0,Color::RGB(5,5,5));
    let mut main_room = Room::new(200,200,4,2,Color::RGB(200,0,0));
    // let mut button = Button::new(350, 250, 100, 50);

    
    
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {break 'running;}
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                }
                Event::KeyDown{keycode: Some(keycode),..}
                if keycode == sdl2::keyboard::Keycode::Down||
                keycode == sdl2::keyboard::Keycode::Up||
                keycode == sdl2::keyboard::Keycode::Left||
                keycode == sdl2::keyboard::Keycode::Right =>{
                    main_dweller.move_dweller(Some(keycode));
                    is_dweller_in(&main_dweller,&mut main_room,&mut canvas);
                    
                }
                
                _ => {
                    // button.handle_event(&event);
                }
            }
        }

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


        main_dweller.render(&mut canvas);
        main_room.render(&mut canvas);
        // button.render(&mut canvas);
        canvas.present();
        // Sleep for a short duration to avoid high CPU usage
        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
