use player::*;
use raylib::prelude::*;

mod player;

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

    let mut player = Player::new(&mut rl, &thread);

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        // UPDATE
        //let d_time = rl.get_frame_time();
        player.update(&mut rl);

        // DRAW
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        player.draw(&mut d);
    }
}
