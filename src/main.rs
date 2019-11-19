use sfml::window::{Event, Style, Key};
use sfml::graphics::{RenderWindow, RenderTarget, Color, Vertex, PrimitiveType};

mod pieces;
mod game;



fn main() {
    let mut vertices = Vec::with_capacity(game::BLOCKS_NUMBER as usize);
    

    for i in 1..game::FIELD_WIDTH {
        vertices.push(Vertex::new(((i * game::TETRO_SIZE) as f32, 0.0), Color::BLACK, Default::default()));
        vertices.push(Vertex::new(((i * game::TETRO_SIZE) as f32, (game::FIELD_HEIGHT * game::TETRO_SIZE) as f32), Color::BLACK, Default::default()));
    }

    for i in 1..game::FIELD_HEIGHT {
        vertices.push(Vertex::new((0.0, (i * game::TETRO_SIZE) as f32), Color::BLACK, Default::default()));
        vertices.push(Vertex::new(((game::FIELD_WIDTH * game::TETRO_SIZE) as f32, (i * game::TETRO_SIZE) as f32), Color::BLACK, Default::default()));
    }


    let mut window = RenderWindow::new((10 * game::TETRO_SIZE, 20 * game::TETRO_SIZE), "Rust Tetris", Style::CLOSE, &Default::default());

    window.set_framerate_limit(60);

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed  | Event::KeyReleased { code: Key::Escape, .. } => window.close(),
                _ => (),            
            };
        }

        window.clear(Color::WHITE);

        window.draw_primitives(&vertices, PrimitiveType::Lines, Default::default());

        window.display();
    }
}
