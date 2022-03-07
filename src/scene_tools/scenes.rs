use crate::player::*;
use raylib::prelude::*;

/* TRAITS */
//----------------------------------------------------------------
pub trait Scene {
    fn new(rl_handle: &mut RaylibHandle, rl_thread: &RaylibThread) -> Self;
    fn update(&mut self, rl_handle: &mut RaylibHandle, delta_time: f32);
    fn draw(&mut self, delta_time: f32, rl_draw: &mut RaylibDrawHandle);
}
//----------------------------------------------------------------

/* MENU SCENE */
//----------------------------------------------------------------
pub struct MenuScene {}

impl Scene for MenuScene {
    fn new(_rl_handle: &mut RaylibHandle, _rl_thread: &RaylibThread) -> Self {
        MenuScene {}
    }
    fn update(&mut self, _rl_handle: &mut RaylibHandle, _delta_time: f32) {
        // UPDATE
        println!("IN THE MENU");
    }
    fn draw(&mut self, _delta_time: f32, rl_draw: &mut RaylibDrawHandle) {
        // DRAW
        rl_draw.draw_text("MENU SCENE", 40, 40, 20, Color::PURPLE);
    }
}
//------------------------------------------------------------

// GAME SCENE
//------------------------------------------------------------
pub struct GameScene {
    player: Player,
}

impl Scene for GameScene {
    fn new(rl_handle: &mut RaylibHandle, rl_thread: &RaylibThread) -> Self {
        GameScene {
            player: Player::new(rl_handle, rl_thread),
        }
    }
    fn update(&mut self, rl_handle: &mut RaylibHandle, delta_time: f32) {
        // UPDATE
        self.player.update(rl_handle, delta_time);
    }
    fn draw(&mut self, delta_time: f32, rl_draw: &mut RaylibDrawHandle) {
        // DRAW
        self.player.draw(delta_time, rl_draw);
    }
}
//-----------------------------------------------------------
