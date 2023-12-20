use super::prelude::*;
use bracket_lib::prelude::*;
use specs::*;
use std::cmp::{max, min};

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
    pub blocked: Vec<bool>,
    pub tile_content: Vec<Vec<Entity>>,
}

impl Map {
    fn is_exit_valid(&self, x: i32, y: i32) -> bool {
        if x < 1 || x > self.width - 1 || y < 1 || y > self.height - 1 { return false; }
        let idx = self.point2d_to_index(Point::new(x, y));
        !self.blocked[idx]
    }

    fn fill_floor(&mut self, point: Point) {
        let idx = self.point2d_to_index(point);
        if idx > 0 && idx < (self.width * self.height) as usize {
            self.tiles[idx] = TileType::Floor;
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

    pub fn populate_blocked(&mut self) {
        for (i, tile) in self.tiles.iter_mut().enumerate() {
            self.blocked[i] = *tile == TileType::Wall;
        }
    }

    pub fn clear_content_index(&mut self) {
        for content in self.tile_content.iter_mut() {
            content.clear();
        }
    }

    pub fn new_map_rooms_and_corridors(width: i32, height: i32) -> Self {
        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        let vec_size = (width * height) as usize;
        let mut map = Map {
            tiles: vec![TileType::Wall; vec_size],
            rooms: Vec::new(),
            width,
            height,
            revealed_tiles: vec![false; vec_size],
            visible_tiles: vec![false; vec_size],
            blocked: vec![false; vec_size],
            tile_content: vec![Vec::new(); vec_size],
        };

        let mut rng = RandomNumberGenerator::new();
        for _ in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, map.width - 1 - w) - 1;
            let y = rng.roll_dice(1, map.height - 1 - h) - 1;

            let new_room = Rect::with_size(x, y, w, h);
            let is_separate = map.rooms.iter().all(|room| !new_room.intersect(room));

            if is_separate {
                map.apply_room_to_map(&new_room);

                if !map.rooms.is_empty() {
                    let new_room_center = new_room.center();
                    let prev_room_center = map.rooms[map.rooms.len() - 1].center();

                    if rng.range(0, 2) == 1 {
                        map.apply_horizontal_tunnel(
                            prev_room_center.x,
                            new_room_center.x,
                            prev_room_center.y,
                        );
                        map.apply_vertical_tunnel(
                            prev_room_center.y,
                            new_room_center.y,
                            new_room_center.x,
                        );
                    } else {
                        map.apply_vertical_tunnel(
                            prev_room_center.y,
                            new_room_center.y,
                            prev_room_center.x,
                        );
                        map.apply_horizontal_tunnel(
                            prev_room_center.x,
                            new_room_center.x,
                            new_room_center.y,
                        );
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

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let p1 = self.index_to_point2d(idx1);
        let p2 = self.index_to_point2d(idx2);
        DistanceAlg::Pythagoras.distance2d(p1, p2)
    }

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let point = self.index_to_point2d(idx);
        let (x, y) = (point.x, point.y);
        let w = self.width as usize;

        // Cardinal directions
        if self.is_exit_valid(x - 1, y) { exits.push((idx - 1, 1.0)) };
        if self.is_exit_valid(x + 1, y) { exits.push((idx + 1, 1.0)) };
        if self.is_exit_valid(x, y - 1) { exits.push((idx - w, 1.0)) };
        if self.is_exit_valid(x, y + 1) { exits.push((idx + w, 1.0)) };

        // Diagonals
        if self.is_exit_valid(x - 1, y - 1) { exits.push(((idx - w) - 1, 1.45)); }
        if self.is_exit_valid(x + 1, y - 1) { exits.push(((idx - w) + 1, 1.45)); }
        if self.is_exit_valid(x - 1, y + 1) { exits.push(((idx + w) - 1, 1.45)); }
        if self.is_exit_valid(x + 1, y + 1) { exits.push(((idx + w) + 1, 1.45)); }

        exits
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

pub fn draw_map(ecs: &World, ctx: &mut BTerm) {
    let mut draw_batch = DrawBatch::new();
    let map = ecs.fetch::<Map>();
    let positions = ecs.read_storage::<Position>();
    let players = ecs.read_storage::<Player>();

    for (idx, tile) in map.tiles.iter().enumerate() {
        let map_point = map.index_to_point2d(idx);
        if map.revealed_tiles[idx] {
            let (glyph, mut fg) = match tile {
                TileType::Floor => (to_cp437('.'), RGB::named(GRAY).to_rgba(1.0)),
                TileType::Wall => (to_cp437('#'), RGB::named(GREEN).to_rgba(1.0)),
            };

            if !map.visible_tiles[idx] {
                fg = RGB::named(GRAY).to_rgba(0.5);
            } else {
                for (pos, _) in (&positions, &players).join() {
                    let distance = 1.0
                        - (DistanceAlg::Pythagoras.distance2d(map_point, Point::new(pos.x, pos.y))
                        / 10.0);
                    fg = fg * distance;
                }
            }

            draw_batch.set(map_point, ColorPair::new(fg, BLACK), glyph);
        }
    }

    draw_batch.submit(0).expect("Batch error");
    render_draw_buffer(ctx).expect("Render error");
}
