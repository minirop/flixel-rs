use crate::FlxState;

pub trait ConcreteState {
    fn state(&self) -> &FlxState;
}

pub trait State: ConcreteState {
    fn create(&mut self);
    fn update(&mut self, elapsed: f64);
}
