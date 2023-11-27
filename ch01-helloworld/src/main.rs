use bracket_lib::prelude::*;

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(25, "Hello world!");
    }
}

fn main() -> BError {
    let ctx = BTermBuilder::simple80x50().build()?;

    let gs = State {};
    main_loop(ctx, gs)
}