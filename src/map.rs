use std::cmp::{min, max};
use rltk::{Algorithm2D, BaseMap, Point};
use specs::prelude::*;

use crate::Player;

use super::rect::Rect;
use super::{Fov};

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Wall,
    Floor
}

pub struct Map {
    pub tiles : Vec<TileType>,
    pub rooms : Vec<Rect>,
    pub width : u32,
    pub height : u32,
    pub revealed_tiles: Vec<bool>,
}

impl Map {

    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    pub fn new_map_rooms_and_corridors() -> Map {
        let mut map = Map {
            tiles : vec![TileType::Wall; 80*50],
            rooms : Vec::new(),
            width : 80,
            height : 50,
            revealed_tiles : vec![false; 80*50],
        };
    
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
            for other_room in map.rooms.iter() {
                if new_room.is_intersect(other_room) {
                    ok = false
                }
            }
    
            if ok {
                map.add_room_to_map(&new_room);
                
                if !map.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].center();
                    if rng.range(0,2) == 1 {
                        map.add_horizontal_corridor(prev_x, new_x, prev_y);
                        map.add_vertical_corridor(prev_y, new_y, new_x);
                    } else {
                        map.add_vertical_corridor(prev_y, new_y, prev_x);
                        map.add_horizontal_corridor(prev_x, new_x, new_y);
                    }
                }
    
                map.rooms.push(new_room);
            }
        }
    
        map
    }

    fn add_room_to_map(&mut self, room : &Rect) {
        for y in room.y1 + 1 ..= room.y2 {
            for x in room.x1 +1 ..= room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }
    
    fn add_horizontal_corridor(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1,x2)..=max(x1,x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < 80*50 {
                self.tiles[idx] = TileType::Floor;
            }
        }
    }
    
    fn add_vertical_corridor(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1,y2)..=max(y1,y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < 80*50 {
                self.tiles[idx] = TileType::Floor;
            }
        }
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] == TileType::Wall
    }
}

pub fn map_draw(ecs: &World, ctx: &mut rltk::Rltk) {

    let map = ecs.fetch::<Map>();
    let mut x = 0;
    let mut y = 0;

    for (idx,tile) in map.tiles.iter().enumerate() {
        if map.revealed_tiles[idx] {
            match tile {
                TileType::Floor => {
                    ctx.set(x, y,
                        rltk::RGB::from_f32(0.5,0.5,0.5),
                        rltk::RGB::from_f32(0.,0.,0.),
                        rltk::to_cp437('.'));
                }
                TileType::Wall => {
                    ctx.set(x, y,
                        rltk::RGB::from_f32(0.0,1.0,0.0),
                        rltk::RGB::from_f32(0.,0.,0.),
                        rltk::to_cp437('#'));
                }
            }
        }

        x += 1;
        if x > (map.width - 1) {
            x = 0;
            y += 1;
        }
    }
}

pub fn xx2_map_draw(ecs: &World, ctx: &mut rltk::Rltk) {
    let mut viewsheds = ecs.write_storage::<Fov>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Map>();

    for (_pl, viewshed) in (&mut players, &mut viewsheds).join() {
        let mut x = 0;
        let mut y = 0;
        for tile in map.tiles.iter() {
            let pt = Point::new(x, y);
            
            if viewshed.visible_tiles.contains(&pt) {
                match tile {
                    TileType::Floor => {
                        ctx.set(x, y,
                            rltk::RGB::from_f32(0.5,0.5,0.5),
                            rltk::RGB::from_f32(0.,0.,0.),
                            rltk::to_cp437('.'));
                    }
                    TileType::Wall => {
                        ctx.set(x, y,
                            rltk::RGB::from_f32(0.0,1.0,0.0),
                            rltk::RGB::from_f32(0.,0.,0.),
                            rltk::to_cp437('#'));
                    }
                }
            }
        
            x += 1;
            if x > (map.width - 1) {
                x = 0;
                y += 1;
            }
        }
    }
}

pub fn xx1_map_draw(map: &Map, ctx: &mut rltk::Rltk) {
    let mut x = 0;
    let mut y = 0;

    for tile in map.tiles.iter() {
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
        if x > (map.width - 1){
            x = 0;
            y += 1;
        }
    }
}
