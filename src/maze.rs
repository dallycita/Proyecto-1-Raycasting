use std::fs;

pub type Maze = Vec<Vec<char>>;

pub fn load_maze(filename: &str) -> Maze {
    let contents = fs::read_to_string(filename)
        .expect("No se pudo leer el archivo maze.txt");

    contents
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

pub fn find_player_start(maze: &mut Maze) -> (usize, usize) {
    for (row, line) in maze.iter_mut().enumerate() {
        for (col, cell) in line.iter_mut().enumerate() {
            if *cell == 'p' {
                *cell = ' ';
                return (col, row);
            }
        }
    }

    panic!("No se encontro la posicion inicial 'p'");
}
