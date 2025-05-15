// use sdl2::{event::Event, image::InitFlag};
// use sdl2::keyboard::Keycode;
// use sdl2::pixels::Color;
// use std::time::{Duration, Instant};

// mod state;
// mod graphics;
// mod input;
// mod utils;
// mod mesh;

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

//     let _gl_context = window.gl_create_context()?;
//     let gl_loader = |s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void;
//     gl::load_with(gl_loader);

//     let mut canvas = window
//         .into_canvas()
//         .accelerated()
//         .present_vsync() // this will cap FPS to display's refresh rate
//         .build()
//         .map_err(|e| e.to_string())?;

//     let mut event_pump = sdl_context.event_pump()?;

//     // Enable relative mouse mode (captures mouse and hides cursor)
//     let mouse_util = sdl_context.mouse();
//     mouse_util.set_relative_mouse_mode(true);

//     // Ground shaders
//     let ground_vert_src = std::fs::read_to_string("assets/shaders/ground.vert").map_err(|e| e.to_string())?;
//     let ground_frag_src = std::fs::read_to_string("assets/shaders/ground.frag").map_err(|e| e.to_string())?;
//     let ground_shader_program = unsafe { graphics::create_program(&ground_vert_src, &ground_frag_src) };

//     // Ground textures settings
//     let ground_basecolor;
//     let ground_roughness;
//     let ground_normal;
//     let ground_height;
//     unsafe {
//         ground_basecolor = utils::load_texture("assets/textures/ground/ground_basecolor.png", gl::TEXTURE0);
//         ground_roughness = utils::load_texture("assets/textures/ground/ground_roughness.png", gl::TEXTURE1);
//         ground_normal = utils::load_texture("assets/textures/ground/ground_normal.png", gl::TEXTURE2);
//         ground_height = utils::load_texture("assets/textures/ground/ground_height.png", gl::TEXTURE3);
//     }

//     // Initialize ground mesh
//     let ground_mesh = mesh::load_obj("assets/models/ground.obj");

//     let mut vao = 0;
//     let mut vbo = 0;
//     let mut ebo = 0;

//     unsafe {
//         gl::GenVertexArrays(1, &mut vao);
//         gl::GenBuffers(1, &mut vbo);
//         gl::GenBuffers(1, &mut ebo);

//         gl::BindVertexArray(vao);

//         // Buffer data
//         let vertex_data: Vec<f32> = ground_mesh.vertices.iter().zip(&ground_mesh.normals).zip(&ground_mesh.texcoords)
//             .flat_map(|((v, n), t)| vec![v.x, v.y, v.z, n.x, n.y, n.z, t.x, t.y])
//             .collect();

//         gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
//         gl::BufferData(gl::ARRAY_BUFFER, 
//             (vertex_data.len() * std::mem::size_of::<f32>()) as isize, 
//             vertex_data.as_ptr() as *const _, 
//             gl::STATIC_DRAW
//         );

//         gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
//         gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
//             (ground_mesh.indices.len() * std::mem::size_of::<u32>()) as isize,
//             ground_mesh.indices.as_ptr() as *const _,
//             gl::STATIC_DRAW
//         );

//         let stride = 8 * std::mem::size_of::<f32>() as i32;

//         // Position
//         gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
//         gl::EnableVertexAttribArray(0);
//         // Normal
//         gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * std::mem::size_of::<f32>()) as *const _);
//         gl::EnableVertexAttribArray(1);
//         // Texcoords
//         gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, stride, (6 * std::mem::size_of::<f32>()) as *const _);
//         gl::EnableVertexAttribArray(2);

//         gl::BindVertexArray(0);
//     }

//     // Initialize PBR shaders
//     let pbr_vert = std::fs::read_to_string("assets/shaders/pbr.vert").map_err(|e| e.to_string())?;
//     let pbr_frag = std::fs::read_to_string("assets/shaders/pbr.frag").map_err(|e| e.to_string())?;
//     let pbr_shader_program = unsafe { graphics::create_program(&pbr_vert, &pbr_frag) };

//     // Initial player state
//     let mut player = Player {
//         x: 12.0,
//         y: 12.0,
//         dir_x: -1.0,
//         dir_y: 0.0,
//         plane_x: 0.0,
//         plane_y: 0.66, // FOV = 66 degrees
//     };

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

//         // Clear screen
//         canvas.set_draw_color(Color::RGB(0, 0, 0));
//         canvas.clear();

//         // Activate shader and bind textures (place exactly here 👇)
//         unsafe {
//             gl::UseProgram(ground_shader_program);

//             gl::Uniform1i(gl::GetUniformLocation(ground_shader_program, "baseTexture\0".as_ptr() as *const i8), 0);
//             gl::Uniform1i(gl::GetUniformLocation(ground_shader_program, "roughnessTexture\0".as_ptr() as *const i8), 1);
//             gl::Uniform1i(gl::GetUniformLocation(ground_shader_program, "normalTexture\0".as_ptr() as *const i8), 2);
//             gl::Uniform1i(gl::GetUniformLocation(ground_shader_program, "heightTexture\0".as_ptr() as *const i8), 3);
//         }

//         // PBR Rendering
//         unsafe {
//             gl::UseProgram(pbr_shader_program);
        
//             gl::Uniform3f(gl::GetUniformLocation(pbr_shader_program, "camPos\0".as_ptr() as *const _), player.x as f32, 1.0, player.y as f32);
//             gl::Uniform3f(gl::GetUniformLocation(pbr_shader_program, "lightPos\0".as_ptr() as *const _), 10.0, 10.0, 10.0);
//             gl::Uniform3f(gl::GetUniformLocation(pbr_shader_program, "lightColor\0".as_ptr() as *const _), 1.0, 1.0, 1.0);
        
//             // Bind Textures
//             gl::ActiveTexture(gl::TEXTURE0);
//             gl::BindTexture(gl::TEXTURE_2D, ground_basecolor);
//             gl::Uniform1i(gl::GetUniformLocation(pbr_shader_program, "albedoMap\0".as_ptr() as *const _), 0);
        
//             gl::ActiveTexture(gl::TEXTURE1);
//             gl::BindTexture(gl::TEXTURE_2D, ground_normal);
//             gl::Uniform1i(gl::GetUniformLocation(pbr_shader_program, "normalMap\0".as_ptr() as *const _), 1);
        
//             gl::ActiveTexture(gl::TEXTURE2);
//             gl::BindTexture(gl::TEXTURE_2D, ground_roughness);
//             gl::Uniform1i(gl::GetUniformLocation(pbr_shader_program, "roughnessMap\0".as_ptr() as *const _), 2);
        
//             gl::BindVertexArray(vao);
//             gl::DrawElements(gl::TRIANGLES, ground_mesh.indices.len() as i32, gl::UNSIGNED_INT, std::ptr::null());
//         }

//         // Draw world using raycasting
//         render(&mut canvas, &player, &MAZE_EASY, current_fps, &skybox);

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


use nalgebra::{Matrix4, Perspective3, Point3, Vector3};
use sdl2::{event::Event, image::InitFlag};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::{Duration, Instant};

mod state;
mod graphics;
mod input;
mod utils;
mod mesh;

use state::{Player, MAZE_EASY};
use graphics::render;
use input::handle_input;

fn main() -> Result<(), String> {
    // SDL2 Init
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Ropher Faggots", 1280, 720)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    // let _gl_context = window.gl_create_context()?;
    // let gl_loader = |s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void;
    // gl::load_with(gl_loader);

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync() // this will cap FPS to display's refresh rate
        .build()
        .map_err(|e| e.to_string())?;

    let _gl_context = canvas.window().gl_create_context()?;
    let gl_loader = |s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void;
    gl::load_with(gl_loader);

    let mut event_pump = sdl_context.event_pump()?;

    // Enable relative mouse mode (captures mouse and hides cursor)
    let mouse_util = sdl_context.mouse();
    mouse_util.set_relative_mouse_mode(true);

    // Ground Mesh - VAO, VBO, EBO, Textures, Shaders
    let ground_mesh = mesh::load_obj("assets/models/ground.obj");
    let ground_gl_ctx = unsafe { graphics::init_gl_context(&ground_mesh) };

    // Initialize PBR shaders
    let pbr_vert = std::fs::read_to_string("assets/shaders/pbr.vert").map_err(|e| e.to_string())?;
    let pbr_frag = std::fs::read_to_string("assets/shaders/pbr.frag").map_err(|e| e.to_string())?;
    let pbr_shader_program = unsafe { graphics::create_program(&pbr_vert, &pbr_frag) };

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

    // Initialize Skybox Settings
    let _image_context = sdl2::image::init(InitFlag::PNG)?;
    let texture_creator = canvas.texture_creator();
    let skybox = graphics::Skybox::load(&texture_creator)?;

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

        // // Camera setup
        // let projection = Perspective3::new(1280.0 / 720.0, 45.0f32.to_radians(), 0.1, 100.0).to_homogeneous();
        // let view = Matrix4::look_at_rh(
        //     &Point3::new(player.x as f32, 2.0, player.y as f32 + 5.0), // Camera position
        //     &Point3::new(player.x as f32, 0.0, player.y as f32),      // Looking at player
        //     &Vector3::y_axis(),                                        // Up direction
        // );
        // let model = Matrix4::identity();

        // Clear screen
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Explicitly tell SDL2 that you're using OpenGL context for rendering
        canvas.window().gl_make_current(&_gl_context)?;

        // // Activate shader and bind textures (place exactly here 👇)
        // unsafe {
        //     graphics::render_ground(&ground_gl_ctx, (player.x as f32, player.y as f32));
        // }

        // Draw world using raycasting
        render(&mut canvas, &player, &MAZE_EASY, current_fps, &skybox, &ground_gl_ctx);

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
