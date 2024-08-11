extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::rect::Rect;
use sdl2::video::Window;
use sdl2::keyboard::Keycode;

use crate::my_consts::LENGHT_UNIT;
// use crate::my_consts::LENGHT_ROOM;
use crate::my_consts::LENGHT_DWELLER;


pub struct Dweller{
    pub dweller_body: Rect,
    pub x: i32,
    pub y: i32,
    pub center_x: i32,
    pub center_y: i32,
    pub dweller_color: Color,
    pub is_hovered: bool,
    working_gravity: bool
}


impl Dweller{
    pub fn new(x:i32,y:i32,v_x:f64,v_y:f64,col: Color)->Self{
        Self{
            dweller_body: Rect::new(x - 2*LENGHT_UNIT,y - 2*LENGHT_UNIT,LENGHT_DWELLER as u32,LENGHT_DWELLER as u32),
            center_x: x,
            center_y: y,
            x:x - 2*LENGHT_UNIT,
            y:y - 2*LENGHT_UNIT,
            dweller_color:col,
            is_hovered: false,
            working_gravity: false
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>){
        canvas.set_draw_color(self.dweller_color);
        canvas.fill_rect(self.dweller_body).ok().unwrap_or_default();
    }
    pub fn move_dweller(&mut self, keycode: Option<Keycode>){
        if keycode.unwrap() == sdl2::keyboard::Keycode::Right {
            self.x += LENGHT_DWELLER;
            // self.center_x += LENGHT_DWELLER;
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
    pub fn free_fall(&mut self,is_floor:bool, speed: i32){
        if !is_floor && self.working_gravity{
            self.y += LENGHT_DWELLER*speed;
            self.center_y += LENGHT_DWELLER*speed;
            self.dweller_body.set_y(self.y);
        }
    }
    pub fn restart_position(&mut self){
        self.center_y = 150;
        self.center_x = 150;
        self.x = 150 - 2*LENGHT_UNIT;
        self.y = 150 - 2*LENGHT_UNIT;
        self.dweller_body.set_y(self.y);
        self.dweller_body.set_x(self.x);
        self.working_gravity = !self.working_gravity; 
    }
}