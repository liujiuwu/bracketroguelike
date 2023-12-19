use super::prelude::*;
use crate::components::Position;
use bracket_lib::prelude::*;
use specs::prelude::*;

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    #[allow(clippy::type_complexity)]
    type SystemData = ( WriteExpect<'a, Map>,
                        ReadExpect<'a, Point>,
                        ReadExpect<'a, Entity>,
                        ReadExpect<'a, RunState>,
                        Entities<'a>,
                        WriteStorage<'a, Viewshed>,
                        ReadStorage<'a, Monster>,
                        WriteStorage<'a, Position>,
                        WriteStorage<'a, WantsToMelee>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, player_pos, player_entity, runstate, entities, mut viewshed, monster, mut position, mut wants_to_melee) = data;

        if *runstate != RunState::MonsterTurn { return; }

        for (entity, mut viewshed,_monster,mut pos) in (&entities, &mut viewshed, &monster, &mut position).join() {
            let distance = DistanceAlg::Pythagoras.distance2d(Point::new(pos.x, pos.y), *player_pos);
            if distance < 1.5 {
                wants_to_melee.insert(entity, WantsToMelee{ target: *player_entity }).expect("Unable to insert attack");
            } else if viewshed.visible_tiles.contains(&*player_pos) {
                let path = a_star_search(
                    map.point2d_to_index(Point::new(pos.x, pos.y)) as i32,
                    map.point2d_to_index(Point::new(player_pos.x, player_pos.y)) as i32,
                    &mut *map,
                );

                if path.success && path.steps.len() > 1 {
                    let mut idx = map.point2d_to_index(Point::new(pos.x, pos.y));
                    map.blocked[idx] = false;

                    let point = map.index_to_point2d(path.steps[1]);
                    (pos.x, pos.y) = (point.x, point.y);

                    idx = map.point2d_to_index(Point::new(pos.x, pos.y));
                    map.blocked[idx] = true;
                    viewshed.dirty = true;
                }
            }
        }
    }
}
