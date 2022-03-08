use raylib::prelude::*;

pub struct ParallaxedBackground {
    background: Texture2D,
    bg_pos_1: f32,
    bg_pos_2: f32,
    bg_pos_3: f32,
}

impl ParallaxedBackground {
    pub fn new(bg_texture: Texture2D, initial_pos: f32) -> Self {
        let width = bg_texture.width();
        ParallaxedBackground {
            background: bg_texture,
            bg_pos_1: initial_pos,
            bg_pos_2: (width as f32 * crate::SCALE) + initial_pos,
            bg_pos_3: ((width * 2) as f32 * crate::SCALE) + initial_pos,
        }
    }
    pub fn update(&mut self, move_speed: f32, delta_time: f32) {
        // UPDATE 1ST BACKGROUND POSITION
        if self.bg_pos_1 > -1.0 * self.background.width() as f32 * crate::SCALE {
            self.bg_pos_1 -= move_speed * delta_time;
        } else {
            // RESET IF OFFSCREEN
            self.bg_pos_1 = (self.background.width * 2) as f32 * crate::SCALE;
        }

        // UPDATE 2ND BACKGROUND POSITION
        if self.bg_pos_2 > -1.0 * self.background.width() as f32 * crate::SCALE {
            self.bg_pos_2 -= move_speed * delta_time;
        } else {
            // RESET IF OFFSCREEN
            self.bg_pos_2 = (self.background.width * 2) as f32 * crate::SCALE;
        }

        // UPDATE 3RD BACKGROUND POSITION
        if self.bg_pos_3 > -1.0 * self.background.width() as f32 * crate::SCALE {
            self.bg_pos_3 -= move_speed * delta_time;
        } else {
            // RESET IF OFFSCREEN
            self.bg_pos_3 = (self.background.width * 2) as f32 * crate::SCALE;
        }
    }
    pub fn draw(&mut self, draw_handle: &mut RaylibDrawHandle) {
        // DRAW 1ST BACKGROUND
        draw_handle.draw_texture_ex(
            &self.background,
            Vector2::new(self.bg_pos_1, 0.0),
            0.0,
            crate::SCALE,
            Color::WHITE,
        );

        // DRAW 2ND BACKGROUND
        draw_handle.draw_texture_ex(
            &self.background,
            Vector2::new(self.bg_pos_2, 0.0),
            0.0,
            crate::SCALE,
            Color::WHITE,
        );

        // DRAW 3RD BACKGROUND
        draw_handle.draw_texture_ex(
            &self.background,
            Vector2::new(self.bg_pos_3, 0.0),
            0.0,
            crate::SCALE,
            Color::WHITE,
        );
    }
}
