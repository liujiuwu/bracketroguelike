use bracket_lib::prelude::*;
use specs::prelude::*;
use std::cmp::{max, min};
use super::prelude::*;


pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut renderables = ecs.write_storage::<Renderable>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let map = ecs.fetch::<Map>();

    for (_player, pos, viewshed, _renderable) in (&mut players, &mut positions, &mut viewsheds, &mut renderables).join() {
        let destination_idx = map.point2d_to_index(Point::new(pos.x + delta_x, pos.y + delta_y));
        if map.tiles[destination_idx] != TileType::Wall {
            pos.x = min(map.width - 1, max(0, pos.x + delta_x));
            pos.y = min(map.height - 1, max(0, pos.y + delta_y));
            viewshed.dirty = true;

            let mut ppos = ecs.write_resource::<Point>();
            ppos.x = pos.x;
            ppos.y = pos.y;
        }
    }
}


pub fn player_input(ecs: &mut World, ctx: &mut BTerm) -> RunState {
    match ctx.key {
        None => return RunState::Paused,
        Some(key) => match key {
            VirtualKeyCode::Left | VirtualKeyCode::Numpad4 | VirtualKeyCode::H => try_move_player(-1, 0, ecs),
            VirtualKeyCode::Right | VirtualKeyCode::Numpad6 | VirtualKeyCode::L => try_move_player(1, 0, ecs),
            VirtualKeyCode::Up | VirtualKeyCode::Numpad8 | VirtualKeyCode::K => try_move_player(0, -1, ecs),
            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 | VirtualKeyCode::J => try_move_player(0, 1, ecs),
            VirtualKeyCode::S => ctx.screenshot("roguelike.png"),
            _ => return RunState::Paused
        }
    }
    RunState::Running
}