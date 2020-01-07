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
}

impl Game {
    pub fn new() -> Game {
        Game {
            field: [0; (FIELD_HEIGHT * FIELD_WIDTH) as usize],
            factory: TetrominoFactory::new(),
            current_tetromino: Tetromino::default(),
        }
    }

    pub fn get_field(self: &Game) -> &[u32; (FIELD_HEIGHT * FIELD_WIDTH) as usize] {
        return &self.field;
    }

    pub fn init(self: &mut Game) {
        for i in 0..7 {
            self.factory.register_tetromino(TETROS[i]);
        }
        //self.factory.register_tetromino(TETROS[0]);
    }
    
    pub fn update(self: &mut Game) {
        //if newone {}
        let tetros_index = rand::thread_rng().gen_range(0, 7);
        println!("index = {}", tetros_index);
        let tetros = self.factory.create(tetros_index);
        self.current_tetromino = tetros;
        //let tetros = self.factory.create(tetros_index);
        /*for i in 0..16 {
            if i % 4 == 0 {
                print!("\n");
            }
            print!("{}\t", tetros.field[0][i]);
        }*/
        //self.current_tetromino.draw(self, Point::new(FIELD_WIDTH / 2 - 2, 0));  // -2 because of the size of tetros matrix (=4)
        self.draw(Point::new(FIELD_WIDTH / 2 - 2, 0));
   /* } else {
        self.draw(self.current_tetromino, xy)
    }*/
    }

    fn draw(self: &mut Game, xy: Point) {
        let tetros: &Tetromino = &self.current_tetromino;
        for i in 0..4 {
            for j in 0..4 {
                //print!("{}\t", self.field[self.current_rotation as usize][i * 4 + j]);
                self.field[((xy.y + i as u32) * FIELD_WIDTH + (xy.x + j as u32)) as usize] = tetros.field[tetros.current_rotation as usize][i * 4 + j];
            }
            //print!("\n");
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
                //local_tetros = local_tetros >> 1;
            }
        }
        /*let mut local_tetros = tetros[0];
        println!("{}", local_tetros);
        for i in 0..16 {
            println!("i:{} => {:b}", i, local_tetros);
            field[0][i] = local_tetros as u32 & 0x1;
            local_tetros = local_tetros >> 1;
        }*/
        

        let tetromino = Tetromino::new(field, color);

        /*for i in 0..16 {
            if i % 4 == 0 {
                print!("\n");
            }
            print!("{}\t", tetromino.field[0][i]);
        }
        print!("\n");*/

        /*println!("tetros {}", self.tetrominos.len());
        for i in 0..16 {
            if i % 4 == 0 {
                print!("\n");
            }
            print!("{}\t", tetromino.field[0][i]);
        }*/

        self.tetrominos.push(tetromino);
    }
}

impl ParameterizedFactory for TetrominoFactory {
    type Item = Tetromino;

    type Parameter = u8;
    fn create(self: &TetrominoFactory, param: Self::Parameter) -> Self::Item {
       /* println!("in create : {}", param);
        println!("tetrominos size : {}", self.tetrominos.len());*/
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

    fn draw(self: &mut Tetromino, game: &mut Game, xy: Point) {
        for i in 0..16 {
            if i % 4 == 0 {
                print!("\n");
            }
            print!("{}\t", self.field[0][i]);
        }
        print!("\n");
        for i in 0..4 {
            for j in 0..4 {
                //print!("{}\t", self.field[self.current_rotation as usize][i * 4 + j]);
                game.field[((xy.y + i as u32) * FIELD_WIDTH + (xy.x + j as u32)) as usize] = self.field[self.current_rotation as usize][i * 4 + j];
            }
            //print!("\n");
        }

        for i in 0..FIELD_HEIGHT {
            for j in 0..FIELD_WIDTH {
                print!("{}\t", game.field[(i * FIELD_WIDTH + j) as usize]);
                if j == FIELD_WIDTH - 1 {
                    print!("\n");
                }
            }
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
