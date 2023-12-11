use bracket_lib::prelude::*;
use std::cmp::{max, min};
use specs::*;
use crate::{Player, Position};

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
    fn fill_floor(&mut self, point: Point) {
        let idx = self.point2d_to_index(point);
        if idx > 0 && idx < (self.width * self.height) as usize {
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
        const WIDTH: i32 = 80;
        const HEIGHT: i32 = 50;
        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        let mut map = Map {
            tiles: vec![TileType::Wall; (WIDTH * HEIGHT) as usize],
            rooms: Vec::new(),
            width: WIDTH,
            height: HEIGHT,
            revealed_tiles: vec![false; (WIDTH * HEIGHT) as usize],
            visible_tiles: vec![false; (WIDTH * HEIGHT) as usize],
        };

        let mut rng = RandomNumberGenerator::new();
        for _ in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, WIDTH - 1 - w) - 1;
            let y = rng.roll_dice(1, HEIGHT - 1 - h) - 1;

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

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] == TileType::Wall
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}


pub fn draw_map(ecs: &World, draw_batch: &mut DrawBatch) {
    draw_batch.target(0);
    draw_batch.cls();

    let map = ecs.fetch::<Map>();
    let positions = ecs.read_storage::<Position>();
    let players = ecs.read_storage::<Player>();

    for (idx, tile) in map.tiles.iter().enumerate() {
        if map.revealed_tiles[idx] {
            let map_point = map.index_to_point2d(idx);
            let mut fg = RGB::from_f32(1.0, 1.0, 1.0);
            let glyph = match tile {
                TileType::Floor => 0,
                TileType::Wall => 1
            };

            if !map.visible_tiles[idx] {
                fg = fg * 0.3;
            } else {
                for (pos, _) in (&positions, &players).join() {
                    let distance = 1.0 - (DistanceAlg::Pythagoras.distance2d(map_point, Point::new(pos.x, pos.y)) as f32 / 10.0);
                    fg = RGB::from_f32(distance, distance, distance);
                }
            }

            draw_batch.set(map_point, ColorPair::new(fg, BLACK), glyph);
        }
    }
}