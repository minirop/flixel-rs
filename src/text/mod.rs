use crate::draw_text;
use raylib_ffi::colors::WHITE;
use crate::FlxObject;
use flixel_derive::widget;
use crate::FlxWidget;

#[widget]
pub struct FlxText {
    text: String,
    size: i32,
}

impl FlxText {
    pub fn new_with(x: i32, y: i32, text: &str, size: i32) -> Box<Self> {
        let mut s = Self::new();
        s.set_position(x, y);
        s.set_text(text);
        s.set_size(size);
        s
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    pub fn set_size(&mut self, size: i32) {
        self.size = size;
    }
}

impl FlxObject for FlxText {
    fn update(&mut self, _: f64) {}

    fn draw(&self) {
        draw_text(&self.text, self.parent.x, self.parent.y, self.size, WHITE);
    }
    
    fn enter(&mut self) {}
    fn leave(&mut self) {}
    fn click(&mut self) {}
}
