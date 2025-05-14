use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::{Duration, Instant};

mod state;
mod graphics;
mod input;

use state::{Player, MAZE_EASY};
use graphics::render;
use input::handle_input;

fn main() -> Result<(), String> {
    // SDL2 Init
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Ropher", 1280, 720)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync() // this will cap FPS to display's refresh rate
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    // Enable relative mouse mode (captures mouse and hides cursor)
    let mouse_util = sdl_context.mouse();
    mouse_util.set_relative_mouse_mode(true);

    // Initial player state
    let mut player = Player {
        x: 12.0,
        y: 12.0,
        dir_x: -1.0,
        dir_y: 0.0,
        plane_x: 0.0,
        plane_y: 0.66, // FOV = 66 degrees
    };

    // Timing for FPS counter
    let mut last_frame = Instant::now();
    let mut fps_timer = Instant::now();
    let mut frame_count = 0;
    let mut current_fps = 0;

    // Game Loop
    'running: loop {
        let now = Instant::now();
        let delta_time = now.duration_since(last_frame).as_secs_f64();
        last_frame = now;

        // Handle input
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } = event
            {
                break 'running;
            }
        }
        
        let keyboard_state = event_pump.keyboard_state();
        let mouse_state = event_pump.relative_mouse_state();
        handle_input(&keyboard_state, &mut player, delta_time, &MAZE_EASY, mouse_state.x());

        // Clear screen
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Draw world using raycasting
        render(&mut canvas, &player, &MAZE_EASY, current_fps);

        // Present to screen
        canvas.present();

        // FPS counter
        frame_count += 1;
        if fps_timer.elapsed() >= Duration::from_secs(1) {
            current_fps = frame_count;
            frame_count = 0;
            fps_timer = Instant::now();
        }

        std::thread::sleep(Duration::from_millis(1));
    }

    Ok(())
}
