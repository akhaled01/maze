
use gl::types::{GLenum, GLuint};
use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::{Canvas, Texture, TextureCreator},
    image::{LoadTexture, InitFlag},
    video::{Window, WindowContext},
};
use crate::{mesh::Mesh, utils};
use nalgebra::{Matrix4, Perspective3, Point3, Vector3};

use crate::state::{MAP_HEIGHT, MAP_WIDTH};

use crate::state::Player;

// Shaders settings
pub struct GLContext {
    pub shader_program: GLuint,
    pub vao: GLuint,
    pub textures: [GLuint; 3],
    pub index_count: usize,
}

pub unsafe fn compile_shader(src: &str, shader_type: GLenum) -> GLuint {
    unsafe {
        let shader = gl::CreateShader(shader_type);
        gl::ShaderSource(shader, 1, &(src.as_ptr() as *const i8), &(src.len() as i32));
        gl::CompileShader(shader);
        shader
    }
}

pub unsafe fn create_program(vert_src: &str, frag_src: &str) -> GLuint {
    unsafe {
        let vert_shader = compile_shader(vert_src, gl::VERTEX_SHADER);
        let frag_shader = compile_shader(frag_src, gl::FRAGMENT_SHADER);

        let program = gl::CreateProgram();
        gl::AttachShader(program, vert_shader);
        gl::AttachShader(program, frag_shader);
        gl::LinkProgram(program);

        gl::DeleteShader(vert_shader);
        gl::DeleteShader(frag_shader);
        program
    }
}

pub unsafe fn init_gl_context(mesh: &Mesh) -> GLContext {
    unsafe {
        // Load and compile shaders
        let vert_src = std::fs::read_to_string("assets/shaders/pbr.vert").expect("Vertex shader error");
        let frag_src = std::fs::read_to_string("assets/shaders/pbr.frag").expect("Fragment shader error");
        let shader_program = create_program(&vert_src, &frag_src);

        // Load textures
        let textures = [
            utils::load_texture("assets/textures/ground/ground_basecolor.png", gl::TEXTURE0),
            utils::load_texture("assets/textures/ground/ground_normal.png", gl::TEXTURE1),
            utils::load_texture("assets/textures/ground/ground_roughness.png", gl::TEXTURE2),
        ];

        // Setup mesh VAO
        let (mut vao, mut vbo, mut ebo) = (0, 0, 0);
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);

        gl::BindVertexArray(vao);

        let vertex_data: Vec<f32> = mesh
            .vertices
            .iter()
            .zip(&mesh.normals)
            .zip(&mesh.texcoords)
            .flat_map(|((v, n), t)| vec![v.x, v.y, v.z, n.x, n.y, n.z, t.x, t.y])
            .collect();

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertex_data.len() * std::mem::size_of::<f32>()) as isize,
            vertex_data.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (mesh.indices.len() * std::mem::size_of::<u32>()) as isize,
            mesh.indices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        let stride = 8 * std::mem::size_of::<f32>() as i32;

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * std::mem::size_of::<f32>()) as *const _);
        gl::EnableVertexAttribArray(1);

        gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, stride, (6 * std::mem::size_of::<f32>()) as *const _);
        gl::EnableVertexAttribArray(2);

        gl::BindVertexArray(0);

        GLContext {
            shader_program,
            vao,
            textures,
            index_count: mesh.indices.len(),
        }
    }
}

pub struct Skybox<'a> {
    pub front: Texture<'a>,
    pub back: Texture<'a>,
    pub left: Texture<'a>,
    pub right: Texture<'a>,
    pub up: Texture<'a>,
    pub down: Texture<'a>,
}

impl<'a> Skybox<'a> {
    pub fn load(texture_creator: &'a TextureCreator<WindowContext>) -> Result<Self, String> {
        Ok(Self {
            front: texture_creator.load_texture("assets/textures/skybox/front.png")?,
            back: texture_creator.load_texture("assets/textures/skybox/back.png")?,
            left: texture_creator.load_texture("assets/textures/skybox/left.png")?,
            right: texture_creator.load_texture("assets/textures/skybox/right.png")?,
            up: texture_creator.load_texture("assets/textures/skybox/up.png")?,
            down: texture_creator.load_texture("assets/textures/skybox/down.png")?,
        })
    }
}

const MINIMAP_CELL_SIZE: u32 = 4; // Size of each cell in the minimap
const MINIMAP_PADDING: i32 = 20; // Padding from the edges of the screen

pub unsafe fn render_ground(gl_ctx: &GLContext, player_pos: (f32, f32)) {
    let projection = Perspective3::new(1280.0 / 720.0, 45.0f32.to_radians(), 0.1, 100.0).to_homogeneous();
    let view = Matrix4::look_at_rh(
        &Point3::new(player_pos.0, 2.0, player_pos.1 + 5.0),
        &Point3::new(player_pos.0, 0.0, player_pos.1),
        &Vector3::y_axis(),
    );
    let model = Matrix4::identity();

    unsafe {
        gl::UseProgram(gl_ctx.shader_program);

        gl::UniformMatrix4fv(gl::GetUniformLocation(gl_ctx.shader_program, "model\0".as_ptr() as _), 1, gl::FALSE, model.as_ptr());
        gl::UniformMatrix4fv(gl::GetUniformLocation(gl_ctx.shader_program, "view\0".as_ptr() as _), 1, gl::FALSE, view.as_ptr());
        gl::UniformMatrix4fv(gl::GetUniformLocation(gl_ctx.shader_program, "projection\0".as_ptr() as _), 1, gl::FALSE, projection.as_ptr());

        for (i, texture) in gl_ctx.textures.iter().enumerate() {
            gl::ActiveTexture(gl::TEXTURE0 + i as u32);
            gl::BindTexture(gl::TEXTURE_2D, *texture);
        }

        gl::BindVertexArray(gl_ctx.vao);
        // gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        gl::DrawElements(gl::TRIANGLES, gl_ctx.index_count as i32, gl::UNSIGNED_INT, std::ptr::null());
        gl::BindVertexArray(0);
    }
}

pub fn draw_skybox(
    canvas: &mut Canvas<Window>,
    skybox: &Skybox,
    player: &Player,
    screen_width: u32,
    screen_height: u32,
) -> Result<(), String> {
    let angle = player.dir_y.atan2(player.dir_x).to_degrees();
    let texture = if angle >= -45.0 && angle < 45.0 {
        &skybox.front
    } else if angle >= 45.0 && angle < 135.0 {
        &skybox.left
    } else if angle >= -135.0 && angle < -45.0 {
        &skybox.right
    } else {
        &skybox.back
    };

    canvas.copy(texture, None, Rect::new(0, 0, screen_width, screen_height / 2))?;
    Ok(())
}

fn draw_fps(canvas: &mut Canvas<Window>, fps: u32, screen_width: u32) {
    // Draw FPS text in the top right corner
    canvas.set_draw_color(Color::RGB(255, 255, 0));
    let fps_text = format!("FPS: {}", fps);
    let text_width = fps_text.len() as i32 * 8; // Approximate width based on character count
    let text_x = screen_width as i32 - text_width - MINIMAP_PADDING;

    // Draw each character manually since we don't have TTF support
    for (i, c) in fps_text.chars().enumerate() {
        let x = text_x + i as i32 * 8;
        let y = MINIMAP_PADDING;

        // Simple pixel representation of characters
        match c {
            'F' => {
                canvas
                    .draw_line(Point::new(x, y), Point::new(x + 6, y))
                    .unwrap();
                canvas
                    .draw_line(Point::new(x, y), Point::new(x, y + 8))
                    .unwrap();
                canvas
                    .draw_line(Point::new(x, y + 4), Point::new(x + 4, y + 4))
                    .unwrap();
            }
            'P' => {
                canvas
                    .draw_line(Point::new(x, y), Point::new(x + 6, y))
                    .unwrap();
                canvas
                    .draw_line(Point::new(x, y), Point::new(x, y + 8))
                    .unwrap();
                canvas
                    .draw_line(Point::new(x, y + 4), Point::new(x + 6, y + 4))
                    .unwrap();
                canvas
                    .draw_line(Point::new(x + 6, y), Point::new(x + 6, y + 4))
                    .unwrap();
            }
            'S' => {
                canvas
                    .draw_line(Point::new(x, y), Point::new(x + 6, y))
                    .unwrap();
                canvas
                    .draw_line(Point::new(x, y), Point::new(x, y + 4))
                    .unwrap();
                canvas
                    .draw_line(Point::new(x, y + 4), Point::new(x + 6, y + 4))
                    .unwrap();
                canvas
                    .draw_line(Point::new(x + 6, y + 4), Point::new(x + 6, y + 8))
                    .unwrap();
                canvas
                    .draw_line(Point::new(x, y + 8), Point::new(x + 6, y + 8))
                    .unwrap();
            }
            ':' => {
                canvas.draw_point(Point::new(x + 3, y + 2)).unwrap();
                canvas.draw_point(Point::new(x + 3, y + 6)).unwrap();
            }
            c if c.is_digit(10) => {
                let n = c.to_digit(10).unwrap();
                if n != 1 {
                    canvas
                        .draw_line(Point::new(x, y), Point::new(x + 6, y))
                        .unwrap();
                }
                if n != 1 && n != 7 {
                    canvas
                        .draw_line(Point::new(x, y + 8), Point::new(x + 6, y + 8))
                        .unwrap();
                }
                if n != 5 && n != 6 {
                    canvas
                        .draw_line(Point::new(x + 6, y), Point::new(x + 6, y + 4))
                        .unwrap();
                }
                if n != 2 {
                    canvas
                        .draw_line(Point::new(x + 6, y + 4), Point::new(x + 6, y + 8))
                        .unwrap();
                }
                if n != 1 && n != 2 && n != 3 && n != 7 {
                    canvas
                        .draw_line(Point::new(x, y + 4), Point::new(x, y + 8))
                        .unwrap();
                }
                if n != 1 && n != 3 && n != 4 && n != 5 && n != 7 && n != 9 {
                    canvas
                        .draw_line(Point::new(x, y), Point::new(x, y + 4))
                        .unwrap();
                }
                if n != 0 && n != 1 && n != 7 {
                    canvas
                        .draw_line(Point::new(x, y + 4), Point::new(x + 6, y + 4))
                        .unwrap();
                }
            }
            _ => {}
        }
    }
}

fn draw_minimap(
    canvas: &mut Canvas<Window>,
    player: &Player,
    map: &[[u8; MAP_WIDTH]; MAP_HEIGHT],
    screen_width: u32,
    screen_height: u32,
) {
    let minimap_width = (MAP_WIDTH as u32 * MINIMAP_CELL_SIZE) as i32;
    let minimap_height = (MAP_HEIGHT as u32 * MINIMAP_CELL_SIZE) as i32;

    // Position minimap at bottom right
    let minimap_x = screen_width as i32 - minimap_width - MINIMAP_PADDING;
    let minimap_y = screen_height as i32 - minimap_height - MINIMAP_PADDING;

    // Draw map cells
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let cell_x = minimap_x + (x as i32 * MINIMAP_CELL_SIZE as i32);
            let cell_y = minimap_y + (y as i32 * MINIMAP_CELL_SIZE as i32);

            let color = if map[y][x] == 1 {
                Color::RGB(128, 128, 128) // Wall color
            } else {
                Color::RGB(32, 32, 32) // Floor color
            };

            canvas.set_draw_color(color);
            canvas
                .fill_rect(Rect::new(
                    cell_x,
                    cell_y,
                    MINIMAP_CELL_SIZE,
                    MINIMAP_CELL_SIZE,
                ))
                .unwrap();
        }
    }

    // Draw player position
    let player_size = (MINIMAP_CELL_SIZE as f64 * 0.8) as u32;
    let player_x = minimap_x + (player.x * MINIMAP_CELL_SIZE as f64) as i32;
    let player_y = minimap_y + (player.y * MINIMAP_CELL_SIZE as f64) as i32;

    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas
        .fill_rect(Rect::new(
            player_x - (player_size / 2) as i32,
            player_y - (player_size / 2) as i32,
            player_size,
            player_size,
        ))
        .unwrap();

    // Draw player direction line
    let dir_length = MINIMAP_CELL_SIZE as f64 * 1.5;
    let dir_end_x = player_x + (player.dir_x * dir_length) as i32;
    let dir_end_y = player_y + (player.dir_y * dir_length) as i32;

    canvas.set_draw_color(Color::RGB(255, 255, 0));
    canvas
        .draw_line(
            Point::new(player_x, player_y),
            Point::new(dir_end_x, dir_end_y),
        )
        .unwrap();
}

pub fn render(
    canvas: &mut Canvas<Window>,
    player: &Player,
    map: &[[u8; MAP_WIDTH]; MAP_HEIGHT],
    fps: u32,
    skybox: &Skybox,
    gl_ctx: &GLContext,
) {
    let screen_width = 1280;
    let screen_height = 720;
    unsafe {
        render_ground(gl_ctx, (player.x as f32, player.y as f32));
    }

    let _ = draw_skybox(canvas, skybox, player, screen_width, screen_height);

    // for x in 0..screen_width {
    //     let camera_x = 2.0 * x as f64 / screen_width as f64 - 1.0;

    //     let ray_dir_x = player.dir_x + player.plane_x * camera_x;
    //     let ray_dir_y = player.dir_y + player.plane_y * camera_x;

    //     // Map square the ray is in
    //     let mut map_x = player.x.floor() as i32;
    //     let mut map_y = player.y.floor() as i32;

    //     // Distance to next x or y side
    //     let delta_dist_x = if ray_dir_x == 0.0 {
    //         1e30
    //     } else {
    //         (1.0 / ray_dir_x).abs()
    //     };
    //     let delta_dist_y = if ray_dir_y == 0.0 {
    //         1e30
    //     } else {
    //         (1.0 / ray_dir_y).abs()
    //     };

    //     // Step direction and initial side distances
    //     let (step_x, mut side_dist_x) = if ray_dir_x < 0.0 {
    //         (-1, (player.x - map_x as f64) * delta_dist_x)
    //     } else {
    //         (1, (map_x as f64 + 1.0 - player.x) * delta_dist_x)
    //     };
    //     let (step_y, mut side_dist_y) = if ray_dir_y < 0.0 {
    //         (-1, (player.y - map_y as f64) * delta_dist_y)
    //     } else {
    //         (1, (map_y as f64 + 1.0 - player.y) * delta_dist_y)
    //     };

    //     let mut hit = false;
    //     let mut side = 0;

    //     // Perform DDA
    //     while !hit {
    //         if side_dist_x < side_dist_y {
    //             side_dist_x += delta_dist_x;
    //             map_x += step_x;
    //             side = 0;
    //         } else {
    //             side_dist_y += delta_dist_y;
    //             map_y += step_y;
    //             side = 1;
    //         }

    //         if map[map_y as usize][map_x as usize] > 0 {
    //             hit = true;
    //         }
    //     }

    //     // Calculate distance
    //     let perp_wall_dist = if side == 0 {
    //         (map_x as f64 - player.x + (1.0 - step_x as f64) / 2.0) / ray_dir_x
    //     } else {
    //         (map_y as f64 - player.y + (1.0 - step_y as f64) / 2.0) / ray_dir_y
    //     };

    //     // Clamp line_height to prevent overflow
    //     let line_height = ((screen_height as f64 / perp_wall_dist) as i32).min(i32::MAX / 2);
    //     let half_line_height = line_height / 2;
    //     let half_screen_height = screen_height as i32 / 2;
    //     let draw_start = (half_screen_height - half_line_height).max(0);
    //     let draw_end = (half_screen_height + half_line_height).min(screen_height as i32 - 1);

    //     // Draw vertical line
    //     let color = if side == 0 {
    //         Color::RGB(255, 255, 255)
    //     } else {
    //         Color::RGB(160, 160, 160)
    //     };

    //     canvas.set_draw_color(color);
    //     let _ = canvas.draw_line(
    //         Point::new(x as i32, draw_start),
    //         Point::new(x as i32, draw_end),
    //     );
    // }
    for x in 0..screen_width {
        let camera_x = 2.0 * x as f64 / screen_width as f64 - 1.0;
        let ray_dir_x = player.dir_x + player.plane_x * camera_x;
        let ray_dir_y = player.dir_y + player.plane_y * camera_x;

        let mut map_x = player.x.floor() as i32;
        let mut map_y = player.y.floor() as i32;

        let delta_dist_x = if ray_dir_x == 0.0 { 1e30 } else { (1.0 / ray_dir_x).abs() };
        let delta_dist_y = if ray_dir_y == 0.0 { 1e30 } else { (1.0 / ray_dir_y).abs() };

        let (step_x, mut side_dist_x) = if ray_dir_x < 0.0 {
            (-1, (player.x - map_x as f64) * delta_dist_x)
        } else {
            (1, (map_x as f64 + 1.0 - player.x) * delta_dist_x)
        };
        let (step_y, mut side_dist_y) = if ray_dir_y < 0.0 {
            (-1, (player.y - map_y as f64) * delta_dist_y)
        } else {
            (1, (map_y as f64 + 1.0 - player.y) * delta_dist_y)
        };

        let mut hit = false;
        let mut side = 0;

        while !hit {
            if side_dist_x < side_dist_y {
                side_dist_x += delta_dist_x;
                map_x += step_x;
                side = 0;
            } else {
                side_dist_y += delta_dist_y;
                map_y += step_y;
                side = 1;
            }

            if map[map_y as usize][map_x as usize] > 0 {
                hit = true;
            }
        }

        let perp_wall_dist = if side == 0 {
            (map_x as f64 - player.x + (1.0 - step_x as f64) / 2.0) / ray_dir_x
        } else {
            (map_y as f64 - player.y + (1.0 - step_y as f64) / 2.0) / ray_dir_y
        };

        let line_height = ((screen_height as f64 / perp_wall_dist) as i32).min(i32::MAX / 2);
        let half_line_height = line_height / 2;
        let half_screen_height = screen_height as i32 / 2;
        let draw_start = (half_screen_height - half_line_height).max(0);
        let draw_end = (half_screen_height + half_line_height).min(screen_height as i32 - 1);

        // Only draw vertical wall stripe (skip floor)
        let color = if side == 0 {
            Color::RGB(255, 255, 255)
        } else {
            Color::RGB(160, 160, 160)
        };

        canvas.set_draw_color(color);
        let _ = canvas.draw_line(
            Point::new(x as i32, draw_start),
            Point::new(x as i32, draw_end),
        );
    }

    // Draw minimap after 3D rendering
    draw_minimap(
        canvas,
        player,
        map,
        screen_width as u32,
        screen_height as u32,
    );

    // Draw FPS counter
    draw_fps(canvas, fps, screen_width as u32);
}
