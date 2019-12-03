use factory::{ParameterizedFactory};

pub const FIELD_WIDTH: u8 = 10;
pub const FIELD_HEIGHT: u8 = 20;
pub const TETRO_SIZE: u32 = 20;
pub const BLOCKS_NUMBER: u8 = (FIELD_WIDTH - 1) * 2 + (FIELD_HEIGHT - 1) * 2;

pub struct Game {
    field: [u32; (FIELD_HEIGHT * FIELD_WIDTH) as usize],
}

impl Game {
    pub fn new() -> Game {
        Game {
            field: [0; (FIELD_HEIGHT * FIELD_WIDTH) as usize],
        }
    }

    pub fn check_full_lines(&self) -> Vec<u8> {
        let mut lines_index = Vec::with_capacity(FIELD_HEIGHT as usize);
        // we start from the end
        for i in (FIELD_HEIGHT - 1)..0 {
            let mut one_empty: bool = false;
            for j in 0..FIELD_WIDTH {
                one_empty = self.field[(i * FIELD_WIDTH + j) as usize] == 0;
                if one_empty {
                    break;
                }
            }

            if !one_empty {
                lines_index.push(i);
            }
        }
        return lines_index;
    }
}

struct TetrominoFactory {
    tetrominos: Vec< Tetromino >,
}

impl TetrominoFactory {
    fn new() -> TetrominoFactory {
        TetrominoFactory {
            tetrominos: Vec::new(),
        }
    }

    fn registerTetromino(self: &mut TetrominoFactory, tetros: [i16; 4]) {
        let color = 0xFF0000FF;
        let mut field: [[u32; 16]; 4] = [[0; 16]; 4];
        for index in 0..3 {
            let mut local_tetros = tetros[index];
            for i in 0..15 {
                local_tetros = local_tetros << i;
                field[index][i] = local_tetros as u32 & 0x1;
            }
        }

        let tetromino = Tetromino::new(field, color);
        self.tetrominos.push(tetromino);
    }
}

impl ParameterizedFactory for TetrominoFactory {
    type Item = Tetromino;

    type Parameter = u8;

    fn create(self: &TetrominoFactory, param: Self::Parameter) -> Self::Item {
        return self.tetrominos[param as usize].clone();
    }
}

#[derive(Clone)]
struct Tetromino {
    field: [[u32; 16]; 4],
    color: u32,
    current_rotation: u8,
}

impl Tetromino {

    fn new(field: [[u32; 16]; 4], color: u32) -> Tetromino {
        Tetromino {
            field: field,
            color: color,
            current_rotation: 0,
        }
    }

    fn draw(self: &Tetromino, game: &mut Game, xy: Point) {
        for i in 0..3 {
            for j in 0..3 {
                game.field[(xy.y * FIELD_WIDTH + xy.x) as usize] = self.field[self.current_rotation as usize][i * 4 + j];
            }
        }
    }
}

struct Point {
    x: u8,
    y: u8,
}

impl Point {
    fn new(px: u8, py: u8) -> Point {
        Point {
            x: px,
            y: py,
        }
    }
}

impl Default for Point {
    fn default() -> Point {
        Point::new(0, 0)
    }
}
