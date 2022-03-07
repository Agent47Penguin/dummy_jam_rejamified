use crate::scene_tools::scenes::*;
use raylib::prelude::*;

pub enum SceneType {
    MainMenu,
    GameScene,
    //EndScene,
}

pub struct SceneManager {
    rl_handle: RaylibHandle,
    rl_thread: RaylibThread,
    current_scene: SceneType,
    menu_scene: MenuScene,
    game_scene: GameScene,
}

impl SceneManager {
    pub fn init() -> Self {
        // create rl handle and thread
        let (mut handle, thread) = raylib::init()
            .size(
                crate::SCREEN_WIDTH as i32 * crate::SCALE as i32,
                crate::SCREEN_HEIGHT as i32 * crate::SCALE as i32,
            )
            .title("Dummy Jam: ReJamified")
            .build();

        // create scenes with the rl handle and thread
        let m_scene = MenuScene::new(&mut handle, &thread);
        let g_scene = GameScene::new(&mut handle, &thread);

        // construct SceneManager and return it
        SceneManager {
            rl_handle: handle,
            rl_thread: thread,
            current_scene: SceneType::MainMenu,
            menu_scene: m_scene,
            game_scene: g_scene,
        }
    }
    fn change_scene(&mut self, new_scene: SceneType) {
        match new_scene {
            SceneType::MainMenu => self.current_scene = SceneType::MainMenu, // set to mani menu
            SceneType::GameScene => self.current_scene = SceneType::GameScene, // set to game scene
        }
    }
    fn restart_scene(&mut self) {
        match self.current_scene {
            SceneType::MainMenu => {
                self.menu_scene = MenuScene::new(&mut self.rl_handle, &self.rl_thread)
            }
            SceneType::GameScene => {
                self.game_scene = GameScene::new(&mut self.rl_handle, &self.rl_thread)
            }
        }
    }
    pub fn run(&mut self) {
        while !self.rl_handle.window_should_close() {
            //update
            //-----------------------------------------
            let d_time = self.rl_handle.get_frame_time();
            match self.current_scene {
                SceneType::MainMenu => self.menu_scene.update(&mut self.rl_handle, d_time),
                SceneType::GameScene => self.game_scene.update(&mut self.rl_handle, d_time),
            }
            //-----------------------------------------

            // scene management
            //------------------------------------------

            /* next scene */
            if self.rl_handle.is_key_pressed(KeyboardKey::KEY_N) {
                self.change_scene(SceneType::GameScene);
            }

            /* restart scene */
            if self.rl_handle.is_key_pressed(KeyboardKey::KEY_R) {
                self.restart_scene();
            }

            /* previous scene */
            if self.rl_handle.is_key_pressed(KeyboardKey::KEY_P) {
                self.change_scene(SceneType::MainMenu)
            }

            //-----------------------------------------

            // draw
            //-----------------------------------------
            let mut d = self.rl_handle.begin_drawing(&self.rl_thread);
            d.clear_background(Color::BLACK);

            match self.current_scene {
                SceneType::MainMenu => self.menu_scene.draw(d_time, &mut d),
                SceneType::GameScene => self.game_scene.draw(d_time, &mut d),
            }
            //-----------------------------------------
        }
    }
}
