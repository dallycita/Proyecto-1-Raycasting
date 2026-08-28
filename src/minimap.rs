use raylib::prelude::*;

use crate::maze::Maze;
use crate::player::Player;

pub fn draw_minimap(
    d: &mut RaylibDrawHandle,
    maze: &Maze,
    player: &Player,
    block_size: f32,
    screen_width: i32,
) {
    let mini_block = 8;

    let map_width =
        maze[0].len() as i32 * mini_block;

    let margin = 15;

    let start_x =
        screen_width - map_width - margin;

    let start_y = margin;

    // Fondo del minimapa
    d.draw_rectangle(
        start_x - 5,
        start_y - 5,
        map_width + 10,
        maze.len() as i32 * mini_block + 10,
        Color::new(0, 0, 0, 180),
    );

    // Dibujar celdas del mapa
    for (row, line) in maze.iter().enumerate() {
        for (col, &cell) in line.iter().enumerate() {
            let x =
                start_x + col as i32 * mini_block;

            let y =
                start_y + row as i32 * mini_block;

            let color = match cell {
                '+' => Color::RED,
                '-' => Color::BLUE,
                '|' => Color::GREEN,
                'g' => Color::PURPLE,
                'E' => Color::GOLD,
                _ => Color::DARKGRAY,
            };

            if cell != ' ' {
                d.draw_rectangle(
                    x,
                    y,
                    mini_block,
                    mini_block,
                    color,
                );
            }
        }
    }

    // Posicion del jugador dentro del mapa
    let player_map_x =
        player.pos.x / block_size;

    let player_map_y =
        player.pos.y / block_size;

    let player_x =
        start_x
        + (player_map_x * mini_block as f32) as i32;

    let player_y =
        start_y
        + (player_map_y * mini_block as f32) as i32;

    // Jugador
    d.draw_circle(
        player_x,
        player_y,
        3.0,
        Color::YELLOW,
    );

    // Direccion del jugador
    let dir_length = 10.0;

    let dir_x =
        player_x as f32
        - player.a.cos() * dir_length;

    let dir_y =
        player_y as f32
        - player.a.sin() * dir_length;

    d.draw_line(
        player_x,
        player_y,
        dir_x as i32,
        dir_y as i32,
        Color::WHITE,
    );
}
