use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::video::Window;
use sdl2::mouse::MouseButton;
use sdl2::event::Event;

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