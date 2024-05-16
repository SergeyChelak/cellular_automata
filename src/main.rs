mod generator;
mod matrix;

use std::time::{Duration, Instant};

use generator::Generator;
use matrix::Position;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::{Color, PixelFormatEnum},
    rect::Rect,
    render::{Texture, TextureCreator},
    video::WindowContext,
    EventPump,
};

const WINDOW_TITLE: &str = "Cellular Automata Demo";
const WINDOW_SIZE_HEIGHT: u32 = 800;
const WINDOW_SIZE_WIDTH: u32 = 800;
const TARGET_FPS: u128 = 60;
const TARGET_FRAME_DURATION: u128 = 1000 / TARGET_FPS;

type CAColor = (u8, u8, u8);

const CONTOUR_SATURATION: u8 = 70;
const DEFAULT_COLOR: CAColor = (0xff, 0xff, 0xff);
const REGION_COLOR: [CAColor; 20] = [
    (255, 192, 64),
    (64, 255, 192),
    (192, 64, 255),
    (255, 224, 32),
    (96, 255, 160),
    (64, 192, 255),
    (255, 96, 160),
    (160, 255, 96),
    (96, 160, 255),
    (255, 32, 224),
    (224, 255, 32),
    (32, 96, 255),
    (255, 160, 96),
    (32, 255, 224),
    (192, 255, 64),
    (64, 255, 224),
    (255, 64, 192),
    (224, 32, 255),
    (160, 96, 255),
    (255, 32, 96),
];

type CAResult<T> = Result<T, String>;

enum Command {
    IncreaseNoiseDensity,
    DecreaseNoiseDensity,
    IncreaseIterations,
    NextIteration,
    DecreaseIterations,
    Regenerate,
    ShowStatus,
    Filter,
    Exit,
}

fn main() -> CAResult<()> {
    let sdl = sdl2::init()?;
    let video_subsystem = sdl.video()?;
    let window = video_subsystem
        .window(WINDOW_TITLE, WINDOW_SIZE_WIDTH, WINDOW_SIZE_HEIGHT)
        .position_centered()
        .build()
        .map_err(|err| err.to_string())?;
    let mut canvas = window
        .into_canvas()
        .accelerated()
        .target_texture()
        .present_vsync()
        .build()
        .map_err(|err| err.to_string())?;
    let mut event_pump = sdl.event_pump().map_err(|err| err.to_string())?;
    let mut generator = Generator::new();
    generator.regenerate();

    let texture_creator = canvas.texture_creator();
    let dest = Rect::new(0, 0, WINDOW_SIZE_WIDTH, WINDOW_SIZE_HEIGHT);
    loop {
        let frame_start = Instant::now();
        if let Some(command) = get_events(&mut event_pump) {
            use Command::*;
            match command {
                Exit => break,
                DecreaseIterations => generator.decrease_iterations(),
                IncreaseIterations => generator.increase_iterations(),
                DecreaseNoiseDensity => generator.decrease_noise_density(),
                IncreaseNoiseDensity => generator.increase_noise_density(),
                Regenerate => generator.regenerate(),
                NextIteration => generator.next_iteration(),
                Filter => generator.filter(),
                ShowStatus => println!("{}", get_status_bar(&generator)),
            }
        }
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        let texture = create_texture(&generator, &texture_creator)?;
        let query = texture.query();
        let src = Rect::new(0, 0, query.width, query.height);
        canvas.copy(&texture, src, dest)?;
        canvas.present();
        // delay the rest of the time if needed
        let suspend_ms = TARGET_FRAME_DURATION.saturating_sub(frame_start.elapsed().as_millis());
        if suspend_ms > 0 {
            let duration = Duration::from_millis(suspend_ms as u64);
            std::thread::sleep(duration);
        }
    }

    Ok(())
}

fn get_events(event_pump: &mut EventPump) -> Option<Command> {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => return Some(Command::Exit),

            Event::KeyDown {
                keycode: Some(Keycode::Q),
                ..
            } => return Some(Command::IncreaseNoiseDensity),

            Event::KeyDown {
                keycode: Some(Keycode::A),
                ..
            } => return Some(Command::DecreaseNoiseDensity),

            Event::KeyDown {
                keycode: Some(Keycode::W),
                ..
            } => return Some(Command::IncreaseIterations),

            Event::KeyDown {
                keycode: Some(Keycode::S),
                ..
            } => return Some(Command::DecreaseIterations),

            Event::KeyDown {
                keycode: Some(Keycode::R),
                ..
            } => return Some(Command::Regenerate),

            Event::KeyDown {
                keycode: Some(Keycode::N),
                ..
            } => return Some(Command::NextIteration),

            Event::KeyDown {
                keycode: Some(Keycode::F),
                ..
            } => return Some(Command::Filter),

            Event::KeyDown { .. } => return Some(Command::ShowStatus),
            _ => {}
        }
    }
    None
}

fn create_texture<'l>(
    generator: &Generator,
    texture_creator: &'l TextureCreator<WindowContext>,
) -> CAResult<Texture<'l>> {
    let matrix = generator.get_matrix();
    let rows = matrix.len() as u32;
    let cols = matrix
        .first()
        .map(|x| x.len() as u32)
        .ok_or("matrix is invalid".to_string())?;
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, cols, rows)
        .map_err(|err| err.to_string())?;
    let contours = generator.get_contours();
    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        for row in 0..rows as usize {
            for col in 0..cols as usize {
                let pos = row * pitch + 3 * col;
                let mut color = (0, 0, 0);
                let position = Position { row, col };
                if matrix[row][col] > 0 {
                    color = if let Some(id) = generator.region_id(&position) {
                        REGION_COLOR[id % REGION_COLOR.len()]
                    } else {
                        DEFAULT_COLOR
                    };
                };
                if contours.contains(&position) {
                    color = saturate_color(&color, CONTOUR_SATURATION);
                }
                let (r, g, b) = color;
                buffer[pos + 0] = r;
                buffer[pos + 1] = g;
                buffer[pos + 2] = b;
            }
        }
    })?;
    Ok(texture)
}

fn get_status_bar(generator: &Generator) -> String {
    format!(
        "Noise density |Q+ {} -A|  Iterations |W+ {} -S|  R(egenerate)  N(ext iteration)",
        generator.noise_density(),
        generator.iterations()
    )
}

fn saturate_color(color: &CAColor, val: u8) -> CAColor {
    let (r, g, b) = color;
    (
        r.saturating_add(val),
        g.saturating_add(val),
        b.saturating_add(val),
    )
}
