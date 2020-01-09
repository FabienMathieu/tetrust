use crate::tetrust::pieces::TETROS;
use factory::{ParameterizedFactory};
use rand::Rng;

pub const FIELD_WIDTH: u32 = 10;
pub const FIELD_HEIGHT: u32 = 20;
pub const TETRO_SIZE: u32 = 20;
pub const BLOCKS_NUMBER: u32 = (FIELD_WIDTH - 1) * 2 + (FIELD_HEIGHT - 1) * 2;

pub struct Game {
    field: [u32; (FIELD_HEIGHT * FIELD_WIDTH) as usize],
    factory: TetrominoFactory,
    current_tetromino: Tetromino,
    current_position: Point,
}

impl Game {
    pub fn new() -> Game {
        Game {
            field: [0; (FIELD_HEIGHT * FIELD_WIDTH) as usize],
            factory: TetrominoFactory::new(),
            current_tetromino: Tetromino::default(),
            current_position: Point::default(),
        }
    }

    pub fn get_field(self: &Game) -> &[u32; (FIELD_HEIGHT * FIELD_WIDTH) as usize] {
        return &self.field;
    }

    pub fn init(self: &mut Game) {
        for i in 0..7 {
            self.factory.register_tetromino(TETROS[i]);
        }
    }
    
    pub fn update(self: &mut Game) {
        let tetros_index = rand::thread_rng().gen_range(0, 7);
        let tetros = self.factory.create(tetros_index);
        self.current_tetromino = tetros;
        self.current_position = Point::new(FIELD_WIDTH / 2 - 2, 0);
        self.draw();
    }

    fn draw(self: &mut Game) {
        let tetros: &Tetromino = &self.current_tetromino;
        let xy: &Point = &self.current_position;
        for i in 0..4 {
            for j in 0..4 {
                self.field[((xy.y + i as u32) * FIELD_WIDTH + (xy.x + j as u32)) as usize] = tetros.field[tetros.current_rotation as usize][i * 4 + j];
            }
        }
    }

    pub fn move_piece_left(self: &mut Game) {
        // clean old place
        self.clean_old_position();
        // move one case to left
        if self.current_position.x > 0 {
            self.current_position.x -= 1;
        }
        // redraw
        self.draw();
    }

    pub fn move_piece_right(self: &mut Game) {
        // clean old place
        self.clean_old_position();
        // move one case to right
        if self.current_position.x < (FIELD_WIDTH - 4) {
            self.current_position.x += 1;
        }
        // redraw
        self.draw();
    }

    pub fn rotate_piece(self: &mut Game) {
        // clean old place
        self.clean_old_position();
        // change rotation
        self.current_tetromino.current_rotation += 1;
        if self.current_tetromino.current_rotation == 4 {
            self.current_tetromino.current_rotation = 0;
        }
        // redraw
        self.draw();
    }

    fn clean_old_position(self: &mut Game) {
        let xy: &Point = &self.current_position;
        for i in 0..4 {
            for j in 0..4 {
                self.field[((xy.y + i as u32) * FIELD_WIDTH + (xy.x + j as u32)) as usize] = 0;
            }
        }
    }

    pub fn check_full_lines(&self) -> Vec<u32> {
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

    fn register_tetromino(self: &mut TetrominoFactory, tetros: [i16; 4]) {
        let color = 0xFF0000FF;
        let mut field: [[u32; 16]; 4] = [[0; 16]; 4];
        for index in 0..4 {
            let tetros_value = tetros[index];
            for i in 0..16 {
                let local_tetros = tetros_value >> (15 - i); // From the "beginning" of the number
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
}

impl Default for Tetromino {
    fn default() -> Tetromino {
        let empty : [[u32; 16]; 4] = [[0; 16]; 4];
        Tetromino::new(empty, 0x00000000)
    }
}

struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn new(px: u32, py: u32) -> Point {
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
