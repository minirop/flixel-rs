use crate::is_mouse_button_pressed;
use crate::measure_text;
use crate::get_mouse_position;
use crate::draw_text;
use crate::draw_rectangle;
use crate::FlxObject;
use flixel_derive::widget;
use crate::FlxWidget;
use raylib_ffi::colors::*;

#[widget]
pub struct FlxButton {
    text: String,
    hovered: bool,
    on_click: Option<fn()>,
    width: i32,
    height: i32,
}

impl FlxButton {
    pub fn new_with(x: i32, y: i32, text: &str, click: fn()) -> Box<Self> {
        let mut s = Self::new();
        s.set_position(x, y);
        s.set_text(text);
        s.on_click = Some(click);
        s
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
        self.width = measure_text(text, 24) + 20;
        self.height = 30;
    }
}

impl FlxObject for FlxButton {
    fn update(&mut self, _elapsed: f64) {
        let mouse = get_mouse_position();
        let x = mouse.x as i32;
        let y = mouse.y as i32;

        if x < self.parent.x || y < self.parent.y
        || x > self.parent.x + self.width || y > self.parent.y + self.height {
            if self.hovered {
                self.leave();
            }
            self.hovered = false;
            return;
        }

        if !self.hovered {
            self.enter();
        }
        self.hovered = true;
        if is_mouse_button_pressed(0) {
            self.click();
        }
    }

    fn draw(&self) {
        let left = (self.width - measure_text(&self.text, 24)) / 2;
        draw_rectangle(self.parent.x, self.parent.y, self.width, 30, if self.hovered { BLUE } else { RED });
        draw_text(&self.text, self.parent.x + left, self.parent.y + 3, 24, WHITE);
    }
    
    fn enter(&mut self) {}
    fn leave(&mut self) {}
    fn click(&mut self) {
        if let Some(fun) = self.on_click {
            fun();
        }
    }
}
