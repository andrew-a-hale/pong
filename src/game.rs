use crate::WindowConfig;

use raylib::ffi::{Color, GetRandomValue, Rectangle};

pub fn get_random_value() -> i32 {
    unsafe { 2 * GetRandomValue(0, 1) - 1 }
}

pub struct GameConfig {
    pub game_height: i32,
    pub game_width: i32,
    pub paddle_height: i32,
    pub paddle_width: i32,
    pub paddle_color: Color,
    pub paddle_speed: i32,
    pub background: Color,
    margin_right: i32,
    margin_left: i32,
    pub ball_size: i32,
    pub ball_color: Color,
    pub base_ball_speed: i32,
    pub fps: u32,
}

impl GameConfig {
    pub fn new(config: &WindowConfig) -> Self {
        Self {
            margin_right: 10,
            margin_left: 10,
            paddle_height: 100,
            paddle_width: 10,
            paddle_color: Color::WHITE,
            paddle_speed: 3,
            background: Color::BLACK,
            game_height: config.height,
            game_width: config.width,
            ball_size: 20,
            ball_color: Color::WHITE,
            base_ball_speed: 4,
            fps: 60,
        }
    }
}

pub struct GameState {
    pub player_one_pos: (i32, i32),
    pub player_two_pos: (i32, i32),
    pub ball_pos: (i32, i32),
    pub ball_direction: (i32, i32),
    pub score: (i32, i32),
    pub paused: bool,
    pub hit: bool,
    pub miss: bool,
    pub reset_timer: u32,
    pub rally: i32,
    pub ball_speed: i32,
    pub ball_speed_display_timer: u32,
}

impl GameState {
    pub fn new(config: &GameConfig) -> Self {
        let ball_dir = (get_random_value(), get_random_value());

        GameState {
            player_one_pos: (
                config.margin_left,
                config.game_height / 2 - config.paddle_height / 2,
            ),
            player_two_pos: (
                config.game_width - config.margin_right - config.paddle_width,
                config.game_height / 2 - config.paddle_height / 2,
            ),
            ball_pos: (
                config.game_width / 2 - config.ball_size / 2,
                config.game_height / 2 - config.ball_size / 2,
            ),
            ball_direction: ball_dir,
            score: (0, 0),
            paused: false,
            hit: false,
            miss: false,
            reset_timer: config.fps * 3,
            rally: 0,
            ball_speed: config.base_ball_speed,
            ball_speed_display_timer: 0,
        }
    }

    pub fn player_one_hitbox(&self, config: &GameConfig) -> Rectangle {
        Rectangle::new(
            self.player_one_pos.0 as f32,
            self.player_one_pos.1 as f32,
            config.paddle_width as f32,
            config.paddle_height as f32,
        )
    }

    pub fn player_two_hitbox(&self, config: &GameConfig) -> Rectangle {
        Rectangle::new(
            self.player_two_pos.0 as f32,
            self.player_two_pos.1 as f32,
            config.paddle_width as f32,
            config.paddle_height as f32,
        )
    }

    pub fn ball_hitbox(&self, config: &GameConfig, new_ball_pos: (i32, i32)) -> Rectangle {
        Rectangle::new(
            new_ball_pos.0 as f32,
            new_ball_pos.1 as f32,
            config.ball_size as f32,
            config.ball_size as f32,
        )
    }

    pub fn score_and_reset(&mut self, config: &GameConfig, winner: i32) {
        assert!(winner == 1 || winner == 2, "winner should be 1 or 2");
        let coin_flip = get_random_value();

        if winner == 1 {
            self.score.0 += 1;
            self.ball_direction = (-1, coin_flip)
        } else {
            self.score.1 += 1;
            self.ball_direction = (1, coin_flip);
        }

        self.reset_timer = config.fps * 3;
        self.ball_pos = (
            config.game_width / 2 - config.ball_size / 2,
            config.game_height / 2 - config.ball_size / 2,
        );
        self.rally = 0;
        self.ball_speed = config.base_ball_speed;
    }
}
