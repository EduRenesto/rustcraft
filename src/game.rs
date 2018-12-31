use std::boxed::Box;

use crate::block_manager::BlockManager;
use crate::block::Block;
use crate::actor::Actor;
use crate::test_actor::TestActor;

type IVec2 = cgmath::Vector2<u32>;

pub struct Game {
    actors: Vec<Box<dyn Actor>>,
}

impl Game {
    pub fn new() -> Game {
        let mut manager = BlockManager::new();

        manager.add_block(0, Block::new("Air", false, vec![]));
        manager.add_block(1, Block::new("Stone", false, vec![IVec2::new(1, 0)]));
        manager.add_block(2, Block::new("Dirt", false, vec![IVec2::new(2, 0)]));

        let a = TestActor::new(&manager);
        Game { actors: vec![Box::new(a)] }
    }

    pub fn render(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        for actor in self.actors.iter() {
            actor.render();
        }
    }

    pub fn update(&self) {
        for actor in self.actors.iter() {
            actor.update();
        }
    }
}
