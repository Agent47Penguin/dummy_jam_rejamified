//use raylib::prelude::*;
use scene_manager::*;

mod player;
mod scene;
mod scene_manager;

pub const SCREEN_WIDTH: u16 = 320; // 240
pub const SCREEN_HEIGHT: u16 = 180; // 135
pub const SCALE: f32 = 6.0;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(
            SCREEN_WIDTH as i32 * SCALE as i32,
            SCREEN_HEIGHT as i32 * SCALE as i32,
        )
        .title("Dummy Jam: ReJamified")
        .build();

    let mut scene_manager = SceneManager::new();

    rl.set_target_fps(60);
    scene_manager.run(&mut rl, &thread);
}
