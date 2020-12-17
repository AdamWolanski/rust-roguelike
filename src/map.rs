use std::cmp::{min, max};
use super::{Rect};
//TODO: ⁉ use crate::components;

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Wall,
    Floor
}

const SCREEN_WIDTH: u32 = 80;
const SCREEN_HEIGHT: u32 = 50;

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

//TODO: 🔨 cleanup
// pub fn xx_new_map_create() -> Vec<TileType>{
//     let mut map = vec![TileType::Floor; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize];

//     for x in 0..SCREEN_WIDTH {
//         map[xy_idx(x, 0)] = TileType::Wall;
//         map[xy_idx(x, SCREEN_HEIGHT - 1)] = TileType::Wall;
//     }

//     for y in 0..SCREEN_HEIGHT {
//         map[xy_idx(0, y)] = TileType::Wall;
//         map[xy_idx(SCREEN_WIDTH - 1, y)] = TileType::Wall;
//     }
//     println!("map size: {}\n", map.len());

//     map
// }

pub fn map_draw(map: &[TileType], ctx: &mut rltk::Rltk) {
    let mut x = 0;
    let mut y = 0;
    
    for tile in map.iter() {
        match tile {
            TileType::Floor => {
                ctx.set(x, y, 
                    rltk::RGB::from_f32(0.5, 0.5, 0.5),
                    rltk::RGB::from_f32(0., 0., 0.),
                    rltk::to_cp437('.'));
            }
            TileType::Wall => {
                ctx.set(x, y,
                    rltk::RGB::from_f32(0., 1.0, 0.),
                    rltk::RGB::from_f32(0., 0., 0.),
                    rltk::to_cp437('#'));
            }
        }
        x += 1;
        if x > (SCREEN_WIDTH-1){
            x = 0;
            y += 1;
        }
    }
}

pub fn xx_new_map_rooms_and_corridors() -> Vec<TileType> {
    let mut map = vec![TileType::Wall; 80*50];

    let room1 = Rect::new(20, 15, 10, 15);
    let room2 = Rect::new(35, 15, 10, 15);

    add_room_to_map(&room1, &mut map);
    add_room_to_map(&room2, &mut map);
    add_horizontal_corridor(&mut map, 25, 40, 25);

    map
}

pub fn new_map_rooms_and_corridors() -> (Vec<Rect>, Vec<TileType>) {
    let mut map = vec![TileType::Wall; 80*50];
    let mut rooms : Vec<Rect> = Vec::new();

    const MAX_ROOMS: i32 = 30;
    const MIN_SIZE: i32 = 6;
    const MAX_SIZE: i32 = 10;

    let mut rng = rltk::RandomNumberGenerator::new();

    for _i in 0..MAX_ROOMS {
        let w = rng.range(MIN_SIZE, MAX_SIZE);
        let h = rng.range(MIN_SIZE, MAX_SIZE);
        let x = rng.roll_dice(1, 80 - w - 1) - 1;
        let y = rng.roll_dice(1, 50 - h - 1) - 1;
        let new_room = Rect::new(x, y, w, h);

        let mut ok = true;
        for other_room in rooms.iter() {
            if new_room.is_intersect(other_room) {
                ok = false
            }
        }

        if ok {
            add_room_to_map(&new_room, &mut map);
            
            if !rooms.is_empty() {
                let (new_x, new_y) = new_room.center();
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();
                if rng.range(0,2) == 1 {
                    add_horizontal_corridor(&mut map, prev_x, new_x, prev_y);
                    add_vertical_corridor(&mut map, prev_y, new_y, new_x);
                } else {
                    add_vertical_corridor(&mut map, prev_y, new_y, prev_x);
                    add_horizontal_corridor(&mut map, prev_x, new_x, new_y);
                }
            }

            rooms.push(new_room);
        }
    }

    (rooms, map)
}



fn add_room_to_map(room : &Rect, map : &mut [TileType]) {
    for y in room.y1 + 1 ..= room.y2 {
        for x in room.x1 +1 ..= room.x2 {
            map[xy_idx(x, y)] = TileType::Floor;
        }
    }
}

fn add_horizontal_corridor(map: &mut [TileType], x1: i32, x2: i32, y: i32) {
    for x in min(x1,x2)..=max(x1,x2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < 80*50 {
            map[idx] = TileType::Floor;
        }
    }
}

fn add_vertical_corridor(map: &mut [TileType], y1: i32, y2: i32, x: i32) {
    for y in min(y1,y2)..=max(y1,y2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < 80*50 {
            map[idx] = TileType::Floor;
        }
    }
}