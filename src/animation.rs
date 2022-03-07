use raylib::prelude::*;

pub struct AnimationData {
    current_frame: u8,
    max_frames: u8,
    next_frame: Vector2,
    frame_rect: Rectangle,
    running_time: f32,
    update_time: f32,
}

impl AnimationData {
    pub fn new(spritesheet: Texture2D, sprite_size: i32) -> Self {
        AnimationData {
            current_frame: 0,
            max_frames: 0,
            next_frame: Vector2::new((spritesheet.width() / sprite_size) as f32, 0.0),
            frame_rect: Rectangle::new(0.0, 0.0, 0.0, spritesheet.height() as f32),
            running_time: 0.0,
            update_time: 1.0 / 12.0,
        }
    }
    pub fn update(&mut self, delta_time: f32) -> Rectangle {
        self.running_time += delta_time;

        if self.running_time >= self.update_time {
            // reset running time
            self.running_time = 0.0;

            // precede to next frame
            self.current_frame += 1;

            // update src rect to reflect next frame
            self.frame_rect.x = self.current_frame as f32 * self.next_frame.x;

            // reset current frame if larger or equal to max frames
            if self.current_frame >= self.max_frames {
                self.current_frame = 0
            }
        }

        self.frame_rect
    }
}
