use sdl2::video::Window;
use crate::resource_manager::ResourceManager;
use crate::level::{generate_tile_instances, TileKind, MAP_WIDTH, MAP_HEIGHT};
use obj::ObjData;
use std::collections::HashMap;
use std::ffi::c_void;
use std::ptr;

use crate::resource_manager::{self, ResourceManager};

pub struct GLMesh {
    pub vao: u32,
    pub vbo: u32,
    pub ebo: u32,
    pub index_count: i32,
}

pub struct Renderer {
    screen_width: u32,
    screen_height: u32,
    meshes: HashMap<String, GLMesh>, // keyed by "ground" and "wall"
    res: ResourceManager,
}

impl Renderer {
    pub fn new() -> Self {
        let mut res = ResourceManager::new();
        res.load_model("wall", "/wall/wall.obj")?;
        res.load_model("ground", "/ground/ground.obj")?;
        Self::start(&res);
        Self {
            screen_width: 1280,
            screen_height: 720,
            meshes: HashMap::new(),
            res,
        }
    }

    pub fn start(res: &ResourceManager) {
        self.prepare_level_meshes(res);
    }

    pub fn render(&self, window: &Window) {
        // Level rendering (expand to draw more as you add entities)
        self.render_level();
        window.gl_swap_window();
    }

    pub fn get_window_width(&self) -> u32 {
        self.screen_width
    }

    pub fn get_window_height(&self) -> u32 {
        self.screen_height
    }

    pub fn render_level(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // For each tile, draw the correct mesh at (x, y, z)
        for tile in generate_tile_instances() {
            let mesh_id = match tile.kind {
                TileKind::Ground => "ground",
                TileKind::Wall => "wall",
            };
            if let Some(mesh) = self.meshes.get(mesh_id) {
                // Set up model matrix for tile position
                let model = glm::translate(
                    &glm::Mat4::identity(),
                    &glm::vec3(tile.x as f32, tile.y as f32, 0.0),
                );
                // TODO: Set shader uniforms (model matrix etc.) here

                unsafe {
                    gl::BindVertexArray(mesh.vao);
                    gl::DrawElements(gl::TRIANGLES, mesh.index_count, gl::UNSIGNED_INT, ptr::null());
                    gl::BindVertexArray(0);
                }
            }
        }
    }

    /// Converts ObjData into OpenGL buffers (simple position-only for brevity)
    pub fn upload_obj_as_mesh(&mut self, id: &str, obj: &ObjData) {
        let mut vao = 0;
        let mut vbo = 0;
        let mut ebo = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);
            gl::BindVertexArray(vao);

            // Flatten vertex positions for VBO
            let vertices: Vec<f32> = obj.position.iter().flat_map(|v| [v[0], v[1], v[2]]).collect();

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as isize,
                vertices.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );

            // Indices (flatten to u32)
            let indices: Vec<u32> = obj.objects
                .iter()
                .flat_map(|o| o.groups.iter())
                .flat_map(|g| g.polys.iter())
                .flat_map(|poly| poly.0.iter().map(|idx| idx.0 as u32))
                .collect();

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as isize,
                indices.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );

            // Position attribute (location 0)
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                3 * std::mem::size_of::<f32>() as i32,
                ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            gl::BindVertexArray(0);
        }

        let mesh = GLMesh {
            vao,
            vbo,
            ebo,
            index_count: indices.len() as i32,
        };
        self.meshes.insert(id.to_string(), mesh);
    }

    pub fn prepare_level_meshes(&mut self, res: &ResourceManager) {
        // Only upload once!
        if self.meshes.contains_key("ground") && self.meshes.contains_key("wall") {
            return;
        }
        if let Some(ground_obj) = res.get_model("ground") {
            self.upload_obj_as_mesh("ground", ground_obj);
        }
        if let Some(wall_obj) = res.get_model("wall") {
            self.upload_obj_as_mesh("wall", wall_obj);
        }
    }
}
