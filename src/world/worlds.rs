use bevy::prelude::{Schedule, World};
use slab::Slab;

pub struct Worlds {
    worlds: Slab<World>,
    scheduler: Schedule,
}

impl Worlds {
    pub fn new() -> Self {
        let mut scheduler = Schedule::new();
        scheduler.add_system(test_system);

        let mut worlds = Self {
            worlds: Slab::new(),
            scheduler
        };

        worlds.init();
        worlds
    }

    pub fn init(&mut self) {
        self.new_world();
    }

    pub fn new_world(&mut self) -> usize {
        let world = World::new();
        let key = self.worlds.insert(world);
        self.scheduler.run(self.worlds.get_mut(key).unwrap());
        key
    }
}

fn test_system() {
    println!("Helli!")
}