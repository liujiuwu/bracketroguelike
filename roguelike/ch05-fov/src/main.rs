use bracket_lib::prelude::*;
use specs::prelude::*;
use roguelike::*;

embedded_resource!(TILE_FONT, "../resources/roguelike_tiles.png");

fn main() -> BError {
    link_resource!(TILE_FONT, "resources/roguelike_tiles.png");

    let map = Map::new_map_rooms_and_corridors();

    let ctx = BTermBuilder::new()
        .with_dimensions(map.width as u32, map.height as u32)
        .with_tile_dimensions(16u32, 16u32)
        .with_title("Walk fov")
        .with_font("roguelike_tiles.png", 16u32, 16u32)
        .with_simple_console(map.width as u32, map.height as u32, "roguelike_tiles.png")
        .with_sparse_console_no_bg(map.width as u32, map.height as u32, "roguelike_tiles.png")
        .build()?;

    let mut gs = State { ecs: World::new() };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();

    let player_center = map.rooms[0].center();
    gs.ecs.insert(map);

    gs.ecs.create_entity()
        .with(Position { x: player_center.x, y: player_center.y })
        .with(Renderable { glyph: 2, fg: RGB::named(YELLOW), bg: RGB::named(BLACK) })
        .with(Player {})
        .with(Viewshed { visible_tiles: Vec::new(), range: 8, dirty: true })
        .build();

    main_loop(ctx, gs)
}
