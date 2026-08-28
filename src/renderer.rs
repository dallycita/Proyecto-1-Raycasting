use raylib::prelude::*;

use crate::caster::cast_ray;
use crate::maze::Maze;
use crate::player::Player;
use crate::textures::TextureManager;

pub fn wall_color(cell: char) -> Color {
    match cell {
        '+' => Color::RED,
        '-' => Color::BLUE,
        '|' => Color::GREEN,
        'g' => Color::PURPLE,
        'E' => Color::GOLD,
        _ => Color::GRAY,
    }
}

pub fn render_world(
    d: &mut RaylibDrawHandle,
    maze: &Maze,
    player: &Player,
    textures: &mut TextureManager,
    screen_width: i32,
    screen_height: i32,
    block_size: f32,
) -> Vec<f32> {

    let half_height =
        screen_height / 2;

    // --------------------------------
    // Z-BUFFER
    // --------------------------------

    let mut z_buffer =
        vec![
            f32::MAX;
            screen_width as usize
        ];

    // --------------------------------
    // CIELO
    // --------------------------------

    d.draw_rectangle(
        0,
        0,
        screen_width,
        half_height,
        Color::new(
            80,
            120,
            170,
            255,
        ),
    );

    // --------------------------------
    // PISO
    // --------------------------------

    d.draw_rectangle(
        0,
        half_height,
        screen_width,
        half_height,
        Color::new(
            50,
            50,
            50,
            255,
        ),
    );

    let num_rays =
        screen_width;

    let projection_plane_distance =
        (screen_width as f32 / 2.0)
            / (player.fov / 2.0).tan();

    // --------------------------------
    // RAYCASTING
    // --------------------------------

    for x in 0..num_rays {

        let ray_progress =
            x as f32
                / num_rays as f32;

        let ray_angle =
            player.a
                - player.fov / 2.0
                + player.fov
                    * ray_progress;

        let hit =
            cast_ray(
                maze,
                player,
                ray_angle,
                block_size,
            );

        // --------------------------------
        // CORRECCION FISH-EYE
        // --------------------------------

        let corrected_distance =
            hit.distance
                * (
                    ray_angle
                        - player.a
                )
                .cos();

        let safe_distance =
            corrected_distance.max(1.0);

        // --------------------------------
        // GUARDAR DISTANCIA EN Z-BUFFER
        // --------------------------------

        z_buffer[x as usize] =
            safe_distance;

        // --------------------------------
        // ALTURA DE LA PARED
        // --------------------------------

        let wall_height =
            (
                block_size
                    / safe_distance
            )
                * projection_plane_distance;

        let mut stake_height =
            wall_height as i32;

        if stake_height
            > screen_height * 2
        {
            stake_height =
                screen_height * 2;
        }

        let original_top =
            half_height
                - stake_height / 2;

        let original_bottom =
            half_height
                + stake_height / 2;

        let stake_top =
            original_top.max(0);

        let stake_bottom =
            original_bottom
                .min(screen_height - 1);

        // --------------------------------
        // TAMANO DE TEXTURA
        // --------------------------------

        let (
            texture_width,
            texture_height
        ) =
            textures.get_size(
                hit.impact
            );

        // --------------------------------
        // TX
        // --------------------------------

        let local_x =
            hit.hit_x
                / block_size;

        let local_y =
            hit.hit_y
                / block_size;

        let frac_x =
            local_x
                - local_x.floor();

        let frac_y =
            local_y
                - local_y.floor();

        let use_x =
            if frac_x < 0.01
                || frac_x > 0.99
            {
                frac_y
            } else {
                frac_x
            };

        let mut tx =
            (
                use_x
                    * texture_width
                        as f32
            )
                as i32;

        if tx >= texture_width {
            tx =
                texture_width - 1;
        }

        if tx < 0 {
            tx = 0;
        }

        // --------------------------------
        // TY
        // --------------------------------

        for y in
            stake_top
                ..stake_bottom
        {
            let wall_y =
                y - original_top;

            let normalized_y =
                wall_y as f32
                    / stake_height
                        as f32;

            let mut ty =
                (
                    normalized_y
                        * texture_height
                            as f32
                )
                    as i32;

            if ty >= texture_height {
                ty =
                    texture_height - 1;
            }

            if ty < 0 {
                ty = 0;
            }

            // --------------------------------
            // COLOR DE TEXTURA
            // --------------------------------

            let color =
                textures.get_pixel_color(
                    hit.impact,
                    tx,
                    ty,
                );

            d.draw_pixel(
                x,
                y,
                color,
            );
        }
    }

    // --------------------------------
    // TEXTO
    // --------------------------------

    d.draw_text(
        "Vista 3D",
        10,
        10,
        20,
        Color::WHITE,
    );

    d.draw_text(
        "M = cambiar vista",
        10,
        35,
        18,
        Color::WHITE,
    );

    d.draw_text(
        "WASD = mover | Mouse/Flechas = girar",
        10,
        60,
        18,
        Color::WHITE,
    );

    // --------------------------------
    // DEVOLVER Z-BUFFER
    // --------------------------------

    z_buffer
}
