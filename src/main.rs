use raylib::prelude::*;
use std::ffi::CString;

mod maze;
mod player;
mod caster;
mod renderer;
mod minimap;
mod textures;
mod sprite;

use maze::{
    find_player_start,
    load_maze,
};

use player::Player;
use caster::cast_ray;

use renderer::{
    render_world,
    wall_color,
};

use minimap::draw_minimap;
use textures::TextureManager;
use sprite::AnimatedSprite;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;
const BLOCK_SIZE: i32 = 40;

fn main() {

    // ============================================================
    // VENTANA
    // ============================================================

    let (mut rl, thread) =
        raylib::init()
            .size(
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
            )
            .title(
                "Ray Caster - Proyecto"
            )
            .build();

    rl.set_target_fps(60);

    // ============================================================
    // AUDIO
    // ============================================================

    unsafe {
        raylib::ffi::InitAudioDevice();
    }

    let music_path =
        CString::new(
            "assets/background.wav"
        )
        .unwrap();

    let shot_path =
        CString::new(
            "assets/shot.wav"
        )
        .unwrap();

    let hit_path =
        CString::new(
            "assets/hit.wav"
        )
        .unwrap();

    let success_path =
        CString::new(
            "assets/success.wav"
        )
        .unwrap();

    let music =
        unsafe {
            raylib::ffi::LoadMusicStream(
                music_path.as_ptr()
            )
        };

    let shot_sound =
        unsafe {
            raylib::ffi::LoadSound(
                shot_path.as_ptr()
            )
        };

    let hit_sound =
        unsafe {
            raylib::ffi::LoadSound(
                hit_path.as_ptr()
            )
        };

    let success_sound =
        unsafe {
            raylib::ffi::LoadSound(
                success_path.as_ptr()
            )
        };

    unsafe {

        raylib::ffi::SetMusicVolume(
            music.clone(),
            0.35,
        );

        raylib::ffi::SetSoundVolume(
            shot_sound.clone(),
            0.55,
        );

        raylib::ffi::SetSoundVolume(
            hit_sound.clone(),
            0.65,
        );

        raylib::ffi::SetSoundVolume(
            success_sound.clone(),
            0.60,
        );

        raylib::ffi::PlayMusicStream(
            music.clone()
        );
    }

    // ============================================================
    // NIVEL INICIAL
    // ============================================================

    let mut maze =
        load_maze(
            "maze1.txt"
        );

    let (
        player_col,
        player_row,
    ) =
        find_player_start(
            &mut maze
        );

    let mut start_x =
        player_col as f32
            * BLOCK_SIZE as f32
            + BLOCK_SIZE as f32
                / 2.0;

    let mut start_y =
        player_row as f32
            * BLOCK_SIZE as f32
            + BLOCK_SIZE as f32
                / 2.0;

    let mut player =
        Player::new(
            start_x,
            start_y,
        );

    let mut textures =
        TextureManager::new();

    // ============================================================
    // SPRITE ANIMADO
    // ============================================================

    let mut enemy =
        AnimatedSprite::new(
            9.5
                * BLOCK_SIZE as f32,

            11.5
                * BLOCK_SIZE as f32,
        );

    // ============================================================
    // ESTADO DEL JUEGO
    // ============================================================

    let mut mode_3d =
        true;

    let mut welcome_screen =
        true;

    let mut level_complete =
        false;

    let mut current_level =
        1;

    let mut shot_flash_frames =
        0;

    let mut hit_message_frames =
        0;

    rl.enable_cursor();

    // ============================================================
    // LOOP PRINCIPAL
    // ============================================================

    while !rl.window_should_close() {

        // --------------------------------------------------------
        // La musica en streaming debe actualizarse cada frame.
        // --------------------------------------------------------

        unsafe {
            raylib::ffi::UpdateMusicStream(
                music.clone()
            );
        }

        let current_fps =
            rl.get_fps();

        // ========================================================
        // MENU
        // ========================================================

        if welcome_screen {

            let level_1_pressed =
                rl.is_key_pressed(
                    KeyboardKey::KEY_ONE
                );

            let level_2_pressed =
                rl.is_key_pressed(
                    KeyboardKey::KEY_TWO
                );

            if level_1_pressed
                || level_2_pressed
            {
                let level_file;

                if level_1_pressed {

                    current_level = 1;

                    level_file =
                        "maze1.txt";

                } else {

                    current_level = 2;

                    level_file =
                        "maze2.txt";
                }

                maze =
                    load_maze(
                        level_file
                    );

                let (
                    new_player_col,
                    new_player_row,
                ) =
                    find_player_start(
                        &mut maze
                    );

                start_x =
                    new_player_col
                        as f32
                        * BLOCK_SIZE
                            as f32
                        + BLOCK_SIZE
                            as f32
                            / 2.0;

                start_y =
                    new_player_row
                        as f32
                        * BLOCK_SIZE
                            as f32
                        + BLOCK_SIZE
                            as f32
                            / 2.0;

                player =
                    Player::new(
                        start_x,
                        start_y,
                    );

                enemy =
                    AnimatedSprite::new(
                        9.5
                            * BLOCK_SIZE
                                as f32,

                        11.5
                            * BLOCK_SIZE
                                as f32,
                    );

                mode_3d =
                    true;

                level_complete =
                    false;

                welcome_screen =
                    false;

                shot_flash_frames =
                    0;

                hit_message_frames =
                    0;

                rl.disable_cursor();
            }

            let mut d =
                rl.begin_drawing(
                    &thread
                );

            d.clear_background(
                Color::new(
                    18,
                    18,
                    28,
                    255,
                ),
            );

            // Fondo decorativo
            d.draw_rectangle(
                170,
                70,
                460,
                475,
                Color::new(
                    30,
                    30,
                    45,
                    255,
                ),
            );

            d.draw_rectangle_lines(
                170,
                70,
                460,
                475,
                Color::GOLD,
            );

            d.draw_text(
                "RAY CASTER",
                235,
                100,
                50,
                Color::GOLD,
            );

            d.draw_text(
                "Proyecto - Parte 3",
                280,
                165,
                25,
                Color::WHITE,
            );

            d.draw_text(
                "Selecciona un nivel",
                280,
                225,
                25,
                Color::LIGHTGRAY,
            );

            d.draw_rectangle(
                230,
                275,
                340,
                55,
                Color::new(
                    60,
                    60,
                    80,
                    255,
                ),
            );

            d.draw_text(
                "[1] NIVEL 1",
                320,
                290,
                25,
                Color::GREEN,
            );

            d.draw_rectangle(
                230,
                345,
                340,
                55,
                Color::new(
                    60,
                    60,
                    80,
                    255,
                ),
            );

            d.draw_text(
                "[2] NIVEL 2",
                320,
                360,
                25,
                Color::SKYBLUE,
            );

            d.draw_text(
                "WASD = mover",
                275,
                425,
                18,
                Color::LIGHTGRAY,
            );

            d.draw_text(
                "MOUSE = girar",
                275,
                450,
                18,
                Color::LIGHTGRAY,
            );

            d.draw_text(
                "CLICK / SPACE = disparar",
                275,
                475,
                18,
                Color::LIGHTGRAY,
            );

            d.draw_text(
                "M = cambiar vista",
                275,
                500,
                18,
                Color::LIGHTGRAY,
            );

            d.draw_text(
                "Llega a la salida dorada",
                275,
                525,
                18,
                Color::GOLD,
            );

            continue;
        }

        // ========================================================
        // PANTALLA DE EXITO
        // ========================================================

        if level_complete {

            let enter_pressed =
                rl.is_key_pressed(
                    KeyboardKey::KEY_ENTER
                );

            if enter_pressed {

                level_complete =
                    false;

                welcome_screen =
                    true;

                rl.enable_cursor();
            }

            let mut d =
                rl.begin_drawing(
                    &thread
                );

            d.clear_background(
                Color::new(
                    15,
                    20,
                    15,
                    255,
                ),
            );

            d.draw_rectangle(
                160,
                145,
                480,
                300,
                Color::new(
                    25,
                    45,
                    30,
                    255,
                ),
            );

            d.draw_rectangle_lines(
                160,
                145,
                480,
                300,
                Color::GREEN,
            );

            d.draw_text(
                "NIVEL COMPLETADO",
                205,
                190,
                40,
                Color::GREEN,
            );

            let level_text =
                format!(
                    "Completaste el Nivel {}",
                    current_level
                );

            d.draw_text(
                &level_text,
                265,
                260,
                25,
                Color::WHITE,
            );

            d.draw_text(
                "Llegaste a la salida!",
                270,
                305,
                22,
                Color::GOLD,
            );

            d.draw_text(
                "ENTER = volver al menu",
                265,
                365,
                22,
                Color::LIGHTGRAY,
            );

            continue;
        }

        // ========================================================
        // VELOCIDADES
        // ========================================================

        let move_speed =
            3.0;

        let rotation_speed =
            0.05;

        let mouse_sensitivity =
            0.0035;

        // ========================================================
        // SPRITE
        // ========================================================

        if current_level == 2 {
            enemy.update();
        }

        // ========================================================
        // EFECTOS TEMPORALES
        // ========================================================

        if shot_flash_frames > 0 {
            shot_flash_frames -= 1;
        }

        if hit_message_frames > 0 {
            hit_message_frames -= 1;
        }

        // ========================================================
        // GAMEPAD
        // ========================================================

        let gamepad_available =
            rl.is_gamepad_available(0);

        let mut gamepad_left_x =
            0.0;

        let mut gamepad_left_y =
            0.0;

        let mut gamepad_right_x =
            0.0;

        let mut gamepad_shoot =
            false;

        if gamepad_available {

            gamepad_left_x =
                rl.get_gamepad_axis_movement(
                    0,
                    GamepadAxis::GAMEPAD_AXIS_LEFT_X,
                );

            gamepad_left_y =
                rl.get_gamepad_axis_movement(
                    0,
                    GamepadAxis::GAMEPAD_AXIS_LEFT_Y,
                );

            gamepad_right_x =
                rl.get_gamepad_axis_movement(
                    0,
                    GamepadAxis::GAMEPAD_AXIS_RIGHT_X,
                );

            gamepad_shoot =
                rl.is_gamepad_button_pressed(
                    0,
                    GamepadButton::
                        GAMEPAD_BUTTON_RIGHT_FACE_DOWN,
                );
        }

        // ========================================================
        // CAMBIAR VISTA
        // ========================================================

        if rl.is_key_pressed(
            KeyboardKey::KEY_M
        ) {
            mode_3d =
                !mode_3d;
        }

        // ========================================================
        // ROTACION CON FLECHAS
        // ========================================================

        if rl.is_key_down(
            KeyboardKey::KEY_LEFT
        ) {
            player.a -=
                rotation_speed;
        }

        if rl.is_key_down(
            KeyboardKey::KEY_RIGHT
        ) {
            player.a +=
                rotation_speed;
        }

        // ========================================================
        // ROTACION CON MOUSE
        // ========================================================

        let mouse_delta =
            rl.get_mouse_delta();

        player.a +=
            mouse_delta.x
                * mouse_sensitivity;

        // ========================================================
        // ROTACION CON CONTROL
        // ========================================================

        if gamepad_right_x.abs()
            > 0.15
        {
            player.a +=
                gamepad_right_x
                    * 0.045;
        }

        // ========================================================
        // DISPARO
        // ========================================================

        let shoot_pressed =
            rl.is_mouse_button_pressed(
                MouseButton::
                    MOUSE_BUTTON_LEFT
            )
            || rl.is_key_pressed(
                KeyboardKey::KEY_SPACE
            )
            || gamepad_shoot;

        if shoot_pressed
            && mode_3d
        {
            shot_flash_frames =
                5;

            unsafe {
                raylib::ffi::PlaySound(
                    shot_sound.clone()
                );
            }

            if current_level == 2
                && enemy.alive
            {
                let angle_difference =
                    enemy.angle_difference(
                        &player
                    );

                let enemy_distance =
                    enemy.distance_to_player(
                        &player
                    );

                let wall_hit =
                    cast_ray(
                        &maze,
                        &player,
                        player.a,
                        BLOCK_SIZE as f32,
                    );

                let aiming_at_enemy =
                    angle_difference.abs()
                        < 0.07;

                let enemy_before_wall =
                    enemy_distance
                        < wall_hit.distance
                            + 3.0;

                if aiming_at_enemy
                    && enemy_before_wall
                {
                    enemy.defeat();

                    hit_message_frames =
                        90;

                    unsafe {
                        raylib::ffi::PlaySound(
                            hit_sound.clone()
                        );
                    }
                }
            }
        }

        // ========================================================
        // MOVIMIENTO
        // ========================================================

        let mut new_x =
            player.pos.x;

        let mut new_y =
            player.pos.y;

        // W
        if rl.is_key_down(
            KeyboardKey::KEY_W
        ) {
            new_x -=
                player.a.cos()
                    * move_speed;

            new_y -=
                player.a.sin()
                    * move_speed;
        }

        // S
        if rl.is_key_down(
            KeyboardKey::KEY_S
        ) {
            new_x +=
                player.a.cos()
                    * move_speed;

            new_y +=
                player.a.sin()
                    * move_speed;
        }

        // A
        if rl.is_key_down(
            KeyboardKey::KEY_A
        ) {
            new_x -=
                (
                    player.a
                        - std::f32::consts::PI
                            / 2.0
                )
                .cos()
                    * move_speed;

            new_y -=
                (
                    player.a
                        - std::f32::consts::PI
                            / 2.0
                )
                .sin()
                    * move_speed;
        }

        // D
        if rl.is_key_down(
            KeyboardKey::KEY_D
        ) {
            new_x -=
                (
                    player.a
                        + std::f32::consts::PI
                            / 2.0
                )
                .cos()
                    * move_speed;

            new_y -=
                (
                    player.a
                        + std::f32::consts::PI
                            / 2.0
                )
                .sin()
                    * move_speed;
        }

        // ========================================================
        // MOVIMIENTO CON CONTROL
        // ========================================================

        if gamepad_left_y.abs()
            > 0.15
        {
            new_x +=
                player.a.cos()
                    * move_speed
                    * gamepad_left_y;

            new_y +=
                player.a.sin()
                    * move_speed
                    * gamepad_left_y;
        }

        if gamepad_left_x.abs()
            > 0.15
        {
            new_x -=
                (
                    player.a
                        + std::f32::consts::PI
                            / 2.0
                )
                .cos()
                    * move_speed
                    * gamepad_left_x;

            new_y -=
                (
                    player.a
                        + std::f32::consts::PI
                            / 2.0
                )
                .sin()
                    * move_speed
                    * gamepad_left_x;
        }

        // ========================================================
        // POSICION EN MAPA
        // ========================================================

        let map_x =
            (
                new_x
                    / BLOCK_SIZE as f32
            )
                as usize;

        let map_y =
            (
                new_y
                    / BLOCK_SIZE as f32
            )
                as usize;

        // ========================================================
        // SALIDA
        // ========================================================

        if map_y < maze.len()
            && map_x
                < maze[map_y].len()
            && maze[map_y][map_x]
                == 'E'
        {
            player.pos.x =
                new_x;

            player.pos.y =
                new_y;

            level_complete =
                true;

            unsafe {
                raylib::ffi::PlaySound(
                    success_sound.clone()
                );
            }

            rl.enable_cursor();
        }

        // ========================================================
        // COLISION
        // ========================================================

        if !level_complete
            && map_y < maze.len()
            && map_x
                < maze[map_y].len()
            && (
                maze[map_y][map_x]
                    == ' '
                || maze[map_y][map_x]
                    == 'E'
            )
        {
            player.pos.x =
                new_x;

            player.pos.y =
                new_y;
        }

        // ========================================================
        // DIBUJAR
        // ========================================================

        let mut d =
            rl.begin_drawing(
                &thread
            );

        d.clear_background(
            Color::BLACK
        );

        // ========================================================
        // VISTA 3D
        // ========================================================

        if mode_3d {

            let z_buffer =
                render_world(
                    &mut d,
                    &maze,
                    &player,
                    &mut textures,
                    SCREEN_WIDTH,
                    SCREEN_HEIGHT,
                    BLOCK_SIZE as f32,
                );

            // ----------------------------------------------------
            // SPRITE
            // ----------------------------------------------------

            if current_level == 2 {

                enemy.draw(
                    &mut d,
                    &player,
                    &z_buffer,
                    SCREEN_WIDTH,
                    SCREEN_HEIGHT,
                    BLOCK_SIZE as f32,
                );
            }

            // ----------------------------------------------------
            // MINIMAPA
            // ----------------------------------------------------

            draw_minimap(
                &mut d,
                &maze,
                &player,
                BLOCK_SIZE as f32,
                SCREEN_WIDTH,
            );

            // ----------------------------------------------------
            // MIRA
            // ----------------------------------------------------

            let center_x =
                SCREEN_WIDTH / 2;

            let center_y =
                SCREEN_HEIGHT / 2;

            d.draw_circle_lines(
                center_x,
                center_y,
                5.0,
                Color::WHITE,
            );

            d.draw_line(
                center_x - 12,
                center_y,
                center_x - 4,
                center_y,
                Color::WHITE,
            );

            d.draw_line(
                center_x + 4,
                center_y,
                center_x + 12,
                center_y,
                Color::WHITE,
            );

            d.draw_line(
                center_x,
                center_y - 12,
                center_x,
                center_y - 4,
                Color::WHITE,
            );

            d.draw_line(
                center_x,
                center_y + 4,
                center_x,
                center_y + 12,
                Color::WHITE,
            );

            // ----------------------------------------------------
            // FLASH
            // ----------------------------------------------------

            if shot_flash_frames > 0 {

                d.draw_circle(
                    center_x,
                    center_y + 120,
                    35.0,
                    Color::GOLD,
                );

                d.draw_circle(
                    center_x,
                    center_y + 120,
                    18.0,
                    Color::YELLOW,
                );
            }

            // ----------------------------------------------------
            // MENSAJE DE IMPACTO
            // ----------------------------------------------------

            if hit_message_frames > 0 {

                d.draw_rectangle(
                    255,
                    155,
                    290,
                    50,
                    Color::new(
                        0,
                        0,
                        0,
                        180,
                    ),
                );

                d.draw_text(
                    "ENEMIGO DERROTADO!",
                    280,
                    170,
                    25,
                    Color::GREEN,
                );
            }

            // ----------------------------------------------------
            // HUD
            // ----------------------------------------------------

            d.draw_rectangle(
                5,
                80,
                300,
                110,
                Color::new(
                    0,
                    0,
                    0,
                    150,
                ),
            );

            let level_text =
                format!(
                    "Nivel {}",
                    current_level
                );

            d.draw_text(
                &level_text,
                15,
                88,
                18,
                Color::GOLD,
            );

            d.draw_text(
                "Mouse = girar",
                15,
                113,
                17,
                Color::WHITE,
            );

            d.draw_text(
                "Click / SPACE = disparar",
                15,
                136,
                17,
                Color::WHITE,
            );

            if current_level == 2 {

                if enemy.alive {

                    d.draw_text(
                        "Objetivo: derrota al sprite",
                        15,
                        160,
                        17,
                        Color::PINK,
                    );

                } else {

                    d.draw_text(
                        "Sprite derrotado",
                        15,
                        160,
                        17,
                        Color::GREEN,
                    );
                }
            }

            // ----------------------------------------------------
            // CONTROL DETECTADO
            // ----------------------------------------------------

            if gamepad_available {

                d.draw_text(
                    "CONTROL CONECTADO",
                    610,
                    555,
                    15,
                    Color::GREEN,
                );
            }

        // ========================================================
        // VISTA 2D
        // ========================================================

        } else {

            for (row, line)
                in maze.iter().enumerate()
            {
                for (col, &cell)
                    in line.iter().enumerate()
                {
                    let x =
                        col as i32
                            * BLOCK_SIZE;

                    let y =
                        row as i32
                            * BLOCK_SIZE;

                    if cell != ' ' {

                        let color =
                            if cell == 'E' {

                                Color::GOLD

                            } else {

                                wall_color(
                                    cell
                                )
                            };

                        d.draw_rectangle(
                            x,
                            y,
                            BLOCK_SIZE,
                            BLOCK_SIZE,
                            color,
                        );
                    }
                }
            }

            // Sprite en vista 2D
            if current_level == 2
                && enemy.alive
            {
                d.draw_circle(
                    enemy.pos.x
                        as i32,

                    enemy.pos.y
                        as i32,

                    7.0,

                    Color::PINK,
                );
            }

            // Rayos
            let num_rays =
                80;

            for ray in 0..num_rays {

                let ray_angle =
                    player.a
                        - player.fov
                            / 2.0
                        + player.fov
                            * ray as f32
                            / num_rays
                                as f32;

                let hit =
                    cast_ray(
                        &maze,
                        &player,
                        ray_angle,
                        BLOCK_SIZE as f32,
                    );

                d.draw_line(
                    player.pos.x
                        as i32,

                    player.pos.y
                        as i32,

                    hit.hit_x
                        as i32,

                    hit.hit_y
                        as i32,

                    Color::new(
                        255,
                        255,
                        255,
                        80,
                    ),
                );
            }

            d.draw_circle(
                player.pos.x
                    as i32,

                player.pos.y
                    as i32,

                6.0,

                Color::YELLOW,
            );

            d.draw_rectangle(
                5,
                5,
                285,
                145,
                Color::new(
                    0,
                    0,
                    0,
                    170,
                ),
            );

            d.draw_text(
                "Vista 2D + multiples rayos",
                10,
                10,
                20,
                Color::WHITE,
            );

            d.draw_text(
                "M = cambiar a vista 3D",
                10,
                38,
                17,
                Color::LIGHTGRAY,
            );

            d.draw_text(
                "WASD = mover",
                10,
                62,
                17,
                Color::LIGHTGRAY,
            );

            d.draw_text(
                "Mouse/Flechas = girar",
                10,
                86,
                17,
                Color::LIGHTGRAY,
            );

            d.draw_text(
                "E dorada = salida",
                10,
                110,
                17,
                Color::GOLD,
            );

            if current_level == 2
                && enemy.alive
            {
                d.draw_text(
                    "Punto rosa = sprite",
                    10,
                    132,
                    17,
                    Color::PINK,
                );
            }
        }

        // ========================================================
        // FPS
        // ========================================================

        let fps_text =
            format!(
                "FPS: {}",
                current_fps
            );

        let fps_color =
            if current_fps >= 55 {
                Color::GREEN
            } else if current_fps >= 40 {
                Color::YELLOW
            } else {
                Color::RED
            };

        d.draw_rectangle(
            700,
            565,
            95,
            30,
            Color::new(
                0,
                0,
                0,
                170,
            ),
        );

        d.draw_text(
            &fps_text,
            710,
            572,
            17,
            fps_color,
        );
    }

    // ============================================================
    // LIMPIAR AUDIO AL CERRAR
    // ============================================================

    unsafe {

        raylib::ffi::StopMusicStream(
            music.clone()
        );

        raylib::ffi::UnloadMusicStream(
            music
        );

        raylib::ffi::UnloadSound(
            shot_sound
        );

        raylib::ffi::UnloadSound(
            hit_sound
        );

        raylib::ffi::UnloadSound(
            success_sound
        );

        raylib::ffi::CloseAudioDevice();
    }
}
