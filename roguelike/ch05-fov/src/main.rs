use bracket_lib::prelude::*;
use specs::prelude::*;
use roguelike::*;

const WIDTH: i32 = 80;
const HEIGHT: i32 = 50;

embedded_resource!(TILE_FONT, "../resources/sprite.png");

fn main() -> BError {
    link_resource!(TILE_FONT, "resources/sprite.png");

    let ctx = BTermBuilder::new()
        .with_title("Walk fov")
        .with_dimensions(WIDTH, HEIGHT)
        .with_tile_dimensions(16u32, 16u32)
        .with_font("sprite.png", 16u32, 16u32)
        .with_sparse_console_no_bg(WIDTH, HEIGHT, "sprite.png")
        .with_sparse_console_no_bg(WIDTH, HEIGHT, "sprite.png")
        .build()?;

    let mut gs = State { ecs: World::new() };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();

    let map = Map::new_map_rooms_and_corridors(WIDTH, HEIGHT);
    let player_center = map.rooms[0].center();
    gs.ecs.insert(map);

    gs.ecs.create_entity()
        .with(Position { x: player_center.x, y: player_center.y })
        .with(Renderable { glyph: 10, fg: RGB::from_f32(1.0, 1.0, 1.0), bg: RGB::from_f32(0., 0., 0.) })
        .with(Player {})
        .with(Viewshed { visible_tiles: Vec::new(), range: 8, dirty: true })
        .build();

    main_loop(ctx, gs)
}
