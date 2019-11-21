use sfml::graphics::{Color, PrimitiveType, RenderTarget, RenderWindow, Vertex};
use sfml::window::{Event, Key, Style};

mod tetrust;

fn main() {
    let mut vertices = Vec::with_capacity(tetrust::game::BLOCKS_NUMBER as usize);

    for i in 1..tetrust::game::FIELD_WIDTH {
        vertices.push(Vertex::new(
            ((i * tetrust::game::TETRO_SIZE) as f32, 0.0),
            Color::BLACK,
            Default::default(),
        ));
        vertices.push(Vertex::new(
            (
                (i * tetrust::game::TETRO_SIZE) as f32,
                (tetrust::game::FIELD_HEIGHT * tetrust::game::TETRO_SIZE) as f32,
            ),
            Color::BLACK,
            Default::default(),
        ));
    }

    for i in 1..tetrust::game::FIELD_HEIGHT {
        vertices.push(Vertex::new(
            (0.0, (i * tetrust::game::TETRO_SIZE) as f32),
            Color::BLACK,
            Default::default(),
        ));
        vertices.push(Vertex::new(
            (
                (tetrust::game::FIELD_WIDTH * tetrust::game::TETRO_SIZE) as f32,
                (i * tetrust::game::TETRO_SIZE) as f32,
            ),
            Color::BLACK,
            Default::default(),
        ));
    }

    let mut window = RenderWindow::new(
        (
            10 * tetrust::game::TETRO_SIZE,
            20 * tetrust::game::TETRO_SIZE,
        ),
        "Rust Tetris",
        Style::CLOSE,
        &Default::default(),
    );

    window.set_framerate_limit(60);

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed
                | Event::KeyReleased {
                    code: Key::Escape, ..
                } => window.close(),
                _ => (),
            };
        }

        window.clear(Color::WHITE);

        window.draw_primitives(&vertices, PrimitiveType::Lines, Default::default());

        window.display();
    }
}
