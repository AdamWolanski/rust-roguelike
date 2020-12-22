use super::{Position, Player, TileType, Map, State, Fov, RunState};
use specs::prelude::*;
use std::cmp::{min, max};

const SCREEN_WIDTH: u32 = 80;
const SCREEN_HEIGHT: u32 = 50;

pub fn player_move(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut fovs = ecs.write_storage::<Fov>();
    let map = ecs.fetch::<Map>();
    
    for (position, _player, fov) in (&mut positions, &mut players, &mut fovs).join() {
        let dest_pos = map.xy_idx(position.x + delta_x, position.y + delta_y);
        if map.tiles[dest_pos] != TileType::Wall {
            position.x = min((SCREEN_WIDTH - 1) as i32, max(0, position.x + delta_x));
            position.y = min((SCREEN_HEIGHT - 1) as i32, max(0, position.y + delta_y));

            fov.dirty = true;
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut rltk::Rltk) -> RunState {
    match ctx.key {
        None => { return RunState::Paused },
        Some(key) => match key {
            rltk::VirtualKeyCode::W |
            rltk::VirtualKeyCode::Up => player_move(0, -1, &mut gs.ecs),
            rltk::VirtualKeyCode::Down => player_move(0, 1, &mut gs.ecs),
            rltk::VirtualKeyCode::Left => player_move(-1, 0, &mut gs.ecs),
            rltk::VirtualKeyCode::Right => player_move(1, 0, &mut gs.ecs),
            _ => { return RunState::Paused }
        }
    }
    return RunState::Running;
}