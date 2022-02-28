use crate::player::*;
use raylib::prelude::*;

pub struct GameScene {
    player: Player,
}

impl GameScene {
    pub fn new(rl_handle: &mut RaylibHandle, rl_thread: &RaylibThread) -> Self {
        GameScene {
            player: Player::new(rl_handle, rl_thread),
        }
    }
    pub fn update(&mut self, rl_handle: &mut RaylibHandle, delta_time: f32) {
        // UPDATE
        self.player.update(rl_handle, delta_time);
    }
    pub fn draw(&mut self, rl_draw: &mut RaylibDrawHandle) {
        // DRAW
        self.player.draw(rl_draw);
    }
}
