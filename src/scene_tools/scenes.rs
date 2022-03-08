use crate::player::*;
use crate::scene_tools::parallax::*;
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
    fn update(&mut self, _rl_handle: &mut RaylibHandle, _delta_time: f32) {}
    fn draw(&mut self, _delta_time: f32, rl_draw: &mut RaylibDrawHandle) {
        // DRAW
        rl_draw.draw_text("MENU SCENE", 40, 40, 20, Color::PURPLE);
        rl_draw.draw_text("PRESS N TO CONTINUE", 40, 60, 20, Color::RED);
    }
}
//------------------------------------------------------------

// GAME SCENE
//------------------------------------------------------------
pub struct GameScene {
    player: Player,
    parallaxed_bg_1: ParallaxedBackground,
    parallaxed_bg_2: ParallaxedBackground,
}

impl Scene for GameScene {
    fn new(rl_handle: &mut RaylibHandle, rl_thread: &RaylibThread) -> Self {
        GameScene {
            player: Player::new(rl_handle, rl_thread),
            parallaxed_bg_1: ParallaxedBackground::new(
                rl_handle
                    .load_texture(rl_thread, "res/stars_sheet1.png")
                    .unwrap(),
                0.0,
            ),
            parallaxed_bg_2: ParallaxedBackground::new(
                rl_handle
                    .load_texture(rl_thread, "res/stars_sheet2.png")
                    .unwrap(),
                200.0,
            ),
        }
    }
    fn update(&mut self, rl_handle: &mut RaylibHandle, delta_time: f32) {
        // UPDATE PLAYER
        self.player.update(rl_handle, delta_time);

        // UPDATE PARALLAXED BACKGROUNDS
        self.parallaxed_bg_1.update(100.0, delta_time);
        self.parallaxed_bg_2.update(150.0, delta_time);
    }
    fn draw(&mut self, delta_time: f32, rl_draw: &mut RaylibDrawHandle) {
        // DRAW PARALLAXED BACKGROUNDS
        self.parallaxed_bg_1.draw(rl_draw);
        self.parallaxed_bg_2.draw(rl_draw);

        // DRAW PLAYER
        self.player.draw(delta_time, rl_draw);
    }
}
//-----------------------------------------------------------
