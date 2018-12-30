use std::boxed::Box;

use crate::actor::Actor;
use crate::test_actor::TestActor;

pub struct Game {
    actors: Vec<Box<dyn Actor>>
}

impl Game {
    pub fn new() -> Game {
        let a = TestActor::new();
        Game { actors: vec![Box::new(a)] }
    }

    pub fn render(&self) {
        unsafe {
            gl::ClearColor(1.0, 0.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        for actor in self.actors.iter() {
            actor.render();
        }
    }
}
