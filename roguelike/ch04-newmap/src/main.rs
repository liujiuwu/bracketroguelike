use bracket_lib::prelude::*;
use specs::prelude::*;
use roguelike::prelude::*;

fn main() -> BError {
    let ctx = BTermBuilder::simple80x50().with_title("Walk new map").build()?;

    let mut gs = State { ecs: World::new() };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    let (rooms, map) = new_map_rooms_and_corridors();
    gs.ecs.insert(map);

    let player_center = rooms[0].center();

    gs.ecs.create_entity()
        .with(Position { x: player_center.x, y: player_center.y })
        .with(Renderable { glyph: to_cp437('@'), fg: RGB::named(YELLOW), bg: RGB::named(BLACK) })
        .with(Player {})
        .build();

    main_loop(ctx, gs)
}
