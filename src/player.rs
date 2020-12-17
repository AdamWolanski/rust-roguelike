use super::{Position, Player, TileType, Map, State};
use specs::prelude::*;
use std::cmp::{min, max};

const SCREEN_WIDTH: u32 = 80;
const SCREEN_HEIGHT: u32 = 50;

pub fn player_move(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Map>();
    
    for (pos, _pl) in (&mut positions, &mut players).join() {
        let dest_pos = map.xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map.tiles[dest_pos] != TileType::Wall {
            pos.x = min((SCREEN_WIDTH - 1) as i32, max(0, pos.x + delta_x));
            pos.y = min((SCREEN_HEIGHT - 1) as i32, max(0, pos.y + delta_y));
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut rltk::Rltk) {
    match ctx.key {
        None => {},
        Some(key) => match key {
			rltk::VirtualKeyCode::W |
            rltk::VirtualKeyCode::Up => player_move(0, -1, &mut gs.ecs),
            rltk::VirtualKeyCode::Down => player_move(0, 1, &mut gs.ecs),
            rltk::VirtualKeyCode::Left => player_move(-1, 0, &mut gs.ecs),
            rltk::VirtualKeyCode::Right => player_move(1, 0, &mut gs.ecs),
            _ => {}
        }
    }
}