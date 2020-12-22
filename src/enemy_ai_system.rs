use specs::prelude::*;
use super::{Fov, Position, Enemy};
use rltk::console;

pub struct EnemyAISystem;

impl<'a> System<'a> for EnemyAISystem {
    type SystemData = (ReadStorage<'a, Position>,
                       ReadStorage<'a, Fov>,
                       ReadStorage<'a, Enemy>);
    fn run(&mut self, data: Self::SystemData) {
        let (pos, fov, enemy) = data;

        for (_pos, _fov, _enemy) in (&pos, &fov, &enemy).join() {
            console::log("enemy see you");
        }
    }
}