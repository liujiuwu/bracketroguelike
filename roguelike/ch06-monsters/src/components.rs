use bracket_lib::prelude::*;
use specs::{Component, prelude::*};

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Component, Debug)]
pub struct Player;

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles : Vec<Point>,
    pub range : i32,
    pub dirty : bool
}


#[derive(Component, Debug)]
pub struct Monster {}

#[derive(Component, Debug)]
pub struct Name {
    pub name : String
}