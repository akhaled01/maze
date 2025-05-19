
// use nalgebra::{Matrix4, Perspective3, Point3, Vector3};
// use sdl2::{event::Event, image::InitFlag};
// use sdl2::keyboard::Keycode;
// use sdl2::pixels::Color;
// use std::time::{Duration, Instant};

// mod state;
// mod graphics;
// mod input;
// mod utils;
// mod mesh;
// mod shader;

// use state::{Player, MAZE_EASY};
// use graphics::render;
// use input::handle_input;

// fn main() -> Result<(), String> {
//     // SDL2 Init
//     let sdl_context = sdl2::init()?;
//     let video_subsystem = sdl_context.video()?;
//     let window = video_subsystem
//         .window("Ropher Faggots", 1280, 720)
//         .position_centered()
//         .opengl()
//         .build()
//         .map_err(|e| e.to_string())?;

//     // let _gl_context = window.gl_create_context()?;
//     // let gl_loader = |s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void;
//     // gl::load_with(gl_loader);

//     let mut canvas = window
//         .into_canvas()
//         .accelerated()
//         .present_vsync() // this will cap FPS to display's refresh rate
//         .build()
//         .map_err(|e| e.to_string())?;

//     let _gl_context = canvas.window().gl_create_context()?;
//     let gl_loader = |s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void;
//     gl::load_with(gl_loader);

//     let mut event_pump = sdl_context.event_pump()?;

//     // Enable relative mouse mode (captures mouse and hides cursor)
//     let mouse_util = sdl_context.mouse();
//     mouse_util.set_relative_mouse_mode(true);

//     // Ground Mesh - VAO, VBO, EBO, Textures, Shaders
//     let ground_mesh = mesh::load_obj("assets/models/ground.obj");
//     let ground_gl_ctx = unsafe { graphics::init_gl_context(&ground_mesh) };

//     // Initialize PBR shaders
//     let pbr_vert = std::fs::read_to_string("assets/shaders/pbr.vert").map_err(|e| e.to_string())?;
//     let pbr_frag = std::fs::read_to_string("assets/shaders/pbr.frag").map_err(|e| e.to_string())?;
//     let pbr_shader_program = unsafe { graphics::create_program(&pbr_vert, &pbr_frag) };

//     // Initial player state
    // let mut player = Player {
    //     x: 12.0,
    //     y: 12.0,
    //     dir_x: -1.0,
    //     dir_y: 0.0,
    //     plane_x: 0.0,
    //     plane_y: 0.66, // FOV = 66 degrees
    // };

//     // Timing for FPS counter
//     let mut last_frame = Instant::now();
//     let mut fps_timer = Instant::now();
//     let mut frame_count = 0;
//     let mut current_fps = 0;

//     // Initialize Skybox Settings
//     let _image_context = sdl2::image::init(InitFlag::PNG)?;
//     let texture_creator = canvas.texture_creator();
//     let skybox = graphics::Skybox::load(&texture_creator)?;

//     // Game Loop
//     'running: loop {
//         let now = Instant::now();
//         let delta_time = now.duration_since(last_frame).as_secs_f64();
//         last_frame = now;

//         // Handle input
//         for event in event_pump.poll_iter() {
//             if let Event::Quit { .. }
//             | Event::KeyDown {
//                 keycode: Some(Keycode::Escape),
//                 ..
//             } = event
//             {
//                 break 'running;
//             }
//         }
        
//         let keyboard_state = event_pump.keyboard_state();
//         let mouse_state = event_pump.relative_mouse_state();
//         handle_input(&keyboard_state, &mut player, delta_time, &MAZE_EASY, mouse_state.x());

//         // // Camera setup
//         // let projection = Perspective3::new(1280.0 / 720.0, 45.0f32.to_radians(), 0.1, 100.0).to_homogeneous();
//         // let view = Matrix4::look_at_rh(
//         //     &Point3::new(player.x as f32, 2.0, player.y as f32 + 5.0), // Camera position
//         //     &Point3::new(player.x as f32, 0.0, player.y as f32),      // Looking at player
//         //     &Vector3::y_axis(),                                        // Up direction
//         // );
//         // let model = Matrix4::identity();

//         // Clear screen
//         canvas.set_draw_color(Color::RGB(0, 0, 0));
//         canvas.clear();

//         // Explicitly tell SDL2 that you're using OpenGL context for rendering
//         canvas.window().gl_make_current(&_gl_context)?;

//         // // Activate shader and bind textures (place exactly here 👇)
//         // unsafe {
//         //     graphics::render_ground(&ground_gl_ctx, (player.x as f32, player.y as f32));
//         // }

//         // Draw world using raycasting
//         render(&mut canvas, &player, &MAZE_EASY, current_fps, &skybox, &ground_gl_ctx);

//         // Present to screen
//         canvas.present();

//         // FPS counter
//         frame_count += 1;
//         if fps_timer.elapsed() >= Duration::from_secs(1) {
//             current_fps = frame_count;
//             frame_count = 0;
//             fps_timer = Instant::now();
//         }

//         std::thread::sleep(Duration::from_millis(1));
//     }

//     Ok(())
// }

mod state;
mod graphics;
mod input;
mod utils;
mod mesh;
mod shader;

use nalgebra::{Matrix4, Perspective3, Point3, Vector3};
use sdl2::{event::Event, keyboard::Keycode, video::GLProfile};
use std::time::{Duration, Instant};
use crate::graphics::{GLContext, WeaponContext, SkyboxContext};
use crate::graphics::{render_ground, render_weapon, render_skybox};
use crate::graphics::{draw_minimap_gl, draw_fps_gl};
use crate::state::{Player, MAP_WIDTH, MAP_HEIGHT};
use crate::mesh::Mesh;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let window = video_subsystem
        .window("OpenGL Game", 1280, 720)
        .opengl()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    let _gl_context = window.gl_create_context().map_err(|e| e.to_string())?;
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as _);

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::CULL_FACE);
        gl::CullFace(gl::BACK);
        gl::Viewport(0, 0, 1280, 720);
    }

    let mut event_pump = sdl_context.event_pump()?;

    // Load assets
    let ground_mesh = mesh::load_obj("assets/models/ground.obj");
    let weapon_mesh = mesh::load_obj("assets/models/weapons/smg/smg.obj");
    let ground_ctx = unsafe { graphics::init_gl_context(&ground_mesh) };
    let weapon_ctx = unsafe { graphics::init_weapon_context(&weapon_mesh) };
    let skybox_ctx = unsafe { graphics::init_skybox() };

    let mut player = Player {
        x: 12.0,
        y: 12.0,
        dir_x: -1.0,
        dir_y: 0.0,
        plane_x: 0.0,
        plane_y: 0.66, // FOV = 66 degrees
    };
    let mut last_frame = Instant::now();
    let mut frame_count = 0;
    let mut fps_timer = Instant::now();
    let mut current_fps = 0;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }

        // Update logic here (input, movement, etc)

        // Render
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            let projection = Perspective3::new(1280.0 / 720.0, 45.0f32.to_radians(), 0.1, 100.0).to_homogeneous();
            let view = Matrix4::look_at_rh(
                &Point3::new(player.x as f32, 2.0, player.y as f32 + 5.0),
                &Point3::new(player.x as f32, 0.0, player.y as f32),
                &Vector3::y_axis(),
            );

            render_skybox(&skybox_ctx, projection, view);
            render_ground(&ground_ctx, (player.x as f32, player.y as f32));
            render_weapon(&weapon_ctx);
            // draw_minimap_gl(&player, &[[0u8; MAP_WIDTH]; MAP_HEIGHT]);
            // draw_fps_gl(current_fps);
        }

        window.gl_swap_window();

        frame_count += 1;
        if fps_timer.elapsed() >= Duration::from_secs(1) {
            current_fps = frame_count;
            frame_count = 0;
            fps_timer = Instant::now();
        }

        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
