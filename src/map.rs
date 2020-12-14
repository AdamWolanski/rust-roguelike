use std::cmp::{min, max};

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Wall,
    Floor
}

const SCREEN_WIDTH: u32 = 80;
const SCREEN_HEIGHT: u32 = 50;

pub fn xy_idx(x: u32, y: u32) -> usize {
    (y as usize * 80) + x as usize
}

pub fn new_map_create() -> Vec<TileType>{
    let mut map = vec![TileType::Floor; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize];

    for x in 0..SCREEN_WIDTH {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, SCREEN_HEIGHT - 1)] = TileType::Wall;
    }

    for y in 0..SCREEN_HEIGHT {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(SCREEN_WIDTH - 1, y)] = TileType::Wall;
    }
    println!("map size: {}\n", map.len());

    map
}

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