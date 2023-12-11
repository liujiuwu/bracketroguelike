use bracket_lib::prelude::*;
use std::cmp::{max, min};
use specs::World;

const MAX_ROOMS: i32 = 30;
const MIN_SIZE: i32 = 6;
const MAX_SIZE: i32 = 10;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Default)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * 80) + x as usize
    }

    pub fn idx_xy(&self, idx: usize) -> (i32, i32) {
        let x = (idx % 80) as i32;
        let y = (idx / 80) as i32;
        (x, y)
    }

    fn fill_floor(&mut self, point: Point) {
        let idx = self.xy_idx(point.x, point.y);
        if idx > 0 && idx < 80 * 50 {
            self.tiles[idx as usize] = TileType::Floor;
        }
    }

    fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                self.fill_floor(Point::new(x, y));
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            self.fill_floor(Point::new(x, y));
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            self.fill_floor(Point::new(x, y));
        }
    }

    pub fn new_map_rooms_and_corridors() -> Self {
        let mut map = Map {
            tiles: vec![TileType::Wall; 80 * 50],
            rooms: Vec::new(),
            width: 80,
            height: 50,
            revealed_tiles: vec![false; 80 * 50],
            visible_tiles: vec![false; 80 * 50],
        };

        let mut rng = RandomNumberGenerator::new();

        for _ in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, 79 - w) - 1;
            let y = rng.roll_dice(1, 49 - h) - 1;

            let new_room = Rect::with_size(x, y, w, h);
            let is_separate = map.rooms.iter().all(|room| !new_room.intersect(room));

            if is_separate {
                map.apply_room_to_map(&new_room);

                if !map.rooms.is_empty() {
                    let new_room_center = new_room.center();
                    let prev_room_center = map.rooms[map.rooms.len() - 1].center();

                    if rng.range(0, 2) == 1 {
                        map.apply_horizontal_tunnel(prev_room_center.x, new_room_center.x, prev_room_center.y);
                        map.apply_vertical_tunnel(prev_room_center.y, new_room_center.y, new_room_center.x);
                    } else {
                        map.apply_vertical_tunnel(prev_room_center.y, new_room_center.y, prev_room_center.x);
                        map.apply_horizontal_tunnel(prev_room_center.x, new_room_center.x, new_room_center.y);
                    }
                }

                map.rooms.push(new_room);
            }
        }

        map
    }
}

pub fn draw_map(ecs: &World, ctx: &mut BTerm) {
    let map = ecs.fetch::<Map>();
    for (idx, tile) in map.tiles.iter().enumerate() {
        let (x, y) = map.idx_xy(idx);
        match tile {
            TileType::Floor => ctx.set(x, y, RGB::from_f32(0.0, 0.5, 0.5), BLACK, to_cp437('.')),
            TileType::Wall => ctx.set(x, y, RGB::from_f32(0., 1.0, 0.), BLACK, to_cp437('#'))
        }
    }
}