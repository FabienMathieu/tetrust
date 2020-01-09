use sfml::graphics::{Color, PrimitiveType, RenderTarget, RenderWindow, Vertex, Shape, RectangleShape, Transformable};
use sfml::window::{Event, Key, Style, VideoMode};
use sfml::system::{Vector2f};

mod tetrust;

fn main() {
    let mut game = tetrust::game::Game::new();
    game.init();

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
        VideoMode::new(
            10 * tetrust::game::TETRO_SIZE,
            20 * tetrust::game::TETRO_SIZE,
            32,
        ),
        "Rust Tetris",
        Style::CLOSE,
        &Default::default(),
    );

    window.set_framerate_limit(60);

    game.update();

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed
                | Event::KeyReleased {
                    code: Key::Escape, ..
                } => window.close(),
                Event::KeyReleased {
                    code: Key::Left, ..
                } => game.move_piece_left(),
                Event::KeyReleased {
                    code: Key::Right, ..
                } => game.move_piece_right(),
                Event::KeyReleased {
                    code: Key::Space, ..
                } => game.rotate_piece(),
                _ => (),
            };
        }

        window.clear(Color::WHITE);

        window.draw_primitives(&vertices, PrimitiveType::Lines, Default::default());

        draw_game(&mut window, &game);

        window.display();
    }
}

fn draw_game(window: &mut RenderWindow, game: &tetrust::game::Game) {
    let field = game.get_field();
    for i in 0..tetrust::game::FIELD_HEIGHT {
        for j in 0..tetrust::game::FIELD_WIDTH {
            //println!("{},{}", i, j);
            if field[(i * tetrust::game::FIELD_WIDTH + j) as usize] != 0 /*|| (i == 2 && j == 3)*/ {
                //println!("get in");
                let mut tetros_block = RectangleShape::new();
                tetros_block.set_size(
                    Vector2f::new(tetrust::game::TETRO_SIZE as f32, tetrust::game::TETRO_SIZE as f32),
                );
                tetros_block.set_position(
                    Vector2f::new((j * tetrust::game::TETRO_SIZE) as f32, (i* tetrust::game::TETRO_SIZE) as f32),
                );
                tetros_block.set_fill_color(
                  Color::rgb(255, 0, 0),
                );
                window.draw(&tetros_block/*, RenderStates::default()*/);
            }
        }
    }
}
