use bracket_lib::prelude::*;
use std::cmp::{max, min};

const MAX_ROOMS: i32 = 30;
const MIN_SIZE: i32 = 6;
const MAX_SIZE: i32 = 10;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

pub fn idx_xy(idx: usize) -> (i32, i32) {
    let x = (idx % 80) as i32;
    let y = (idx / 80) as i32;
    (x, y)
}

fn fill_floor(map: &mut [TileType], point: Point) {
    let idx = xy_idx(point.x, point.y);
    if idx > 0 && idx < 80 * 50 {
        map[idx as usize] = TileType::Floor;
    }
}

fn apply_room_to_map(room: &Rect, map: &mut [TileType]) {
    for y in room.y1 + 1..=room.y2 {
        for x in room.x1 + 1..=room.x2 {
            fill_floor(map, Point::new(x, y));
        }
    }
}

fn apply_horizontal_tunnel(map: &mut [TileType], x1: i32, x2: i32, y: i32) {
    for x in min(x1, x2)..=max(x1, x2) {
        fill_floor(map, Point::new(x, y));
    }
}

fn apply_vertical_tunnel(map: &mut [TileType], y1: i32, y2: i32, x: i32) {
    for y in min(y1, y2)..=max(y1, y2) {
        fill_floor(map, Point::new(x, y));
    }
}

pub fn new_map_rooms_and_corridors() -> (Vec<Rect>, Vec<TileType>) {
    let mut map = vec![TileType::Wall; 80 * 50];
    let mut rooms: Vec<Rect> = Vec::new();
    let mut rng = RandomNumberGenerator::new();

    for _ in 0..MAX_ROOMS {
        let w = rng.range(MIN_SIZE, MAX_SIZE);
        let h = rng.range(MIN_SIZE, MAX_SIZE);
        let x = rng.roll_dice(1, 79 - w) - 1;
        let y = rng.roll_dice(1, 49 - h) - 1;

        let new_room = Rect::with_size(x, y, w, h);
        let is_separate = rooms.iter().all(|room| !new_room.intersect(room));

        if is_separate {
            apply_room_to_map(&new_room, &mut map);

            if !rooms.is_empty() {
                let new_room_center = new_room.center();
                let prev_room_center = rooms[rooms.len() - 1].center();

                if rng.range(0, 2) == 1 {
                    apply_horizontal_tunnel(&mut map, prev_room_center.x, new_room_center.x, prev_room_center.y);
                    apply_vertical_tunnel(&mut map, prev_room_center.y, new_room_center.y, new_room_center.x);
                } else {
                    apply_vertical_tunnel(&mut map, prev_room_center.y, new_room_center.y, prev_room_center.x);
                    apply_horizontal_tunnel(&mut map, prev_room_center.x, new_room_center.x, new_room_center.y);
                }
            }

            rooms.push(new_room);
        }
    }

    (rooms, map)
}

pub fn draw_map(map: &[TileType], ctx: &mut BTerm) {
    for (idx, tile) in map.iter().enumerate() {
        let (x, y) = idx_xy(idx);
        match tile {
            TileType::Floor => ctx.set(x, y, GRAY30, BLACK, to_cp437('.')),
            TileType::Wall => ctx.set(x, y, GREEN, BLACK, to_cp437('#'))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{idx_xy, xy_idx};

    #[test]
    fn test_xy_idx() {
        let idx = xy_idx(40, 25);
        assert_eq!(2040, idx);
    }

    #[test]
    fn test_idx_xy() {
        let (x, y) = idx_xy(2040);
        assert_eq!((40, 25), (x, y));
    }
}