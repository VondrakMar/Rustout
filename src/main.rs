extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::rect::Rect;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

mod utils;
use utils::utils::Button;

pub struct Dweller{
    pub dweller_body: Rect,
    pub x: i32,
    pub y: i32,
    pub dweller_color: Color
}

impl Dweller{
    pub fn new(x:i32,y:i32,v_x:f64,v_y:f64,col: Color)->Self{
        Self{
            dweller_body: Rect::new(x,y,20,20),
            x:x,
            y:y,
            dweller_color:col
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>){
        canvas.set_draw_color(self.dweller_color);
        canvas.fill_rect(self.dweller_body).ok().unwrap_or_default();
    }
    pub fn move_dweller(&mut self, keycode: Option<Keycode>){
        if keycode.unwrap() == sdl2::keyboard::Keycode::Right {
            let x = self.dweller_body.x();
            self.dweller_body.set_x(x + 5);
        }
        else if keycode.unwrap() == sdl2::keyboard::Keycode::Left {
            let x = self.dweller_body.x();
            self.dweller_body.set_y(x - 5);
        }
        else if keycode.unwrap() == sdl2::keyboard::Keycode::Up {
            let y = self.dweller_body.y();
            self.dweller_body.set_y(y - 5);
        }
        else if keycode.unwrap() == sdl2::keyboard::Keycode::Down {
            let y = self.dweller_body.y();
            self.dweller_body.set_y(y + 5);
        }
    }
}


fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("SDL2 Window", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;
    let mut main_dweller = Dweller::new(20,20,0.0,0.0,Color::RGB(5,5,5));
    let mut button = Button::new(350, 250, 100, 50);
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
                }
                
                _ => {
                    button.handle_event(&event);
                }
            }
        }
        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        canvas.clear();
        main_dweller.render(&mut canvas);
        button.render(&mut canvas);
        canvas.present();
        // Sleep for a short duration to avoid high CPU usage
        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
