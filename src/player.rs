use raylib::prelude::*;

pub struct Player {
    texture: Texture2D,
    position: Vector2,
    move_distance: u16,
}

impl Player {
    pub fn new(rl_handle: &mut RaylibHandle, rl_thread: &RaylibThread) -> Self {
        Player {
            texture: rl_handle.load_texture(rl_thread, "res/player.png").unwrap(), // load player texture
            position: Vector2::new(
                0.0,
                ((crate::SCREEN_HEIGHT as f32 / 2.0)
                    - (rl_handle
                        .load_texture(rl_thread, "res/player.png")
                        .unwrap()
                        .height() as f32
                        / 2.0))
                    * crate::SCALE,
            ), // set player postion to left side, middle of screen
            move_distance: ((crate::SCREEN_HEIGHT as f32 / 2.0
                - rl_handle
                    .load_texture(rl_thread, "res/player.png")
                    .unwrap()
                    .height() as f32
                    / 2.0)
                * crate::SCALE) as u16,
        }
    }
    pub fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        draw_handle.draw_texture_ex(
            &self.texture,
            &self.position,
            0.0,
            crate::SCALE,
            Color::WHITE,
        );
    }
    pub fn update(&mut self, rl_handle: &mut RaylibHandle) {
        if rl_handle.is_key_pressed(KeyboardKey::KEY_W)
            || rl_handle.is_key_pressed(KeyboardKey::KEY_UP)
        {
            self.move_up()
        }

        if rl_handle.is_key_pressed(KeyboardKey::KEY_S)
            || rl_handle.is_key_pressed(KeyboardKey::KEY_DOWN)
        {
            self.move_down()
        }
    }
    fn move_up(&mut self) {
        if self.position.y != 0.0 {
            self.position.y -= self.move_distance as f32;
        }
    }
    fn move_down(&mut self) {
        if self.position.y
            != (crate::SCREEN_HEIGHT as f32 - self.texture.height() as f32) * crate::SCALE
        {
            self.position.y += self.move_distance as f32;
        }
    }
}
