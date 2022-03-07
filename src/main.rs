//use raylib::prelude::*;
mod animation;
mod player;
mod scene_tools;

use scene_tools::scene_manager::*;

pub const SCREEN_WIDTH: u16 = 320; // 240
pub const SCREEN_HEIGHT: u16 = 180; // 135
pub const SCALE: f32 = 6.0;

fn main() {
    let mut window = SceneManager::init();

    window.run();
}
