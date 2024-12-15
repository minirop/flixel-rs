use crate::base::traits::ConcreteState;
use crate::base::traits::State;
use flixel_derive::state;
use crate::FlxState;

#[state]
pub struct EmptyState {
}

impl State for EmptyState {
    fn create(&mut self) {
    }

    fn update(&mut self, _: f64) {
    }
}
