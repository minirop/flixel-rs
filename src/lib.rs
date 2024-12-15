pub mod prelude;
pub mod text;
pub mod ui;
mod base;
mod raylib;
use crate::empty::EmptyState;
use raylib_ffi::colors::BLACK;
use raylib::*;
pub use base::*;
pub use flixel_derive::*;

use std::cell::RefCell;
use std::ops::Deref;

pub trait FlxObject {
    fn update(&mut self, elapsed: f64);
    fn draw(&self);
    fn enter(&mut self);
    fn leave(&mut self);
    fn click(&mut self);
}

pub struct FlxWidget {
    x: i32,
    y: i32,
}

impl FlxWidget {
    pub fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}

impl FlxObject for FlxWidget {
    fn update(&mut self, _elapsed: f64)
    {

    }

    fn draw(&self) {
        unimplemented!();
    }

    fn enter(&mut self) { todo!() }
    fn leave(&mut self) { todo!() }
    fn click(&mut self) { todo!() }
}

impl Default for FlxWidget {
    fn default() -> Self {
        Self {
            x: 0, y: 0,
        }
    }
}

pub struct FlxState {
    children: Vec<Box<dyn FlxObject>>,
}

impl Default for FlxState {
    fn default() -> Self {
        Self {
            children: vec![],
        }
    }
}

impl FlxState {
    pub fn add(&mut self, child: Box<dyn FlxObject>) {
        self.children.push(child);
    }

    fn draw(&self) {
        for child in &self.children {
            child.draw();
        }
    }

    pub fn update(&mut self, elapsed: f64) {
        for child in &mut self.children {
            child.update(elapsed);
        }
    }
}

impl Deref for dyn State {
    type Target = FlxState;

    fn deref(&self) -> &<Self as Deref>::Target {
        self.state()
    }
}

pub struct FlxG {
    state: Box<dyn State>,
}

thread_local! {
    static GLOBAL_STATE: RefCell<FlxG> = RefCell::new(FlxG {
        state: EmptyState::new(),
    });

    static NEXT_STATE: RefCell<Option<Box<dyn State>>> = RefCell::new(None);
}

impl FlxG {
    pub fn switch_state(state: Box<dyn State>) {
        NEXT_STATE.set(Some(state));
    }

    fn update(elapsed: f64) {
        GLOBAL_STATE.with_borrow_mut(|gs| {
            gs.state.update(elapsed);
        });
    }

    fn draw() {
        GLOBAL_STATE.with_borrow_mut(|gs| {
            gs.state.draw();

            if let Some(ns) = NEXT_STATE.take() {
                gs.state = ns;
                gs.state.create();
            }
        });
    }
}

pub fn main(width: i32, height: i32, initial_state: Box<dyn State>) {
    init_window(width, height, "Flixel.rs");
    set_target_fps(60);

    FlxG::switch_state(initial_state);

    'running: loop {
        if window_should_close() {
            break 'running;
        }

        FlxG::update(0.5);

        begin_drawing();
        clear_background(BLACK);
        FlxG::draw();
        end_drawing();
    }
}
