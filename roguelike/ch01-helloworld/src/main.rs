use bracket_lib::prelude::*;

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        for i in 1..=5 {
            ctx.draw_box(25 + i, 15 + i, 30 - i * 2, 20 - i * 2, WHITE, BLACK);
        }
        ctx.print_color_centered(25, YELLOW, BLACK, "Hello world!");
    }
}

fn main() -> BError {
    let ctx = BTermBuilder::simple80x50()
        .with_title("Roguelike game")
        .build()?;

    let gs = State {};
    main_loop(ctx, gs)
}