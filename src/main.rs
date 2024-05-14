use std::time::{Duration, Instant};

use sdl2::{event::Event, keyboard::Keycode, pixels::Color, EventPump};

const WINDOW_TITLE: &str = "Cellular Automata Demo";
const WINDOW_SIZE_HEIGHT: u32 = 800;
const WINDOW_SIZE_WIDTH: u32 = 1200;
const TARGET_FPS: u128 = 60;

type CAResult<T> = Result<T, String>;

enum Command {
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

    let target_duration = 1000 / TARGET_FPS;

    loop {
        let frame_start = Instant::now();
        if let Some(command) = get_events(&mut event_pump) {
            match command {
                Command::Exit => break,
            }
        }
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        // TODO: rendering code
        canvas.present();
        // delay the rest of the time if needed
        let suspend_ms = target_duration.saturating_sub(frame_start.elapsed().as_millis());
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

            _ => {}
        }
    }
    None
}
