use bracket_lib::prelude::*;
use specs::prelude::*;
use std::cmp::{max, min};
use super::*;


pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();

    let map = ecs.fetch::<Map>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map.tiles[destination_idx] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));
        }
    }
}


pub fn player_input(ecs: &mut World, ctx: &mut BTerm) {
    if let Some(key) = ctx.key {
        match key {
            VirtualKeyCode::Left | VirtualKeyCode::Numpad4 | VirtualKeyCode::H => try_move_player(-1, 0, ecs),
            VirtualKeyCode::Right | VirtualKeyCode::Numpad6 | VirtualKeyCode::L => try_move_player(1, 0, ecs),
            VirtualKeyCode::Up | VirtualKeyCode::Numpad8 | VirtualKeyCode::K => try_move_player(0, -1, ecs),
            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 | VirtualKeyCode::J => try_move_player(0, 1, ecs),
            _ => {}
        }
    }
}