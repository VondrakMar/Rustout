use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::rect::Rect;
use sdl2::video::Window;
// use sdl2::keyboard::Keycode;

use crate::my_consts::LENGHT_UNIT;
use crate::my_consts::LENGHT_ROOM;
use crate::my_consts::LENGHT_DWELLER;

pub struct Room{
    pub room: Rect,
    pub x:i32,
    pub x_lenght: i32,
    pub y: i32,
    pub y_lenght: i32,
    pub color: Color,
    pub dweller_in: bool
}

impl Room{
    pub fn new(x:i32,y:i32,x_lenght:i32,y_lenght:i32,color:Color)->Self{
        Self{
            room: Rect::new(x,y,(LENGHT_ROOM*x_lenght) as u32,(LENGHT_ROOM*y_lenght) as u32),
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

pub struct Floor{
    pub floor: Rect,
    pub x_start: i32,
    pub x_units: i32,
    pub y: i32,
    pub h: i32
}

impl Floor{
    pub fn new(x_start:i32,x_units:i32,y:i32)->Self{
        Self{
            floor: Rect::new(x_start,y,(LENGHT_DWELLER*x_units) as u32,LENGHT_DWELLER as u32),
            x_start: x_start,
            x_units: x_units,
            y: y,
            h: LENGHT_DWELLER
        }
    }
    pub fn render(&self, canvas: &mut Canvas<Window>){
        canvas.set_draw_color(Color::RGB(200,200,200));
        canvas.fill_rect(self.floor).ok().unwrap_or_default();
    }
}