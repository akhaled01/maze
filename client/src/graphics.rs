
// use gl::types::{GLenum, GLuint};
// use sdl2::{
//     pixels::Color,
//     rect::{Point, Rect},
//     render::{Canvas, Texture, TextureCreator},
//     image::{LoadTexture, InitFlag},
//     video::{Window, WindowContext},
// };
// use crate::{mesh::Mesh, utils};
// use nalgebra::{Matrix4, Perspective3, Point3, Vector3};

// use crate::state::{MAP_HEIGHT, MAP_WIDTH};

// use crate::state::Player;

// // Shaders settings
// pub struct GLContext {
//     pub shader_program: GLuint,
//     pub vao: GLuint,
//     pub textures: [GLuint; 3],
//     pub index_count: usize,
// }

// pub unsafe fn compile_shader(src: &str, shader_type: GLenum) -> GLuint {
//     unsafe {
//         let shader = gl::CreateShader(shader_type);
//         gl::ShaderSource(shader, 1, &(src.as_ptr() as *const i8), &(src.len() as i32));
//         gl::CompileShader(shader);
//         shader
//     }
// }

// pub unsafe fn create_program(vert_src: &str, frag_src: &str) -> GLuint {
//     unsafe {
//         let vert_shader = compile_shader(vert_src, gl::VERTEX_SHADER);
//         let frag_shader = compile_shader(frag_src, gl::FRAGMENT_SHADER);

//         let program = gl::CreateProgram();
//         gl::AttachShader(program, vert_shader);
//         gl::AttachShader(program, frag_shader);
//         gl::LinkProgram(program);

//         gl::DeleteShader(vert_shader);
//         gl::DeleteShader(frag_shader);
//         program
//     }
// }

// pub unsafe fn init_gl_context(mesh: &Mesh) -> GLContext {
//     unsafe {
//         // Load and compile shaders
//         let vert_src = std::fs::read_to_string("assets/shaders/pbr.vert").expect("Vertex shader error");
//         let frag_src = std::fs::read_to_string("assets/shaders/pbr.frag").expect("Fragment shader error");
//         let shader_program = create_program(&vert_src, &frag_src);

//         // Load textures
//         let textures = [
//             utils::load_texture("assets/textures/ground/ground_basecolor.png", gl::TEXTURE0),
//             utils::load_texture("assets/textures/ground/ground_normal.png", gl::TEXTURE1),
//             utils::load_texture("assets/textures/ground/ground_roughness.png", gl::TEXTURE2),
//         ];

//         // Setup mesh VAO
//         let (mut vao, mut vbo, mut ebo) = (0, 0, 0);
//         gl::GenVertexArrays(1, &mut vao);
//         gl::GenBuffers(1, &mut vbo);
//         gl::GenBuffers(1, &mut ebo);

//         gl::BindVertexArray(vao);

//         let vertex_data: Vec<f32> = mesh
//             .vertices
//             .iter()
//             .zip(&mesh.normals)
//             .zip(&mesh.texcoords)
//             .flat_map(|((v, n), t)| vec![v.x, v.y, v.z, n.x, n.y, n.z, t.x, t.y])
//             .collect();

//         gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
//         gl::BufferData(
//             gl::ARRAY_BUFFER,
//             (vertex_data.len() * std::mem::size_of::<f32>()) as isize,
//             vertex_data.as_ptr() as *const _,
//             gl::STATIC_DRAW,
//         );

//         gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
//         gl::BufferData(
//             gl::ELEMENT_ARRAY_BUFFER,
//             (mesh.indices.len() * std::mem::size_of::<u32>()) as isize,
//             mesh.indices.as_ptr() as *const _,
//             gl::STATIC_DRAW,
//         );

//         let stride = 8 * std::mem::size_of::<f32>() as i32;

//         gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
//         gl::EnableVertexAttribArray(0);

//         gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * std::mem::size_of::<f32>()) as *const _);
//         gl::EnableVertexAttribArray(1);

//         gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, stride, (6 * std::mem::size_of::<f32>()) as *const _);
//         gl::EnableVertexAttribArray(2);

//         gl::BindVertexArray(0);

//         GLContext {
//             shader_program,
//             vao,
//             textures,
//             index_count: mesh.indices.len(),
//         }
//     }
// }

// pub struct Skybox<'a> {
//     pub front: Texture<'a>,
//     pub back: Texture<'a>,
//     pub left: Texture<'a>,
//     pub right: Texture<'a>,
//     pub up: Texture<'a>,
//     pub down: Texture<'a>,
// }

// impl<'a> Skybox<'a> {
//     pub fn load(texture_creator: &'a TextureCreator<WindowContext>) -> Result<Self, String> {
//         Ok(Self {
//             front: texture_creator.load_texture("assets/textures/skybox/front.png")?,
//             back: texture_creator.load_texture("assets/textures/skybox/back.png")?,
//             left: texture_creator.load_texture("assets/textures/skybox/left.png")?,
//             right: texture_creator.load_texture("assets/textures/skybox/right.png")?,
//             up: texture_creator.load_texture("assets/textures/skybox/up.png")?,
//             down: texture_creator.load_texture("assets/textures/skybox/down.png")?,
//         })
//     }
// }

// const MINIMAP_CELL_SIZE: u32 = 4; // Size of each cell in the minimap
// const MINIMAP_PADDING: i32 = 20; // Padding from the edges of the screen

// pub unsafe fn render_ground(gl_ctx: &GLContext, player_pos: (f32, f32)) {
//     let projection = Perspective3::new(1280.0 / 720.0, 45.0f32.to_radians(), 0.1, 100.0).to_homogeneous();
//     let view = Matrix4::look_at_rh(
//         &Point3::new(player_pos.0, 2.0, player_pos.1 + 5.0),
//         &Point3::new(player_pos.0, 0.0, player_pos.1),
//         &Vector3::y_axis(),
//     );
//     let model = Matrix4::identity();

//     unsafe {
//         gl::UseProgram(gl_ctx.shader_program);

//         gl::UniformMatrix4fv(gl::GetUniformLocation(gl_ctx.shader_program, "model\0".as_ptr() as _), 1, gl::FALSE, model.as_ptr());
//         gl::UniformMatrix4fv(gl::GetUniformLocation(gl_ctx.shader_program, "view\0".as_ptr() as _), 1, gl::FALSE, view.as_ptr());
//         gl::UniformMatrix4fv(gl::GetUniformLocation(gl_ctx.shader_program, "projection\0".as_ptr() as _), 1, gl::FALSE, projection.as_ptr());

//         for (i, texture) in gl_ctx.textures.iter().enumerate() {
//             gl::ActiveTexture(gl::TEXTURE0 + i as u32);
//             gl::BindTexture(gl::TEXTURE_2D, *texture);
//         }

//         gl::BindVertexArray(gl_ctx.vao);
//         // gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
//         gl::DrawElements(gl::TRIANGLES, gl_ctx.index_count as i32, gl::UNSIGNED_INT, std::ptr::null());
//         gl::BindVertexArray(0);
//     }
// }

// pub fn draw_skybox(
//     canvas: &mut Canvas<Window>,
//     skybox: &Skybox,
//     player: &Player,
//     screen_width: u32,
//     screen_height: u32,
// ) -> Result<(), String> {
//     let angle = player.dir_y.atan2(player.dir_x).to_degrees();
//     let texture = if angle >= -45.0 && angle < 45.0 {
//         &skybox.front
//     } else if angle >= 45.0 && angle < 135.0 {
//         &skybox.left
//     } else if angle >= -135.0 && angle < -45.0 {
//         &skybox.right
//     } else {
//         &skybox.back
//     };

//     canvas.copy(texture, None, Rect::new(0, 0, screen_width, screen_height / 2))?;
//     Ok(())
// }

// fn draw_fps(canvas: &mut Canvas<Window>, fps: u32, screen_width: u32) {
//     // Draw FPS text in the top right corner
//     canvas.set_draw_color(Color::RGB(255, 255, 0));
//     let fps_text = format!("FPS: {}", fps);
//     let text_width = fps_text.len() as i32 * 8; // Approximate width based on character count
//     let text_x = screen_width as i32 - text_width - MINIMAP_PADDING;

//     // Draw each character manually since we don't have TTF support
//     for (i, c) in fps_text.chars().enumerate() {
//         let x = text_x + i as i32 * 8;
//         let y = MINIMAP_PADDING;

//         // Simple pixel representation of characters
//         match c {
//             'F' => {
//                 canvas
//                     .draw_line(Point::new(x, y), Point::new(x + 6, y))
//                     .unwrap();
//                 canvas
//                     .draw_line(Point::new(x, y), Point::new(x, y + 8))
//                     .unwrap();
//                 canvas
//                     .draw_line(Point::new(x, y + 4), Point::new(x + 4, y + 4))
//                     .unwrap();
//             }
//             'P' => {
//                 canvas
//                     .draw_line(Point::new(x, y), Point::new(x + 6, y))
//                     .unwrap();
//                 canvas
//                     .draw_line(Point::new(x, y), Point::new(x, y + 8))
//                     .unwrap();
//                 canvas
//                     .draw_line(Point::new(x, y + 4), Point::new(x + 6, y + 4))
//                     .unwrap();
//                 canvas
//                     .draw_line(Point::new(x + 6, y), Point::new(x + 6, y + 4))
//                     .unwrap();
//             }
//             'S' => {
//                 canvas
//                     .draw_line(Point::new(x, y), Point::new(x + 6, y))
//                     .unwrap();
//                 canvas
//                     .draw_line(Point::new(x, y), Point::new(x, y + 4))
//                     .unwrap();
//                 canvas
//                     .draw_line(Point::new(x, y + 4), Point::new(x + 6, y + 4))
//                     .unwrap();
//                 canvas
//                     .draw_line(Point::new(x + 6, y + 4), Point::new(x + 6, y + 8))
//                     .unwrap();
//                 canvas
//                     .draw_line(Point::new(x, y + 8), Point::new(x + 6, y + 8))
//                     .unwrap();
//             }
//             ':' => {
//                 canvas.draw_point(Point::new(x + 3, y + 2)).unwrap();
//                 canvas.draw_point(Point::new(x + 3, y + 6)).unwrap();
//             }
//             c if c.is_digit(10) => {
//                 let n = c.to_digit(10).unwrap();
//                 if n != 1 {
//                     canvas
//                         .draw_line(Point::new(x, y), Point::new(x + 6, y))
//                         .unwrap();
//                 }
//                 if n != 1 && n != 7 {
//                     canvas
//                         .draw_line(Point::new(x, y + 8), Point::new(x + 6, y + 8))
//                         .unwrap();
//                 }
//                 if n != 5 && n != 6 {
//                     canvas
//                         .draw_line(Point::new(x + 6, y), Point::new(x + 6, y + 4))
//                         .unwrap();
//                 }
//                 if n != 2 {
//                     canvas
//                         .draw_line(Point::new(x + 6, y + 4), Point::new(x + 6, y + 8))
//                         .unwrap();
//                 }
//                 if n != 1 && n != 2 && n != 3 && n != 7 {
//                     canvas
//                         .draw_line(Point::new(x, y + 4), Point::new(x, y + 8))
//                         .unwrap();
//                 }
//                 if n != 1 && n != 3 && n != 4 && n != 5 && n != 7 && n != 9 {
//                     canvas
//                         .draw_line(Point::new(x, y), Point::new(x, y + 4))
//                         .unwrap();
//                 }
//                 if n != 0 && n != 1 && n != 7 {
//                     canvas
//                         .draw_line(Point::new(x, y + 4), Point::new(x + 6, y + 4))
//                         .unwrap();
//                 }
//             }
//             _ => {}
//         }
//     }
// }

// fn draw_minimap(
//     canvas: &mut Canvas<Window>,
//     player: &Player,
//     map: &[[u8; MAP_WIDTH]; MAP_HEIGHT],
//     screen_width: u32,
//     screen_height: u32,
// ) {
//     let minimap_width = (MAP_WIDTH as u32 * MINIMAP_CELL_SIZE) as i32;
//     let minimap_height = (MAP_HEIGHT as u32 * MINIMAP_CELL_SIZE) as i32;

//     // Position minimap at bottom right
//     let minimap_x = screen_width as i32 - minimap_width - MINIMAP_PADDING;
//     let minimap_y = screen_height as i32 - minimap_height - MINIMAP_PADDING;

//     // Draw map cells
//     for y in 0..MAP_HEIGHT {
//         for x in 0..MAP_WIDTH {
//             let cell_x = minimap_x + (x as i32 * MINIMAP_CELL_SIZE as i32);
//             let cell_y = minimap_y + (y as i32 * MINIMAP_CELL_SIZE as i32);

//             let color = if map[y][x] == 1 {
//                 Color::RGB(128, 128, 128) // Wall color
//             } else {
//                 Color::RGB(32, 32, 32) // Floor color
//             };

//             canvas.set_draw_color(color);
//             canvas
//                 .fill_rect(Rect::new(
//                     cell_x,
//                     cell_y,
//                     MINIMAP_CELL_SIZE,
//                     MINIMAP_CELL_SIZE,
//                 ))
//                 .unwrap();
//         }
//     }

//     // Draw player position
//     let player_size = (MINIMAP_CELL_SIZE as f64 * 0.8) as u32;
//     let player_x = minimap_x + (player.x * MINIMAP_CELL_SIZE as f64) as i32;
//     let player_y = minimap_y + (player.y * MINIMAP_CELL_SIZE as f64) as i32;

//     canvas.set_draw_color(Color::RGB(255, 0, 0));
//     canvas
//         .fill_rect(Rect::new(
//             player_x - (player_size / 2) as i32,
//             player_y - (player_size / 2) as i32,
//             player_size,
//             player_size,
//         ))
//         .unwrap();

//     // Draw player direction line
//     let dir_length = MINIMAP_CELL_SIZE as f64 * 1.5;
//     let dir_end_x = player_x + (player.dir_x * dir_length) as i32;
//     let dir_end_y = player_y + (player.dir_y * dir_length) as i32;

//     canvas.set_draw_color(Color::RGB(255, 255, 0));
//     canvas
//         .draw_line(
//             Point::new(player_x, player_y),
//             Point::new(dir_end_x, dir_end_y),
//         )
//         .unwrap();
// }

// pub fn render(
//     canvas: &mut Canvas<Window>,
//     player: &Player,
//     map: &[[u8; MAP_WIDTH]; MAP_HEIGHT],
//     fps: u32,
//     skybox: &Skybox,
//     gl_ctx: &GLContext,
// ) {
//     let screen_width = 1280;
//     let screen_height = 720;
//     unsafe {
//         render_ground(gl_ctx, (player.x as f32, player.y as f32));
//     }

//     let _ = draw_skybox(canvas, skybox, player, screen_width, screen_height);

//     for x in 0..screen_width {
//         let camera_x = 2.0 * x as f64 / screen_width as f64 - 1.0;
//         let ray_dir_x = player.dir_x + player.plane_x * camera_x;
//         let ray_dir_y = player.dir_y + player.plane_y * camera_x;

//         let mut map_x = player.x.floor() as i32;
//         let mut map_y = player.y.floor() as i32;

//         let delta_dist_x = if ray_dir_x == 0.0 { 1e30 } else { (1.0 / ray_dir_x).abs() };
//         let delta_dist_y = if ray_dir_y == 0.0 { 1e30 } else { (1.0 / ray_dir_y).abs() };

//         let (step_x, mut side_dist_x) = if ray_dir_x < 0.0 {
//             (-1, (player.x - map_x as f64) * delta_dist_x)
//         } else {
//             (1, (map_x as f64 + 1.0 - player.x) * delta_dist_x)
//         };
//         let (step_y, mut side_dist_y) = if ray_dir_y < 0.0 {
//             (-1, (player.y - map_y as f64) * delta_dist_y)
//         } else {
//             (1, (map_y as f64 + 1.0 - player.y) * delta_dist_y)
//         };

//         let mut hit = false;
//         let mut side = 0;

//         while !hit {
//             if side_dist_x < side_dist_y {
//                 side_dist_x += delta_dist_x;
//                 map_x += step_x;
//                 side = 0;
//             } else {
//                 side_dist_y += delta_dist_y;
//                 map_y += step_y;
//                 side = 1;
//             }

//             if map[map_y as usize][map_x as usize] > 0 {
//                 hit = true;
//             }
//         }

//         let perp_wall_dist = if side == 0 {
//             (map_x as f64 - player.x + (1.0 - step_x as f64) / 2.0) / ray_dir_x
//         } else {
//             (map_y as f64 - player.y + (1.0 - step_y as f64) / 2.0) / ray_dir_y
//         };

//         let line_height = ((screen_height as f64 / perp_wall_dist) as i32).min(i32::MAX / 2);
//         let half_line_height = line_height / 2;
//         let half_screen_height = screen_height as i32 / 2;
//         let draw_start = (half_screen_height - half_line_height).max(0);
//         let draw_end = (half_screen_height + half_line_height).min(screen_height as i32 - 1);

//         // Only draw vertical wall stripe (skip floor)
//         let color = if side == 0 {
//             Color::RGB(255, 255, 255)
//         } else {
//             Color::RGB(160, 160, 160)
//         };

//         canvas.set_draw_color(color);
//         let _ = canvas.draw_line(
//             Point::new(x as i32, draw_start),
//             Point::new(x as i32, draw_end),
//         );
//     }

//     // Draw minimap after 3D rendering
//     draw_minimap(
//         canvas,
//         player,
//         map,
//         screen_width as u32,
//         screen_height as u32,
//     );

//     // Draw FPS counter
//     draw_fps(canvas, fps, screen_width as u32);
// }
/////////////////////////////////////////////////////////////////////////////////////////////////

use gl::types::*;
use crate::mesh::Mesh;
use crate::shader::create_program;
use nalgebra::{Matrix4, Perspective3, Point3, Vector3};
use std::ffi::CString;
use std::fs;
use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use rusttype::{Font, Scale, point};
use image::{RgbaImage, Rgba, GenericImageView};

pub struct GLContext {
    pub shader_program: GLuint,
    pub vao: GLuint,
    pub textures: [GLuint; 3],
    pub index_count: usize,
}
pub struct WeaponContext {
    pub shader_program: GLuint,
    pub vao: GLuint,
    pub index_count: usize,
}
pub struct SkyboxContext {
    pub vao: GLuint,
    pub vbo: GLuint,
    pub shader_program: GLuint,
    pub cubemap_texture: GLuint,
}

static SQUARE_SHADER: Lazy<Mutex<Option<GLuint>>> = Lazy::new(|| Mutex::new(None));
static GLYPH_CACHE: Lazy<Mutex<HashMap<char, GLuint>>> = Lazy::new(|| Mutex::new(HashMap::new()));
static FONT_TTF: Lazy<Font<'static>> = Lazy::new(|| {
    let data = fs::read("assets/fonts/PoetsenOne-Regular.ttf").expect("Failed to read font");
    Font::try_from_vec(data).expect("Failed to parse font")
});

pub unsafe fn init_gl_context(mesh: &Mesh) -> GLContext {
    let vert_src = fs::read_to_string("assets/shaders/pbr.vert").unwrap();
    let frag_src = fs::read_to_string("assets/shaders/pbr.frag").unwrap();
    let shader_program = unsafe { create_program(&vert_src, &frag_src) };

    let textures = [0, 0, 0];

    let (mut vao, mut vbo, mut ebo) = (0, 0, 0);
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);

        gl::BindVertexArray(vao);

        let vertex_data: Vec<f32> = mesh
            .vertices.iter()
            .zip(&mesh.normals)
            .zip(&mesh.texcoords)
            .flat_map(|((v, n), t)| vec![v.x, v.y, v.z, n.x, n.y, n.z, t.x, t.y])
            .collect();

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertex_data.len() * 4) as isize,
            vertex_data.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (mesh.indices.len() * 4) as isize,
            mesh.indices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 8 * 4, std::ptr::null());
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 8 * 4, (3 * 4) as *const _);
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, 8 * 4, (6 * 4) as *const _);
        gl::EnableVertexAttribArray(2);

        gl::BindVertexArray(0);
    }

    GLContext {
        shader_program,
        vao,
        textures,
        index_count: mesh.indices.len(),
    }
}

pub unsafe fn init_weapon_context(mesh: &Mesh) -> WeaponContext {
    let vert_src = fs::read_to_string("assets/shaders/pbr.vert").unwrap();
    let frag_src = fs::read_to_string("assets/shaders/pbr.frag").unwrap();
    let shader_program = unsafe { create_program(&vert_src, &frag_src) };

    let (mut vao, mut vbo, mut ebo) = (0, 0, 0);
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);

        gl::BindVertexArray(vao);

        let vertex_data: Vec<f32> = mesh
            .vertices.iter()
            .zip(&mesh.normals)
            .zip(&mesh.texcoords)
            .flat_map(|((v, n), t)| vec![v.x, v.y, v.z, n.x, n.y, n.z, t.x, t.y])
            .collect();

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertex_data.len() * 4) as isize,
            vertex_data.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (mesh.indices.len() * 4) as isize,
            mesh.indices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 8 * 4, std::ptr::null());
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 8 * 4, (3 * 4) as *const _);
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, 8 * 4, (6 * 4) as *const _);
        gl::EnableVertexAttribArray(2);

        gl::BindVertexArray(0);
    }

    WeaponContext {
        shader_program,
        vao,
        index_count: mesh.indices.len(),
    }
}

pub unsafe fn render_ground(ctx: &GLContext, player_pos: (f32, f32)) {
    let projection = Perspective3::new(1280.0 / 720.0, 45.0f32.to_radians(), 0.1, 100.0).to_homogeneous();
    let view = Matrix4::look_at_rh(
        &Point3::new(player_pos.0, 2.0, player_pos.1 + 5.0),
        &Point3::new(player_pos.0, 0.0, player_pos.1),
        &Vector3::y_axis(),
    );
    let model = Matrix4::identity();

    unsafe {
        gl::UseProgram(ctx.shader_program);
        gl::UniformMatrix4fv(gl::GetUniformLocation(ctx.shader_program, CString::new("model").unwrap().as_ptr()), 1, gl::FALSE, model.as_ptr());
        gl::UniformMatrix4fv(gl::GetUniformLocation(ctx.shader_program, CString::new("view").unwrap().as_ptr()), 1, gl::FALSE, view.as_ptr());
        gl::UniformMatrix4fv(gl::GetUniformLocation(ctx.shader_program, CString::new("projection").unwrap().as_ptr()), 1, gl::FALSE, projection.as_ptr());

        gl::BindVertexArray(ctx.vao);
        gl::DrawElements(gl::TRIANGLES, ctx.index_count as i32, gl::UNSIGNED_INT, std::ptr::null());
        gl::BindVertexArray(0);
    }
}

pub unsafe fn render_weapon(ctx: &WeaponContext) {
    let projection = Perspective3::new(1280.0 / 720.0, 45.0f32.to_radians(), 0.01, 100.0).to_homogeneous();
    let view = Matrix4::identity();
    let model = Matrix4::new_translation(&Vector3::new(0.5, -0.6, -1.0)) * Matrix4::new_scaling(0.35);

    unsafe {
        gl::UseProgram(ctx.shader_program);
        gl::UniformMatrix4fv(gl::GetUniformLocation(ctx.shader_program, CString::new("model").unwrap().as_ptr()), 1, gl::FALSE, model.as_ptr());
        gl::UniformMatrix4fv(gl::GetUniformLocation(ctx.shader_program, CString::new("view").unwrap().as_ptr()), 1, gl::FALSE, view.as_ptr());
        gl::UniformMatrix4fv(gl::GetUniformLocation(ctx.shader_program, CString::new("projection").unwrap().as_ptr()), 1, gl::FALSE, projection.as_ptr());

        gl::BindVertexArray(ctx.vao);
        gl::DrawElements(gl::TRIANGLES, ctx.index_count as i32, gl::UNSIGNED_INT, std::ptr::null());
        gl::BindVertexArray(0);
    }
}

// === Skybox ===

const SKYBOX_VERTICES: [f32; 108] = [
    // ... cube vertices for skybox (6 faces x 2 triangles x 3 vertices)
    -1.0,  1.0, -1.0,   -1.0, -1.0, -1.0,    1.0, -1.0, -1.0,
     1.0, -1.0, -1.0,    1.0,  1.0, -1.0,   -1.0,  1.0, -1.0,
    -1.0, -1.0,  1.0,   -1.0, -1.0, -1.0,   -1.0,  1.0, -1.0,
    -1.0,  1.0, -1.0,   -1.0,  1.0,  1.0,   -1.0, -1.0,  1.0,
     1.0, -1.0, -1.0,    1.0, -1.0,  1.0,    1.0,  1.0,  1.0,
     1.0,  1.0,  1.0,    1.0,  1.0, -1.0,    1.0, -1.0, -1.0,
    -1.0, -1.0,  1.0,   -1.0,  1.0,  1.0,    1.0,  1.0,  1.0,
     1.0,  1.0,  1.0,    1.0, -1.0,  1.0,   -1.0, -1.0,  1.0,
    -1.0,  1.0, -1.0,    1.0,  1.0, -1.0,    1.0,  1.0,  1.0,
     1.0,  1.0,  1.0,   -1.0,  1.0,  1.0,   -1.0,  1.0, -1.0,
    -1.0, -1.0, -1.0,   -1.0, -1.0,  1.0,    1.0, -1.0, -1.0,
     1.0, -1.0, -1.0,   -1.0, -1.0,  1.0,    1.0, -1.0,  1.0,
];

unsafe fn load_cubemap(paths: [&str; 6]) -> GLuint {
    let mut texture_id = 0;
    unsafe {
        gl::GenTextures(1, &mut texture_id);
        gl::BindTexture(gl::TEXTURE_CUBE_MAP, texture_id);

        for (i, path) in paths.iter().enumerate() {
            let img = image::open(path).expect("Cubemap texture failed to load").flipv().to_rgb8();
            let (width, height) = img.dimensions();
            let data = img.into_raw();

            gl::TexImage2D(
                gl::TEXTURE_CUBE_MAP_POSITIVE_X + i as u32,
                0,
                gl::RGB as i32,
                width as i32,
                height as i32,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const _,
            );
        }
        gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE as i32);
    }
    texture_id
}

pub unsafe fn init_skybox() -> SkyboxContext {
    let vert_src = fs::read_to_string("assets/shaders/skybox.vert").unwrap();
    let frag_src = fs::read_to_string("assets/shaders/skybox.frag").unwrap();
    let shader_program = unsafe { create_program(&vert_src, &frag_src) };

    let (mut vao, mut vbo) = (0, 0);
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (SKYBOX_VERTICES.len() * 4) as isize,
            SKYBOX_VERTICES.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::BindVertexArray(0);
    }

    let cubemap_texture = unsafe {
        load_cubemap([
            "assets/textures/skybox/right.png",
            "assets/textures/skybox/left.png",
            "assets/textures/skybox/up.png",
            "assets/textures/skybox/down.png",
            "assets/textures/skybox/front.png",
            "assets/textures/skybox/back.png",
        ])
    };

    SkyboxContext { vao, vbo, shader_program, cubemap_texture }
}

pub unsafe fn render_skybox(ctx: &SkyboxContext, projection: Matrix4<f32>, view: Matrix4<f32>) {
    unsafe {
        gl::DepthFunc(gl::LEQUAL);
        gl::UseProgram(ctx.shader_program);

        let view_no_translation = {
            let mut v = view;
            v[(0, 3)] = 0.0;
            v[(1, 3)] = 0.0;
            v[(2, 3)] = 0.0;
            v
        };

        gl::UniformMatrix4fv(gl::GetUniformLocation(ctx.shader_program, CString::new("view").unwrap().as_ptr()), 1, gl::FALSE, view_no_translation.as_ptr());
        gl::UniformMatrix4fv(gl::GetUniformLocation(ctx.shader_program, CString::new("projection").unwrap().as_ptr()), 1, gl::FALSE, projection.as_ptr());

        gl::BindVertexArray(ctx.vao);
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_CUBE_MAP, ctx.cubemap_texture);

        gl::DrawArrays(gl::TRIANGLES, 0, 36);
        gl::BindVertexArray(0);

        gl::DepthFunc(gl::LESS);
    }
}

// === Minimap and FPS rendering using OpenGL ===
pub fn draw_minimap_gl(player: &crate::state::Player, map: &[[u8; crate::state::MAP_WIDTH]; crate::state::MAP_HEIGHT]) {
    ensure_shader_and_glyphcache();
    for y in 0..crate::state::MAP_HEIGHT {
        for x in 0..crate::state::MAP_WIDTH {
            if map[y][x] > 0 {
                draw_square(x as f32 * 0.1, y as f32 * 0.1, [1.0, 1.0, 1.0]);
            }
        }
    }
    draw_square(player.x as f32 * 0.1, player.y as f32 * 0.1, [1.0, 0.0, 0.0]);
}

// === FPS rendering using OpenGL ===
pub fn draw_fps_gl(fps: u32) {
    ensure_shader_and_glyphcache();
    let message = format!("FPS: {}", fps);
    for (i, c) in message.chars().enumerate() {
        draw_char(c, i as f32 * 0.1 - 0.95, -0.9);
    }
}

// === OpenGL rendering ===
fn ensure_shader_and_glyphcache() {
    {
        let mut shader = SQUARE_SHADER.lock().unwrap();
        if shader.is_none() {
            let vert_src = fs::read_to_string("assets/shaders/square.vert").unwrap();
            let frag_src = fs::read_to_string("assets/shaders/square.frag").unwrap();
            *shader = Some(unsafe { create_program(&vert_src, &frag_src) });
        }
    }
    let _glyph = GLYPH_CACHE.lock().unwrap();
}

// Draws a square at the given position with the given color
fn draw_square(x: f32, y: f32, color: [f32; 3]) {
    let shader = *SQUARE_SHADER.lock().unwrap();
    let shader = shader.expect("Square shader not initialized");
    let size = 0.05;
    let square: [f32; 18] = [
        0.0, 0.0, 0.0,
        size, 0.0, 0.0,
        size, size, 0.0,
        size, size, 0.0,
        0.0, size, 0.0,
        0.0, 0.0, 0.0,
    ];
    let mut vao = 0;
    let mut vbo = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER, (square.len() * 4) as isize, square.as_ptr() as *const _, gl::STATIC_DRAW);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::UseProgram(shader);
        let model = Matrix4::new_translation(&Vector3::new(x, y, 0.0));
        let projection = Matrix4::new_orthographic(-1.0, 1.0, -1.0, 1.0, -1.0, 1.0);

        let model_loc = gl::GetUniformLocation(shader, CString::new("model").unwrap().as_ptr());
        let proj_loc = gl::GetUniformLocation(shader, CString::new("projection").unwrap().as_ptr());
        let color_loc = gl::GetUniformLocation(shader, CString::new("color").unwrap().as_ptr());

        gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, model.as_ptr());
        gl::UniformMatrix4fv(proj_loc, 1, gl::FALSE, projection.as_ptr());
        gl::Uniform3f(color_loc, color[0], color[1], color[2]);

        gl::DrawArrays(gl::TRIANGLES, 0, 6);
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteVertexArrays(1, &vao);
    }
}

// Draws a single character at the given position
fn draw_char(c: char, x: f32, y: f32) {
    let mut cache = GLYPH_CACHE.lock().unwrap();
    let _texture = if let Some(tex) = cache.get(&c) {
        *tex
    } else {
        let scale = Scale::uniform(32.0);
        let glyph = FONT_TTF.glyph(c).scaled(scale).positioned(point(0.0, 32.0));
        let bb = glyph.pixel_bounding_box().unwrap_or_default();

        let mut image = RgbaImage::new(bb.width() as u32, bb.height() as u32);
        glyph.draw(|x, y, v| {
            image.put_pixel(x, y, Rgba([255, 255, 255, (v * 255.0) as u8]));
        });

        let mut tex_id = 0;
        unsafe {
            gl::GenTextures(1, &mut tex_id);
            gl::BindTexture(gl::TEXTURE_2D, tex_id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                image.width() as i32,
                image.height() as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                image.as_ptr() as *const _,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        }
        cache.insert(c, tex_id);
        tex_id
    };
    draw_square(x, y, [1.0, 1.0, 1.0]);
}
