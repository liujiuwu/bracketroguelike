use bracket_lib::prelude::*;
use specs::prelude::*;
use super::prelude::{Viewshed, Name, Monster};

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = ( ReadExpect<'a, Point>, ReadStorage<'a, Viewshed>, ReadStorage<'a, Monster>, ReadStorage<'a, Name>);

    fn run(&mut self, data : Self::SystemData) {
        let (player_pos, viewshed, monster,name) = data;

        for (viewshed,_monster,name) in (&viewshed, &monster,&name).join() {
            if viewshed.visible_tiles.contains(&*player_pos) {
                log(&format!("{} 看见你啦！！", name.name));
            }
        }
    }
}
