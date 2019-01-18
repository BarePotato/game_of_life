use sfml::graphics::{
    Color, RectangleShape, RenderStates, RenderTarget, RenderWindow, Shape, Transformable,
};
use sfml::system::Vector2f;
use sfml::window::{mouse, Event, Style};

use rand::{thread_rng, Rng};

use std::time::{Duration, Instant};

fn main() {
    // interface
    let win_w = 1600;
    let win_h = 900;
    let mut win = RenderWindow::new(
        (win_w, win_h),
        "Game of Life",
        Style::CLOSE,
        &Default::default(),
    );

    // FPS counter
    let mut fps = 0;
    let mut frames = 0;
    let mut frame_timer = Instant::now();
    let frame_rate = Duration::from_millis(333);

    // game states
    let scale = 2; // defines grid size. 1 to smallest_dimension
    let step = 10;
    let mut tick = Duration::from_millis(100);
    let mut first_run = true;
    let mut timer = Instant::now();
    let mut generation = true;
    let mut cell = RectangleShape::with_size(Vector2f::new(scale as f32, scale as f32));
    cell.set_fill_color(&Color::WHITE);

    // game
    let grid_w = win_w as usize / scale;
    let grid_h = win_h as usize / scale;
    let mut gen_last = vec![vec![0; grid_w]; grid_h];
    let mut gen_next = vec![vec![0; grid_w]; grid_h];

    fn gen_default(gen: &mut Vec<Vec<usize>>) {
        let mut rng = thread_rng();

        for row in gen.iter_mut() {
            for col in row.iter_mut() {
                *col = rng.gen_range(0, 2);
            }
        }
    };

    gen_default(&mut gen_last);

    // loop
    #[allow(unused_variables)]
    while win.is_open() {
        while let Some(event) = win.poll_event() {
            match event {
                Event::Closed => win.close(),
                Event::MouseButtonPressed { button, x, y } => {
                    if button == mouse::Button::Right {
                        gen_default(&mut gen_last);
                        first_run = true;
                    }
                }
                Event::MouseWheelScrolled { wheel, delta, x, y } => {
                    if delta as isize > 0 {
                        tick += Duration::from_millis(step);
                    } else if 0 > delta as isize && tick >= Duration::from_millis(step) {
                        tick -= Duration::from_millis(step);
                    }
                }
                _ => {}
            }
        }

        if frame_timer.elapsed() >= frame_rate {
            fps = frames
                * (1000.
                    / (frame_rate.as_secs() as f64 * 1000. + f64::from(frame_rate.subsec_millis())))
                    as usize;
            frames = 0;
            frame_timer = Instant::now();
        }

        win.set_title(format!("Game of Life - {:?} - {} fps", tick, fps).as_str());

        if timer.elapsed() < tick && !first_run {
            continue;
        }

        // update grid
        let mut gen_now = &mut gen_last;
        let gen_future = if !generation && !first_run {
            gen_now = &mut gen_next;
            &mut gen_last
        } else {
            &mut gen_next
        };

        for (y, row) in gen_future.iter_mut().enumerate() {
            for (x, col) in row.iter_mut().enumerate() {
                let mut count = 0;
                for pos_y in -1..2 {
                    for pos_x in -1..2 {
                        let yy = (y as isize + grid_h as isize + pos_y) as usize % grid_h;
                        let xx = (x as isize + grid_w as isize + pos_x) as usize % grid_w;

                        count += gen_now[yy][xx];
                    }
                }

                count -= gen_now[y][x];
                match gen_now[y][x] {
                    0 if count == 3 => *col = 1,
                    1 if count < 2 || count > 3 => *col = 0,
                    1 if count == 2 || count == 3 => *col = 1,
                    _ => *col = gen_now[y][x],
                }
            }
        }
        generation = !generation;

        win.clear(&Color::BLACK);
        // draw grid
        for (y, row) in gen_future.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if *col == 1 {
                    cell.set_position(((x * scale) as f32, (y * scale) as f32));
                    win.draw_rectangle_shape(&cell, RenderStates::default());
                }
            }
        }
        win.display();

        if first_run {
            first_run = false;
        }

        timer = Instant::now();

        frames += 1;
    }
}