use specs::prelude::*;
use rltk::{field_of_view, Point};

use super::{Fov, Position, Map, Player};

pub struct FovSystem;

impl<'a> System<'a> for FovSystem {
    type SystemData = (WriteExpect<'a, Map>,
                       Entities<'a>,
                       WriteStorage<'a, Fov>,
                       WriteStorage<'a, Position>,
                       ReadStorage<'a, Player>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, entities, mut viewshed, pos, player) = data;

        for (e, viewshed, pos) in (&entities, &mut viewshed, &pos).join() {
            viewshed.visible_tiles.clear();
            viewshed.visible_tiles = field_of_view(Point::new(pos.x, pos.y),
                                                    viewshed.range, &*map);
            viewshed.visible_tiles.retain(|p| p.x >= 0 &&
                                              p.x <  map.width as i32 &&
                                              p.y >= 0 &&
                                              p.y <  map.width as i32);

            let p : Option<&Player> = player.get(e);
            if let Some(_tmp) = p {
                for v in viewshed.visible_tiles.iter() {
                    let idx = map.xy_idx(v.x, v.y);
                    map.revealed_tiles[idx] = true;
                }
            }
        }
    }
}