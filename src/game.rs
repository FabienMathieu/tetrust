pub const FIELD_WIDTH: u32 = 10;
pub const FIELD_HEIGHT: u32 = 20;
pub const TETRO_SIZE: u32 = 20;
pub const BLOCKS_NUMBER: u32 = (FIELD_WIDTH - 1) * 2 + (FIELD_HEIGHT - 1) * 2;

pub struct Game {
    field: [u32; (FIELD_HEIGHT * FIELD_WIDTH) as usize],
}

impl Game {
    pub fn new() -> Game {
        Game{
            field: [0; (FIELD_HEIGHT * FIELD_WIDTH) as usize],
        }
    }

    pub fn check_full_lines(&self) -> Vec< u32 > {
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