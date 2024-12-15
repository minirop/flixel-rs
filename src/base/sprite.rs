use crate::draw_texture;
use raylib_ffi::colors::WHITE;
use crate::load_texture;
use raylib_ffi::Texture2D;
use crate::FlxObject;
use crate::FlxWidget;
use crate::widget;

#[widget]
pub struct FlxSprite {
    texture: Option<Texture2D>,
}

impl FlxSprite {
    pub fn load_graphic(&mut self, filename: &str) {
        self.texture = Some(load_texture(filename));
    }
}

impl FlxObject for FlxSprite {
    fn update(&mut self, _elapsed: f64) {
    }

    fn draw(&self) {
        if let Some(tex) = &self.texture {
            draw_texture(*tex, 10, 10, WHITE);
        }
    }

    fn enter(&mut self) {}
    fn leave(&mut self) {}
    fn click(&mut self) {}
}
