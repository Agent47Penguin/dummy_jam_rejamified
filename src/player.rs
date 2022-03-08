use crate::animation::*;
use raylib::prelude::*;

pub struct Player {
    texture: Texture2D,
    position: Vector2,
    move_distance: u16,
    next_position: u32,
    is_moving: bool,
    anim_data: AnimationData,
}

impl Player {
    pub fn new(rl_handle: &mut RaylibHandle, rl_thread: &RaylibThread) -> Self {
        let p_texture = rl_handle
            .load_texture(rl_thread, "res/player_sheet1.png")
            .unwrap();
        let h = p_texture.height() as f32;
        let w = p_texture.width() as f32 / 7.0;
        Player {
            texture: rl_handle
                .load_texture(rl_thread, "res/player_sheet1.png")
                .unwrap(), // load player texture
            position: Vector2::new(
                w / 2.0 * crate::SCALE,
                ((crate::SCREEN_HEIGHT as f32 / 2.0) - (h / 2.0)) * crate::SCALE,
            ), // set player postion to left side, middle of screen
            move_distance: ((crate::SCREEN_HEIGHT as f32 / 2.0 - h / 2.0) * crate::SCALE) as u16,
            next_position: 0,
            is_moving: false,
            anim_data: AnimationData::new(p_texture, 32),
        }
    }
    pub fn draw(&mut self, delta_time: f32, draw_handle: &mut RaylibDrawHandle) {
        let dest_rec = Rectangle::new(
            self.position.x,
            self.position.y,
            32.0 * crate::SCALE,
            32.0 * crate::SCALE,
        );
        if !self.is_moving {
            draw_handle.draw_texture_pro(
                &self.texture,
                self.anim_data.animate(delta_time),
                dest_rec,
                Vector2::new(0.0, 0.0),
                0.0,
                Color::WHITE,
            );
        } else {
            // moving down
            draw_handle.draw_texture_pro(
                &self.texture,
                self.anim_data
                    .animate_move(self.position.y, self.next_position as f32),
                dest_rec,
                Vector2::new(0.0, 0.0),
                0.0,
                Color::WHITE,
            );
        }
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
