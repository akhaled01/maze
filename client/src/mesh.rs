// use tobj;
// use nalgebra::{Vector3, Vector2};

// pub struct Mesh {
//     pub vertices: Vec<Vector3<f32>>,
//     pub normals: Vec<Vector3<f32>>,
//     pub texcoords: Vec<Vector2<f32>>,
//     pub indices: Vec<u32>,
// }

// pub fn load_obj(path: &str) -> Mesh {
//     let (models, _) = tobj::load_obj(path, &tobj::GPU_LOAD_OPTIONS).expect("Failed to load OBJ");

//     let mesh = &models[0].mesh;
//     Mesh {
//         vertices: mesh.positions.chunks(3).map(|v| Vector3::new(v[0], v[1], v[2])).collect(),
//         normals: mesh.normals.chunks(3).map(|n| Vector3::new(n[0], n[1], n[2])).collect(),
//         texcoords: mesh.texcoords.chunks(2).map(|t| Vector2::new(t[0], t[1])).collect(),
//         indices: mesh.indices.clone(),
//     }
// }

use tobj;
use nalgebra::{Vector3, Vector2};

pub struct Mesh {
    pub vertices: Vec<Vector3<f32>>,
    pub normals: Vec<Vector3<f32>>,
    pub texcoords: Vec<Vector2<f32>>,
    pub indices: Vec<u32>,
}

pub fn load_obj(path: &str) -> Mesh {
    let (models, _) = tobj::load_obj(path, &tobj::GPU_LOAD_OPTIONS).expect("Failed to load OBJ");

    let mesh = &models[0].mesh;
    Mesh {
        vertices: mesh.positions.chunks(3).map(|v| Vector3::new(v[0], v[1], v[2])).collect(),
        normals: mesh.normals.chunks(3).map(|n| Vector3::new(n[0], n[1], n[2])).collect(),
        texcoords: mesh.texcoords.chunks(2).map(|t| Vector2::new(t[0], t[1])).collect(),
        indices: mesh.indices.clone(),
    }
}
