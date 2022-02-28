use crate::scene::*;
use raylib::prelude::*;

enum SceneType {
    MainMenu,
    GameScene,
    EndScene,
}

pub struct SceneManager {
    current_scene: SceneType,
}

impl SceneManager {
    pub fn new() -> Self {
        SceneManager {
            current_scene: SceneType::GameScene,
        }
    }
    pub fn run(&mut self, rl_handle: &mut RaylibHandle, rl_thread: &RaylibThread) {
        // set current scene
        let mut scene = match self.current_scene {
            SceneType::MainMenu => GameScene::new(rl_handle, rl_thread),
            SceneType::GameScene => GameScene::new(rl_handle, rl_thread),
            SceneType::EndScene => GameScene::new(rl_handle, rl_thread),
        };

        while !rl_handle.window_should_close() {
            // change scene
            if rl_handle.is_key_pressed(KeyboardKey::KEY_R) {
                scene = match self.current_scene {
                    SceneType::MainMenu => GameScene::new(rl_handle, rl_thread),
                    SceneType::GameScene => GameScene::new(rl_handle, rl_thread),
                    SceneType::EndScene => GameScene::new(rl_handle, rl_thread),
                };
            }

            //update
            let d_time = rl_handle.get_frame_time();
            scene.update(rl_handle, d_time);

            // draw
            let mut d = rl_handle.begin_drawing(&rl_thread);
            d.clear_background(Color::BLACK);
            scene.draw(&mut d);
        }
    }
}
