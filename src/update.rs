use raylib::{
    RaylibHandle,
    ffi::KeyboardKey::{KEY_DOWN, KEY_P, KEY_S, KEY_UP, KEY_W},
};

use crate::{game, game::Pong};

fn ball_hit_paddle(
    state: &mut game::GameState,
    config: &game::GameConfig,
    new_ball_pos: (i32, i32),
) -> bool {
    let mut flag = false;
    let ball_hitbox = state.ball_hitbox(config, new_ball_pos);

    let player_one_hitbox = state.player_one_hitbox(config);
    if player_one_hitbox.check_collision_recs(ball_hitbox) && state.ball_direction.0 < 0 {
        flag = true;
    }

    let player_two_hitbox = state.player_two_hitbox(config);
    if player_two_hitbox.check_collision_recs(ball_hitbox) && state.ball_direction.0 > 0 {
        flag = true;
    }

    flag
}

pub fn update(config: &game::GameConfig, state: &mut game::GameState, rl: &RaylibHandle) {
    // pause
    state.paused = if rl.is_key_pressed(KEY_P) && !state.paused {
        true
    } else if rl.is_key_pressed(KEY_P) && state.paused {
        false
    } else {
        state.paused
    };

    if state.paused {
        return;
    }

    if state.reset_timer > 0 {
        state.reset_timer -= 1;
    }

    if state.ball_speed_display_timer > 0 {
        state.ball_speed_display_timer -= 1;
    }

    // player one
    let player_one_dir = if rl.is_key_down(KEY_S) {
        1
    } else if rl.is_key_down(KEY_W) {
        -1
    } else {
        0
    };
    state.player_one_pos.1 = (state.player_one_pos.1 + player_one_dir * config.paddle_speed)
        .clamp(0, config.game_height - config.paddle_height);

    // player two
    let player_two_dir = if rl.is_key_down(KEY_DOWN) {
        1
    } else if rl.is_key_down(KEY_UP) {
        -1
    } else {
        0
    };
    state.player_two_pos.1 = (state.player_two_pos.1 + player_two_dir * config.paddle_speed)
        .clamp(0, config.game_height - config.paddle_height);

    // ball
    if state.reset_timer > 0 {
        return;
    }

    let mut new_ball_pos_0 = state.ball_pos.0 + state.ball_direction.0 * state.ball_speed;
    let mut new_ball_pos_1 = state.ball_pos.1 + state.ball_direction.1 * state.ball_speed;

    if new_ball_pos_1 <= 0 || new_ball_pos_1 >= config.game_height - config.ball_size {
        state.hit = true;
        state.ball_direction.1 *= -1;
        new_ball_pos_1 = state.ball_pos.1 + state.ball_direction.1 * state.ball_speed;
    } else if ball_hit_paddle(state, config, (new_ball_pos_0, new_ball_pos_1)) {
        state.hit = true;
        state.ball_direction.0 *= -1;
        state.rally += 1;
        if state.rally % 5 == 0 && state.rally > 0 {
            state.ball_speed += 1;
            state.ball_speed_display_timer = config.fps;
        }
        new_ball_pos_0 = state.ball_pos.0 + state.ball_direction.0 * state.ball_speed;
    } else if new_ball_pos_0 <= 0 {
        state.miss = true;
        state.score_and_reset(config, 2);
        return;
    } else if new_ball_pos_0 >= config.game_width - config.ball_size {
        state.miss = true;
        state.score_and_reset(config, 1);
        return;
    }

    state.ball_pos.0 = new_ball_pos_0;
    state.ball_pos.1 = new_ball_pos_1;
}
