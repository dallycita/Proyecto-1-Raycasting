use crate::maze::Maze;
use crate::player::Player;

pub struct Intersect {
    pub distance: f32,
    pub impact: char,
    pub hit_x: f32,
    pub hit_y: f32,
}

pub fn cast_ray(
    maze: &Maze,
    player: &Player,
    angle: f32,
    block_size: f32,
) -> Intersect {
    let mut distance = 0.0;

    loop {
        let ray_x = player.pos.x - angle.cos() * distance;
        let ray_y = player.pos.y - angle.sin() * distance;

        if ray_x < 0.0 || ray_y < 0.0 {
            return Intersect {
                distance,
                impact: '#',
                hit_x: ray_x,
                hit_y: ray_y,
            };
        }

        let map_x = (ray_x / block_size) as usize;
        let map_y = (ray_y / block_size) as usize;

        if map_y >= maze.len() || map_x >= maze[map_y].len() {
            return Intersect {
                distance,
                impact: '#',
                hit_x: ray_x,
                hit_y: ray_y,
            };
        }

        let cell = maze[map_y][map_x];

        if cell != ' ' {
            return Intersect {
                distance,
                impact: cell,
                hit_x: ray_x,
                hit_y: ray_y,
            };
        }

        distance += 1.0;
    }
}
