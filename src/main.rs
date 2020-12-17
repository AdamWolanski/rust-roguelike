mod components;
mod player;
mod map;
mod rect;

use rltk::{Rltk, RltkBuilder, GameState};
use specs::prelude::*;

pub use components::*;
pub use player::*;
pub use map::*;
pub use rect::*;

pub struct State {
    ecs : World,
}
impl State {
    fn run_systems(&mut self) {
        let mut lw = LeftMover;
        lw.run_now(&self.ecs);
        self.ecs.maintain();
    }
}
impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        
        ctx.cls();
        
        player_input(self, ctx);
        self.run_systems();
        let map = self.ecs.fetch::<Vec<TileType>>();
        map_draw(&map, ctx);
        
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        
        for (pos, rend) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, rend.fg, rend.bg, rend.glyph);
        }
    }
}

impl<'a> System<'a> for LeftMover {
    type SystemData = (ReadStorage<'a, LeftMover>,
                        WriteStorage<'a, Position>);
    fn run(&mut self, (lefty, mut pos) : Self::SystemData) {
        for (_lefty, pos) in (&lefty, &mut pos).join() {
            pos.x -= 1;
            if pos.x < 0 {
                pos.x = 79;
            }
        }
    }
}

fn components_register(gs : &mut State) {
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();
}

fn main() -> rltk::BError {
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    let mut game_state = State {
        ecs : World::new()
    };

    components_register(&mut game_state);

    let (rooms, map) = map::new_map_rooms_and_corridors();
    game_state.ecs.insert(map);
    let (player_x, player_y) = rooms[0].center();

    game_state.ecs
        .create_entity()
        .with(Position{
            x:player_x, y:player_y
        })
        .with(Renderable{
            glyph: rltk::to_cp437('@'),
            fg: rltk::RGB::named(rltk::YELLOW),
            bg: rltk::RGB::named(rltk::BLACK),
        })
        //.with(LeftMover)
        .with(Player)
        .build();

    rltk::main_loop(context, game_state)
}