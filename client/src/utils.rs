use gl::types::*;
use std::path::Path;
use image::GenericImageView;

pub fn load_image_bytes(path: &str) -> Result<(u32, u32, Vec<u8>), String> {
    let img = image::open(Path::new(path)).map_err(|e| format!("Image loading failed: {}", e))?.flipv().to_rgb8();
    let (width, height) = img.dimensions();
    let pixels = img.into_raw();
    Ok((width, height, pixels))
}

pub unsafe fn load_texture(path: &str, unit: GLenum) -> GLuint {
    let img = image::open(path).expect("Failed to load texture");
    let data = img.flipv().to_rgba8();
    let (width, height) = img.dimensions();

    let mut texture: GLuint = 0;
    
    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::ActiveTexture(unit);
        gl::BindTexture(gl::TEXTURE_2D, texture);

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as GLint,
            width as GLint,
            height as GLint,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            data.as_ptr() as *const GLvoid,
        );

        gl::GenerateMipmap(gl::TEXTURE_2D);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
    }
    texture
}
