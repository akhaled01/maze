use bevy::{prelude::*, render::mesh::{Indices, PrimitiveTopology}};
use bevy_rapier3d::prelude::*;
use bevy::math::primitives::{Plane3d, Cuboid};

pub const MAP_WIDTH: usize = 32;
pub const MAP_HEIGHT: usize = 32;

pub const MAZE: [[u8; MAP_WIDTH]; MAP_HEIGHT] = [
    [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
    [1,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,1,1,1,1,0,1,0,1,1,1,1,1,1,1,0,1,0,1,1,1,1,0,1,1,1,1,1,1,1,0,1],
    [1,0,0,0,0,0,1,0,0,0,0,0,0,0,1,0,1,0,0,0,0,1,0,0,0,0,0,0,0,1,0,1],
    [1,0,1,1,1,1,1,1,1,1,1,1,1,0,1,0,1,1,1,1,0,1,0,1,1,1,1,1,0,1,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,1,0,1,0,0,0,0,0,0,1,0,0,0,0,0,1,0,1,0,1],
    [1,1,1,1,1,1,1,1,1,1,1,0,1,0,1,1,1,1,1,1,1,1,1,1,1,1,0,1,0,1,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,1,0,1,0,1,0,1],
    [1,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,1,0,1,0,1,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,1,0,1,0,1,0,1],
    [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,1,0,1,0,1,0,1,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,1,0,1,0,1,0,1],
    [1,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,1,0,1,0,1,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,1,0,1,0,1],
    [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,1,0,1,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,1,0,1],
    [1,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,1,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,1],
    [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
    [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
];

pub const TILE_SIZE: f32 = 2.0;

pub fn spawn_walls(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh_handle = meshes.add(Cuboid::default());
    let material_handle = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.3, 0.8), // blue-ish wall
        perceptual_roughness: 0.5,
        ..Default::default()
    });

    let cell_size = 2.0;
    let wall_height = 4.0;

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            if MAZE[y][x] == 1 {
                // Adjust center so the maze isn't at (0,0)
                let pos_x = (x as f32 - MAP_HEIGHT as f32 / 2.0) * TILE_SIZE;
                let pos_z = (y as f32 - MAP_WIDTH as f32 / 2.0) * TILE_SIZE;

                commands.spawn((
                    Mesh3d(mesh_handle.clone()),
                    MeshMaterial3d(material_handle.clone()),
                    Transform {
                        translation: Vec3::new(pos_x, TILE_SIZE, pos_z),
                        scale: Vec3::new(cell_size, wall_height, cell_size),
                        ..default()
                    },
                    Collider::cuboid(cell_size / 2.0, wall_height / 2.0, cell_size / 2.0),
                    // Mark as static rigid body
                    RigidBody::Fixed,
                ));
            }
        }
    }
}

// pub fn generate_wall_mesh(level: &[[u8; MAP_WIDTH]; MAP_HEIGHT], wall_size: f32) -> Mesh {
//     let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, bevy::render::render_asset::RenderAssetUsages::RENDER_WORLD);

//     let mut positions = vec![];
//     let mut normals = vec![];
//     let mut uvs = vec![];
//     let mut indices = vec![];

//     let mut idx = 0u32;
//     for y in 0..MAP_HEIGHT {
//         for x in 0..MAP_WIDTH {
//             if level[y][x] == 1 {
//                 // Create a cube or quad for each wall block.
//                 let px = x as f32 * wall_size;
//                 let py = 0.0;
//                 let pz = y as f32 * wall_size;

//                 // ...Insert your quad/cube vertices here...
//                 // For brevity, here is a simple square facing up (for debugging):
//                 positions.push([px, py, pz]);
//                 positions.push([px + wall_size, py, pz]);
//                 positions.push([px + wall_size, py, pz + wall_size]);
//                 positions.push([px, py, pz + wall_size]);
//                 normals.extend([[0.0, 1.0, 0.0]; 4]);
//                 uvs.extend([[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]);
//                 indices.extend([idx, idx + 1, idx + 2, idx, idx + 2, idx + 3]);
//                 idx += 4;
//             }
//         }
//     }

//     mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
//     mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
//     mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
//     mesh.insert_indices(Indices::U32(indices));
//     mesh
// }

// pub fn spawn_batched_walls(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     // ... pass your level data here ...
// ) {
//     let wall_mesh = generate_wall_mesh(&MAZE, 1.0);
//     let mesh_handle = meshes.add(wall_mesh);
//     let material_handle = materials.add(StandardMaterial {
//         base_color: Color::srgb(0.2, 0.3, 0.8), // blue-ish wall
//         perceptual_roughness: 0.5,
//         ..Default::default()
//     });

//     let cell_size = 2.0;
//     let wall_height = 4.0;

//     commands.spawn((
//         Mesh3d(mesh_handle),
//         MeshMaterial3d(material_handle),
//         Transform {
//             translation: Vec3::new(
//                             cell_size - offset,
//                             wall_height / 2.0,
//                             cell_size - offset,
//                         ),
//                         scale: Vec3::new(cell_size, wall_height, cell_size),
//                         ..Default::default()
//         }
//     ));
// }

// pub fn spawn_maze(
//     mut commands: Commands, 
//     mut meshes: ResMut<Assets<Mesh>>, 
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     let cell_size = 2.0;
//     let wall_height = 4.0;
//     let offset = (MAZE.len() as f32 * cell_size) / 2.0 - cell_size / 2.0;
//     for (z, row) in MAZE.iter().enumerate() {
//         for (x, &cell) in row.iter().enumerate() {
//             if cell == 1 {
//                 commands.spawn((
//                     Mesh3d(meshes.add(Mesh::from(Cuboid::default()))),
                    // MeshMaterial3d(materials.add(StandardMaterial {
                    //     base_color: Color::srgb(0.2, 0.2, 0.2),
                    //     ..Default::default()
                    // })),
//                     Transform {
//                         translation: Vec3::new(
//                             x as f32 * cell_size - offset,
//                             wall_height / 2.0,
//                             z as f32 * cell_size - offset,
//                         ),
//                         scale: Vec3::new(cell_size, wall_height, cell_size),
//                         ..Default::default()
//                     },
//                     RigidBody::Fixed,
//                     GlobalTransform::default(),
//                     Visibility::Visible,
//                     InheritedVisibility::default(),
//                 )).insert(Collider::cuboid(cell_size, wall_height, cell_size));
//             }
//         }
//     }
// }

pub fn spawn_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let ground_size = 256.0;
    let mesh = meshes.add(Mesh::from(Plane3d::default()));
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.4, 0.4, 0.4),
        ..Default::default()
    });

    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform {
            translation: Vec3::ZERO,
            scale: Vec3::new(ground_size / 2.0, 1.0, ground_size / 2.0),
            ..Default::default()
        },
        RigidBody::Fixed,
        Collider::cuboid(ground_size / 2.0, 0.5, ground_size / 2.0),
        GlobalTransform::default(),
        Visibility::Visible,
        InheritedVisibility::default(),
    ));
}