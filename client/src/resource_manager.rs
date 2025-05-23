use std::collections::HashMap;
use std::path::Path;
use image::{DynamicImage, ImageReader as ImageReader};
use anyhow::{Result, Context};
use obj::{Obj, ObjData};

pub struct ResourceManager {
    textures: HashMap<String, DynamicImage>,
    models: HashMap<String, ObjData>, // Store loaded OBJ data keyed by string
    // TODO: Add similar maps for audio, fonts, shaders, etc.
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            models: HashMap::new(),
        }
    }

    /// Load a texture from the assets/textures directory, store it by string key.
    pub fn load_texture(&mut self, id: &str, rel_path: &str) -> Result<()> {
        let path = Path::new("assets/textures").join(rel_path);
        let img = ImageReader::open(&path)
            .with_context(|| format!("Failed to open texture file: {:?}", path))?
            .decode()
            .with_context(|| format!("Failed to decode texture: {:?}", path))?;

        self.textures.insert(id.to_string(), img);
        Ok(())
    }

    /// Get a reference to a loaded texture (if available)
    pub fn get_texture(&self, id: &str) -> Option<&DynamicImage> {
        self.textures.get(id)
    }

    /// Example: Load all textures needed by your game
    pub fn load_all_textures(&mut self) -> Result<()> {
        self.load_texture("player", "player.png")?;
        self.load_texture("enemy", "enemy.png")?;
        // ...and so on for every needed texture
        Ok(())
    }

    // TODO: Implement similar methods for models, audio, fonts, and shaders as you convert their users.
    pub fn load_model(&mut self, id: &str, rel_path: &str) -> Result<()> {
        let path = Path::new("assets/models").join(rel_path);
        let obj_data: ObjData = Obj::load(&path)
            .with_context(|| format!("Failed to load OBJ file: {:?}", path))?
            .data;
        self.models.insert(id.to_string(), obj_data);
        Ok(())
    }

    pub fn get_model(&self, id: &str) -> Option<&ModelType> {
        self.models.get(id)
    }

    // --- Example: Bulk loading of all needed models and textures ---
    pub fn load_all_assets(&mut self) -> Result<()> {
        // Textures
        // self.load_texture("ground_tex", "ground.png")?;
        // self.load_texture("wall_tex", "wall.png")?;

        // Models
        self.load_model("ground", "/ground/ground.obj")?;
        self.load_model("wall", "/wall/wall.obj")?;

        // ...add more as needed...
        Ok(())
    }
}
