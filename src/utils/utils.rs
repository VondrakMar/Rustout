use sdl2::rect::{Rect,Point};
use sdl2::pixels::Color;
use sdl2::video::Window;
use sdl2::mouse::MouseButton;
use sdl2::event::Event;
use std::fs::File;
use std::fs;
use std::io::{self,Write,BufRead};
use std::path::Path;
use crate::dweller::Dweller;
use crate::building::Room;
use crate::building::Floor;

use crate::my_consts::LENGHT_UNIT;
use crate::my_consts::LENGHT_DWELLER;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub struct MapGrid{
    pub grid_spacing: usize,
    pub width: u32, 
    pub height: u32
}

impl MapGrid{
    pub fn new(spacing: usize,width: u32,height: u32) -> Self{
        MapGrid{ 
            grid_spacing: spacing,
            width: width,
            height: height
        }
    }

    pub fn render(&self,canvas: &mut sdl2::render::Canvas<Window>,list_of_dweller: &Vec<Dweller>,list_of_rooms: &Vec<Room>, list_of_floors: &Vec<Floor>){
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        for x in (0..self.width).step_by(self.grid_spacing) {
            canvas.draw_line(Point::new(x as i32, 0), Point::new(x as i32, self.height as i32)).unwrap();
        }
        for y in (0..self.height).step_by(self.grid_spacing) {
            canvas.draw_line(Point::new(0, y as i32), Point::new(self.width as i32, y as i32)).unwrap();
        }
        for dweller in list_of_dweller{
            dweller.render(canvas);
        }
        for room in list_of_rooms{
            room.render(canvas);
        }
        for floor in list_of_floors{
            floor.render(canvas);
        }
    }
    pub fn calculate_grid(&self){
        let n_rows = self.height/self.grid_spacing as u32;
        let n_cols = self.width/self.grid_spacing as u32;
        println!("Grid sizes are {} rows and {} cols", n_rows,n_cols);
    }

    pub fn save_map(&self){
        let n_rows = self.height/self.grid_spacing as u32;
        let n_cols = self.width/self.grid_spacing as u32;
        let mut my_map = String::new();
        for row in 0..=n_rows{
            for col in 0..=n_cols{
                my_map.push('#');
            }
            my_map.push('\n');
        }
        let mut map_file = File::create("my_map.txt").unwrap();
        map_file.write_all(my_map.as_bytes()).unwrap();
    }

    // pub fn load_map(&self,file_name: &str, canvas: &mut sdl2::render::Canvas<Window>,list_of_dweller: &mut Vec<Dweller>,list_of_rooms: &mut Vec<Room>, list_of_floors: &mut Vec<Floor>)
    pub fn load_map(&self, canvas: &mut sdl2::render::Canvas<Window>,list_of_dweller: &mut Vec<Dweller>,list_of_rooms: &mut Vec<Room>, list_of_floors: &mut Vec<Floor>)
    {
        let n_rows = self.height/self.grid_spacing as u32;
        let n_cols = self.width/self.grid_spacing as u32;
        let mut count_line = 0;
        // let my_map_string = fs::read_to_string("my_map.txt").expect("No map file");
        // for line in my_map_string.lines(){
        let mut current_line = -1;
        // let mut took_rooms_x Vec<u32> = vec![];
        // let mut took_rooms_y Vec<u32> = vec![];
        if let Ok(my_map_string) = read_lines("my_map_text.txt"){
            for line in my_map_string.flatten(){
                let line_items: Vec<&str> = line.split_whitespace().collect();
                if line_items[0] == "Dweller"{
                    println!("Loading a Dweller");
                    // list_of_dweller.push(Dweller::new(line_items[2]::parse<i32>()*LENGHT_DWELLER)+(2*LENGHT_UNIT),(line_items[1]::parse<i32>()*LENGHT_DWELLER)+(2*LENGHT_UNIT),0.0,0.0,Color::RGB(100,5,5));
                    let x: i32 = if let Ok(num) = line_items[1].parse::<i32>() {
                        num
                    } else {
                        10 // default location
                    };
            
                    let y: i32 = if let Ok(num) = line_items[2].parse::<i32>() {
                        num
                    } else {
                        10 // default location
                    };
                    list_of_dweller.push(Dweller::new((x*LENGHT_DWELLER)-(2*LENGHT_UNIT),(y*LENGHT_DWELLER)-(2*LENGHT_UNIT),0.0,0.0,Color::RGB(100,5,5)));
                }
                else if line_items[0] == "Room"{
		    println!("Loading room");
		    let (x,y,size_x,size_y) = if let (Ok(num1),Ok(num2),Ok(size1),Ok(size2)) = (line_items[1].parse::<i32>(),line_items[2].parse::<i32>(),line_items[3].parse::<i32>(),line_items[4].parse::<i32>()) {
			(num1,num2,size1,size2)
		    } else{
			(20,20,10,10)
		    };
		    list_of_rooms.push(Room::new((x*LENGHT_DWELLER)+(0*LENGHT_UNIT),(y*LENGHT_DWELLER)+(0*LENGHT_UNIT),size_x,size_y,Color::RGB(200,0,0)));
                }
		else if line_items[0] == "Floor"{
		    println!("Loading floor");
		    let (x,x_units,y) = if let (Ok(num1),Ok(size1),Ok(num2)) = (line_items[1].parse::<i32>(),line_items[2].parse::<i32>(),line_items[3].parse::<i32>()) {
			(num1,size1,num2)
		    } else{
			(20,2,10)
		    };
		    list_of_floors.push(Floor::new((x*LENGHT_DWELLER),x_units,(y*LENGHT_DWELLER)));
                }
            }
        }
        // 
        // if let Ok(my_map_string) = read_lines("my_map.txt"){
            // for row in my_map_string.flatten(){
                // current_line += 1;
                // let mut current_col = -1;
                // for col in row.chars(){
                    // current_col += 1;
                    // if col == '#'{ // this will be normal ground
                        // println!("loading a ground");
                        // ();
                    // }
                    // else if col == 'R'{  // room symbol
                        // 
                        // println!("loading a room");
                    // }
                    // else if col == 'F'{ // floor symbol
                        // println!("loading a floor");
                    // }
                    // else if col == 'D'{ // dweller symbol
                        // list_of_dweller.push(Dweller::new((current_col*LENGHT_DWELLER)+(2*LENGHT_UNIT),(current_line*LENGHT_DWELLER)+(2*LENGHT_UNIT),0.0,0.0,Color::RGB(100,5,5)));
                        // println!("loading a dweller {} {}",(current_line-1)*LENGHT_DWELLER,current_col*LENGHT_DWELLER);
                        // 
                    // }
                    // else {
                        // println!("uknnow symbol");
                    // }
                // }
            // }
        // }
        
        // list_of_floors.push(Floor::new(20,10,100));
        // list_of_floors.push(Floor::new(20,10,700));
        // list_of_rooms.push(Room::new(200,200,4,2,Color::RGB(200,0,0)));
        // list_of_rooms.push(Room::new(600,600,2,1,Color::RGB(200,100,0)));
        // list_of_dweller.push(Dweller::new(150,150,0.0,0.0,Color::RGB(5,5,5)));
        // list_of_dweller.push(Dweller::new(400,150,0.0,0.0,Color::RGB(5,0,0)));
        // list_of_dweller.push(Dweller::new(500,500,0.0,0.0,Color::RGB(50,150,50)));
    }
}



pub struct Button {
    rect: Rect,
    color: Color,
    hover_color: Color,
    click_color: Color,
    is_hovered: bool,
    is_clicked: bool,
}

impl Button {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Button {
            rect: Rect::new(x, y, width, height),
            color: Color::RGB(0, 0, 255),
            hover_color: Color::RGB(0, 0, 200),
            click_color: Color::RGB(0, 0, 150),
            is_hovered: false,
            is_clicked: false,
        }
    }

    pub fn render(&self, canvas: &mut sdl2::render::Canvas<Window>) {
        let color = if self.is_clicked {
            self.click_color
        } else if self.is_hovered {
            self.hover_color
        } else {
            self.color
        };
        canvas.set_draw_color(color);
        canvas.fill_rect(self.rect).unwrap();
    }

    pub fn handle_event(&mut self, event: &Event) {
        match event {
            Event::MouseMotion { x, y, .. } => {
                self.is_hovered = self.rect.contains_point((*x, *y));
            }
            Event::MouseButtonDown { x, y, mouse_btn, .. } => {
                if *mouse_btn == MouseButton::Left && self.rect.contains_point((*x, *y)) {
                    self.is_clicked = true;
                }
            }
            Event::MouseButtonUp { mouse_btn, .. } => {
                if *mouse_btn == MouseButton::Left {
                    self.is_clicked = false;
                }
            }
            _ => {}
        }
    }
}
