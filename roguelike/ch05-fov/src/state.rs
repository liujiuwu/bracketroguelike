use bracket_lib::prelude::*;
use specs::prelude::*;
use super::prelude::*;

pub struct State {
    pub ecs: World,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        player_input(&mut self.ecs, ctx);
        self.run_systems();

        draw_map(&self.ecs,ctx);

        ctx.set_active_console(1);
        ctx.cls();
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }

        ctx.set_active_console(2);
        ctx.cls();
        ctx.set_translation_mode(2, CharacterTranslationMode::Unicode);
        ctx.print_centered(1, "Rust地下城与勇士");
    }
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
        self.ecs.maintain();
    }
}