use raylib::prelude::*;

pub struct Player {
    texture: Texture2D,
    position: Vector2,
    move_distance: u16,
    next_position: u32,
    is_moving: bool,
}

impl Player {
    pub fn new(rl_handle: &mut RaylibHandle, rl_thread: &RaylibThread) -> Self {
        Player {
            texture: rl_handle.load_texture(rl_thread, "res/player.png").unwrap(), // load player texture
            position: Vector2::new(
                rl_handle
                    .load_texture(rl_thread, "res/player.png")
                    .unwrap()
                    .width() as f32
                    / 2.0
                    * crate::SCALE,
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
            next_position: 0,
            is_moving: false,
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
    pub fn update(&mut self, rl_handle: &mut RaylibHandle, delta_time: f32) {
        self.process_movement(rl_handle, delta_time);
    }
    fn process_movement(&mut self, rl_handle: &mut RaylibHandle, delta_time: f32) {
        if !self.is_moving {
            self.next_position = self.move_pos(rl_handle);
        } else if self.is_moving {
            if self.next_position > self.position.y as u32 {
                self.position.y += self.move_distance as f32 / 0.25 * delta_time;

                if self.next_position < self.position.y as u32 {
                    self.position.y = self.next_position as f32;
                }
            } else if self.next_position < self.position.y as u32 {
                self.position.y -= self.move_distance as f32 / 0.25 * delta_time;

                if self.next_position > self.position.y as u32 {
                    self.position.y = self.next_position as f32;
                }
            } else if self.next_position == self.position.y as u32 {
                self.is_moving = false;
            }
        }
    }
    fn move_pos(&mut self, rl_handle: &mut RaylibHandle) -> u32 {
        if (rl_handle.is_key_pressed(KeyboardKey::KEY_UP)
            || rl_handle.is_key_pressed(KeyboardKey::KEY_W))
            && !self.is_moving
        {
            self.is_moving = true;

            return (self.position.y - self.move_distance as f32) as u32;
        }
        if (rl_handle.is_key_pressed(KeyboardKey::KEY_DOWN)
            || rl_handle.is_key_pressed(KeyboardKey::KEY_S))
            && !self.is_moving
        {
            self.is_moving = true;

            if (self.position.y as u32 + self.move_distance as u32)
                < ((crate::SCREEN_HEIGHT as u32) - (self.texture.height() as u32 / 2))
                    * crate::SCALE as u32
            {
                return (self.position.y + self.move_distance as f32) as u32;
            } else {
                return self.position.y as u32;
            }
        }

        0
    }
}
