use crate::game::GameState;
use raylib::audio::RaylibAudio;
use raylib::drawing::RaylibDraw;

mod game;
mod render;
mod update;

struct WindowConfig {
    height: i32,
    width: i32,
    name: &'static str,
    pos: (i32, i32),
}

impl WindowConfig {
    fn new() -> Self {
        Self {
            height: 480,
            width: 640,
            name: "Game",
            pos: (2000, 500),
        }
    }
}

fn main() {
    let window_config = WindowConfig::new();
    let game_config = game::GameConfig::new(&window_config);
    let mut game_state = GameState::new(&game_config);

    let (mut rl, thread) = raylib::init()
        .size(window_config.width, window_config.height)
        .title(window_config.name)
        .build();

    let audio_handle = RaylibAudio::init_audio_device().expect("Failed to init audio");
    let hit_sound = audio_handle
        .new_sound("src/resources/blip.wav")
        .expect("Failed to load sound");
    let miss_sound = audio_handle
        .new_sound("src/resources/miss.wav")
        .expect("Failed to load sound");

    rl.set_window_position(window_config.pos.0, window_config.pos.1);
    rl.set_target_fps(game_config.fps);

    while !rl.window_should_close() {
        update::update(&game_config, &mut game_state, &rl);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(game_config.background);
        render::render(&game_config, &game_state, &mut d);

        if game_state.hit {
            hit_sound.play();
            game_state.hit = false;
        } else if game_state.miss {
            miss_sound.play();
            game_state.miss = false;
        }
    }
}
