use bracket_lib::prelude::*;
use specs::prelude::*;
use roguelike::prelude::*;

const WIDTH: i32 = 80;
const HEIGHT: i32 = 50;

fn main() -> BError {
    let ctx = BTermBuilder::simple80x50()
        .with_tile_dimensions(16, 16)
        .with_title("Walk monster")
        .build()?;

    let mut gs = State {
        ecs: World::new()
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<BlocksTile>();
    gs.ecs.register::<CombatStats>();
    gs.ecs.register::<WantsToMelee>();
    gs.ecs.register::<SufferDamage>();

    let map = Map::new_map_rooms_and_corridors(WIDTH, HEIGHT);
    let player_center = map.rooms[0].center();
    let player_entity = gs.ecs.create_entity()
        .with(Position { x: player_center.x, y: player_center.y })
        .with(Renderable { glyph: to_cp437('@'), fg: RGB::named(WHITE), bg: RGB::named(BLACK) })
        .with(Viewshed { visible_tiles: Vec::new(), range: 8, dirty: true })
        .with(Player {})
        .with(Name { name: "Rust勇士".to_string() })
        .with(CombatStats{ max_hp: 30, hp: 30, defense: 2, power: 5 })
        .build();

    let mut rng = RandomNumberGenerator::new();
    for (id, room) in map.rooms.iter().skip(1).enumerate() {
        let roll = rng.roll_dice(1, 2);
        let (glyph, name) = match roll {
            1 => (to_cp437('g'), "小G".to_string()),
            _ => (to_cp437('o'), "小O".to_string())
        };

        let pos = room.center();
        gs.ecs.create_entity()
            .with(Position { x: pos.x, y: pos.y })
            .with(Renderable { glyph, fg: RGB::named(RED), bg: RGB::named(BLACK) })
            .with(Viewshed { visible_tiles: Vec::new(), range: 8, dirty: true })
            .with(Monster {})
            .with(Name { name: format!("Room{}_{}", id, &name) })
            .with(BlocksTile{})
            .with(CombatStats{ max_hp: 16, hp: 16, defense: 1, power: 4 })
            .build();
    }


    gs.ecs.insert(map);
    gs.ecs.insert(Point::new(player_center.x, player_center.y));
    gs.ecs.insert(player_entity);
    gs.ecs.insert(RunState::PreRun);

    main_loop(ctx, gs)
}
