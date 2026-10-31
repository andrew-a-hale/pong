use raylib::{core::drawing::RaylibDrawHandle, drawing::RaylibDraw, ffi::Color};

use crate::game;

fn render_score(config: &game::GameConfig, state: &game::GameState, handle: &mut RaylibDrawHandle) {
    let font_size = 40;
    let score_top_margin_px = 50;

    let player_one_score = format!("{}", state.score.0);
    let player_one_score_size = handle.measure_text(player_one_score.as_str(), font_size);
    handle.draw_text(
        player_one_score.as_str(),
        config.game_width / 4 - player_one_score_size / 2,
        score_top_margin_px,
        font_size,
        Color::WHITE,
    );

    let player_two_score = format!("{}", state.score.1);
    let player_two_score_size = handle.measure_text(player_two_score.as_str(), font_size);
    handle.draw_text(
        player_two_score.as_str(),
        3 * config.game_width / 4 - player_two_score_size / 2,
        score_top_margin_px,
        font_size,
        Color::WHITE,
    );
}

fn render_net(config: &game::GameConfig, _: &game::GameState, handle: &mut RaylibDrawHandle) {
    let dash_size = config.game_height / 100;
    let iter = (0..=config.game_height).step_by(2 * dash_size as usize);
    iter.for_each(|i| {
        handle.draw_rectangle(config.game_width / 2, i, 4, dash_size, Color::WHITE);
    });
}

fn render_ball(config: &game::GameConfig, state: &game::GameState, handle: &mut RaylibDrawHandle) {
    handle.draw_rectangle(
        state.ball_pos.0,
        state.ball_pos.1,
        config.ball_size,
        config.ball_size,
        config.ball_color,
    );
}

fn render_players(
    config: &game::GameConfig,
    state: &game::GameState,
    handle: &mut RaylibDrawHandle,
) {
    handle.draw_rectangle(
        state.player_one_pos.0,
        state.player_one_pos.1,
        config.paddle_width,
        config.paddle_height,
        config.paddle_color,
    );

    handle.draw_rectangle(
        state.player_two_pos.0,
        state.player_two_pos.1,
        config.paddle_width,
        config.paddle_height,
        config.paddle_color,
    );
}

fn render_paused(
    config: &game::GameConfig,
    state: &game::GameState,
    handle: &mut RaylibDrawHandle,
) {
    if !state.paused {
        return;
    }

    handle.draw_rectangle(
        0,
        0,
        config.game_width,
        config.game_height,
        Color::BLACK.alpha(0.9),
    );

    let font_size = 40;
    let text = "PAUSED";
    let text_size = handle.measure_text(text, font_size);

    handle.draw_text(
        text,
        config.game_width / 2 - text_size / 2,
        config.game_height / 2 - font_size / 2,
        font_size,
        Color::WHITE,
    )
}

fn render_reset_timer(
    config: &game::GameConfig,
    state: &game::GameState,
    handle: &mut RaylibDrawHandle,
) {
    if state.reset_timer == 0 {
        return;
    }

    handle.draw_rectangle(
        0,
        0,
        config.game_width,
        config.game_height,
        Color::BLACK.alpha(0.9),
    );

    let font_size = 40;
    let text = format!("{}", state.reset_timer / config.fps);
    let text_size = handle.measure_text(text.as_str(), font_size);

    handle.draw_text(
        text.as_str(),
        config.game_width / 2 - text_size / 2,
        config.game_height / 2 - font_size / 2,
        font_size,
        Color::WHITE,
    )
}

fn render_ball_speed_increase(
    config: &game::GameConfig,
    state: &game::GameState,
    handle: &mut RaylibDrawHandle,
) {
    let font_size = 40;
    let progress = state.ball_speed_display_timer as f32 / config.fps as f32;
    {
        let text = "Speed Up!";
        let text_size = handle.measure_text(text, font_size);

        handle.draw_text(
            text,
            config.game_width / 2 - text_size / 2,
            config.game_height / 4 - font_size / 2,
            font_size,
            Color::WHITE.alpha(progress),
        )
    }

    {
        let text = format!("x{} Rally", state.rally);
        let text_size = handle.measure_text(text.as_str(), font_size);

        handle.draw_text(
            text.as_str(),
            config.game_width / 2 - text_size / 2,
            config.game_height / 4 - font_size / 2 + font_size,
            font_size,
            Color::WHITE.alpha(progress),
        )
    }
}

pub fn render(config: &game::GameConfig, state: &game::GameState, handle: &mut RaylibDrawHandle) {
    render_score(config, state, handle);
    render_net(config, state, handle);
    render_ball(config, state, handle);
    render_players(config, state, handle);
    render_paused(config, state, handle);
    render_reset_timer(config, state, handle);

    if state.ball_speed_display_timer > 0 {
        render_ball_speed_increase(config, state, handle);
    }
}
