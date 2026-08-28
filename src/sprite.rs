use raylib::prelude::*;

use crate::player::Player;

const TRANSPARENT_COLOR: Color =
    Color::new(152, 0, 136, 255);

pub struct AnimatedSprite {
    pub pos: Vector2,

    frames: Vec<Image>,
    current_frame: usize,

    frame_counter: i32,
    frame_speed: i32,

    pub alive: bool,
}

impl AnimatedSprite {

    pub fn new(
        x: f32,
        y: f32,
    ) -> Self {

        let frames = vec![
            create_enemy_frame_1(),
            create_enemy_frame_2(),
            create_enemy_frame_3(),
        ];

        Self {
            pos: Vector2::new(x, y),

            frames,

            current_frame: 0,

            frame_counter: 0,
            frame_speed: 10,

            alive: true,
        }
    }

    // --------------------------------
    // ACTUALIZAR ANIMACION
    // --------------------------------

    pub fn update(&mut self) {

        if !self.alive {
            return;
        }

        self.frame_counter += 1;

        if self.frame_counter
            >= self.frame_speed
        {
            self.frame_counter = 0;

            self.current_frame += 1;

            if self.current_frame
                >= self.frames.len()
            {
                self.current_frame = 0;
            }
        }
    }

    // --------------------------------
    // DISTANCIA AL JUGADOR
    // --------------------------------

    pub fn distance_to_player(
        &self,
        player: &Player,
    ) -> f32 {

        let dx =
            player.pos.x
                - self.pos.x;

        let dy =
            player.pos.y
                - self.pos.y;

        (
            dx * dx
                + dy * dy
        )
        .sqrt()
    }

    // --------------------------------
    // DIFERENCIA ANGULAR
    // --------------------------------

    pub fn angle_difference(
        &self,
        player: &Player,
    ) -> f32 {

        let dx =
            player.pos.x
                - self.pos.x;

        let dy =
            player.pos.y
                - self.pos.y;

        let sprite_angle =
            dy.atan2(dx);

        let mut difference =
            sprite_angle
                - player.a;

        while difference
            > std::f32::consts::PI
        {
            difference -=
                2.0
                    * std::f32::consts::PI;
        }

        while difference
            < -std::f32::consts::PI
        {
            difference +=
                2.0
                    * std::f32::consts::PI;
        }

        difference
    }

    // --------------------------------
    // DERROTAR
    // --------------------------------

    pub fn defeat(&mut self) {
        self.alive = false;
    }

    // --------------------------------
    // DIBUJAR SPRITE
    // --------------------------------

    pub fn draw(
        &mut self,
        d: &mut RaylibDrawHandle,
        player: &Player,
        z_buffer: &[f32],
        screen_width: i32,
        screen_height: i32,
        block_size: f32,
    ) {

        if !self.alive {
            return;
        }

        let dx =
            player.pos.x
                - self.pos.x;

        let dy =
            player.pos.y
                - self.pos.y;

        // atan2
        let sprite_angle =
            dy.atan2(dx);

        let mut angle_difference =
            sprite_angle
                - player.a;

        // Normalizar [-PI, PI]
        while angle_difference
            > std::f32::consts::PI
        {
            angle_difference -=
                2.0
                    * std::f32::consts::PI;
        }

        while angle_difference
            < -std::f32::consts::PI
        {
            angle_difference +=
                2.0
                    * std::f32::consts::PI;
        }

        // Fuera del FOV
        if angle_difference.abs()
            > player.fov / 2.0
        {
            return;
        }

        let distance =
            (
                dx * dx
                    + dy * dy
            )
            .sqrt();

        if distance < 1.0 {
            return;
        }

        let projection_distance =
            (screen_width as f32 / 2.0)
                / (player.fov / 2.0).tan();

        // Tamaño inversamente proporcional
        // a la distancia
        let sprite_size =
            (
                block_size
                    / distance
                    * projection_distance
            )
            .clamp(
                8.0,
                350.0,
            );

        let sprite_size_i =
            sprite_size as i32;

        // Posición horizontal
        let screen_x =
            (
                (
                    angle_difference
                        + player.fov / 2.0
                )
                    / player.fov
                    * screen_width as f32
            )
                as i32;

        let start_x =
            screen_x
                - sprite_size_i / 2;

        // Todos los frames usan exactamente
        // el mismo centro vertical.
        let start_y =
            screen_height / 2
                - sprite_size_i / 2;

        let sprite_depth =
            distance
                * angle_difference.cos();

        let frame =
            &mut self.frames[
                self.current_frame
            ];

        let texture_width =
            frame.width;

        let texture_height =
            frame.height;

        for x in
            start_x
                ..start_x + sprite_size_i
        {
            if x < 0
                || x >= screen_width
            {
                continue;
            }

            // Z-buffer:
            // pared delante = no dibujar.
            if sprite_depth
                >= z_buffer[x as usize]
            {
                continue;
            }

            let relative_x =
                x - start_x;

            let tx =
                (
                    relative_x as f32
                        / sprite_size_i as f32
                        * texture_width as f32
                )
                    as i32;

            for y in
                start_y
                    ..start_y + sprite_size_i
            {
                if y < 0
                    || y >= screen_height
                {
                    continue;
                }

                let relative_y =
                    y - start_y;

                let ty =
                    (
                        relative_y as f32
                            / sprite_size_i as f32
                            * texture_height as f32
                    )
                        as i32;

                if tx < 0
                    || tx >= texture_width
                    || ty < 0
                    || ty >= texture_height
                {
                    continue;
                }

                let color =
                    frame.get_color(
                        tx,
                        ty,
                    );

                if color.r
                        == TRANSPARENT_COLOR.r
                    && color.g
                        == TRANSPARENT_COLOR.g
                    && color.b
                        == TRANSPARENT_COLOR.b
                {
                    continue;
                }

                d.draw_pixel(
                    x,
                    y,
                    color,
                );
            }
        }
    }
}

// --------------------------------
// BASE DEL PERSONAJE
// Todos los frames tienen el MISMO
// tamaño y posición.
// --------------------------------

fn create_enemy_base(
    pupil_offset: i32,
    leg_frame: i32,
) -> Image {

    let mut image =
        Image::gen_image_color(
            64,
            64,
            TRANSPARENT_COLOR,
        );

    let body =
        Color::new(
            230,
            65,
            85,
            255,
        );

    let dark =
        Color::new(
            150,
            30,
            50,
            255,
        );

    // Cabeza
    for y in 15..30 {
        for x in 18..46 {

            if y < 19
                && (x < 23 || x > 40)
            {
                continue;
            }

            image.draw_pixel(
                x,
                y,
                body,
            );
        }
    }

    // Cuerpo
    for y in 30..52 {
        for x in 15..49 {
            image.draw_pixel(
                x,
                y,
                body,
            );
        }
    }

    // Borde inferior
    for x in 15..49 {
        image.draw_pixel(
            x,
            51,
            dark,
        );
    }

    // Ojo izquierdo
    for y in 24..31 {
        for x in 21..28 {
            image.draw_pixel(
                x,
                y,
                Color::WHITE,
            );
        }
    }

    // Ojo derecho
    for y in 24..31 {
        for x in 36..43 {
            image.draw_pixel(
                x,
                y,
                Color::WHITE,
            );
        }
    }

    // Pupilas animadas
    let left_pupil =
        23 + pupil_offset;

    let right_pupil =
        38 + pupil_offset;

    for y in 26..30 {

        for x in
            left_pupil
                ..left_pupil + 3
        {
            image.draw_pixel(
                x,
                y,
                Color::BLACK,
            );
        }

        for x in
            right_pupil
                ..right_pupil + 3
        {
            image.draw_pixel(
                x,
                y,
                Color::BLACK,
            );
        }
    }

    // Boca
    for x in 25..39 {
        image.draw_pixel(
            x,
            40,
            dark,
        );
    }

    // Piernas.
    // Cambian ligeramente, pero sin
    // mover el centro del sprite.
    if leg_frame == 0 {

        for y in 52..60 {

            for x in 18..26 {
                image.draw_pixel(
                    x,
                    y,
                    body,
                );
            }

            for x in 38..46 {
                image.draw_pixel(
                    x,
                    y,
                    body,
                );
            }
        }

    } else if leg_frame == 1 {

        for y in 52..60 {

            for x in 20..28 {
                image.draw_pixel(
                    x,
                    y,
                    body,
                );
            }

            for x in 36..44 {
                image.draw_pixel(
                    x,
                    y,
                    body,
                );
            }
        }

    } else {

        for y in 52..60 {

            for x in 17..25 {
                image.draw_pixel(
                    x,
                    y,
                    body,
                );
            }

            for x in 39..47 {
                image.draw_pixel(
                    x,
                    y,
                    body,
                );
            }
        }
    }

    image
}

fn create_enemy_frame_1() -> Image {
    create_enemy_base(
        0,
        0,
    )
}

fn create_enemy_frame_2() -> Image {
    create_enemy_base(
        1,
        1,
    )
}

fn create_enemy_frame_3() -> Image {
    create_enemy_base(
        -1,
        2,
    )
}
