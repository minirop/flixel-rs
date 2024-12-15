use flixel::State;
use flixel::ui::FlxButton;
use flixel::prelude::*;
use flixel::FlxSprite;
use flixel::text::FlxText;
use flixel::FlxG;

#[state]
struct MenuState {
}

impl State for MenuState {
    fn create(&mut self) {
        let text = FlxText::new_with(200, 20, "Hello World", 64);
        self.add(text);

        let mut sprite = FlxSprite::new();
        sprite.load_graphic("image.png");
        self.add(sprite);

        let button = FlxButton::new_with(20, 200, "click", || {
            println!("clicked!");
            FlxG::switch_state(PlayState::new());
        });
        self.add(button);
    }

    fn update(&mut self, elapsed: f64) {
        self.parent.update(elapsed);
    }
}

#[state]
struct PlayState {
}

impl State for PlayState {
    fn create(&mut self) {
        let text = FlxText::new_with(200, 20, "play World", 64);
        self.add(text);
    }

    fn update(&mut self, _elapsed: f64) {
    }
}

fn main() {
    let menu_state = MenuState::new();
    flixel::main(800, 600, menu_state);
}
