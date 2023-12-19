use bracket_lib::prelude::*;
use specs::prelude::*;
use super::prelude::*;

pub struct MapIndexingSystem {}

impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = (WriteExpect<'a, Map>, ReadStorage<'a, Position>, ReadStorage<'a, BlocksTile>, Entities<'a>, );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, position, blockers, entities) = data;

        map.populate_blocked();
        map.clear_content_index();
        for (entity, position) in (&entities, &position).join() {
            let idx = map.point2d_to_index(Point::new(position.x, position.y));

            // If they block, update the blocking list
            if blockers.get(entity).is_some() {
                map.blocked[idx] = true;
            }
            map.tile_content[idx].push(entity);
        }
    }
}