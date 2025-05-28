use bevy::prelude::*;
use bevy::math::primitives::Capsule3d;

use crate::level::{MAZE, MAP_WIDTH, MAP_HEIGHT};

// Find the first empty cell in a given corner search box
fn find_spawn_in_corner(x0: usize, z0: usize, dx: isize, dz: isize) -> (f32, f32) {
    let search = 4; // Search a 4x4 area at each corner
    for dz_i in 0..search {
        for dx_i in 0..search {
            let x = (x0 as isize + dx_i as isize * dx) as usize;
            let z = (z0 as isize + dz_i as isize * dz) as usize;
            if x < MAP_WIDTH && z < MAP_HEIGHT && MAZE[z][x] == 0 {
                // Place in center of cell, not edge
                return (x as f32, z as f32);
            }
        }
    }
    // Default if not found (shouldn't happen)
    (x0 as f32, z0 as f32)
}

pub fn spawn_players(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cell_size = 2.0;

    let corners = [
        (0, 0, 1, 1),                             // top-left
        (MAP_WIDTH - 1, 0, -1, 1),                // top-right
        (0, MAP_HEIGHT - 1, 1, -1),               // bottom-left
        (MAP_WIDTH - 1, MAP_HEIGHT - 1, -1, -1),  // bottom-right
    ];

    for (i, &(x0, z0, dx, dz)) in corners.iter().enumerate() {
        let (x, z) = find_spawn_in_corner(x0, z0, dx, dz);
        let mesh = meshes.add(Mesh::from(Capsule3d::default()));
        let material = materials.add(Color::srgb(0.2 + 0.2 * i as f32, 0.5, 1.0 - 0.2 * i as f32));
        commands.spawn((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_xyz(
                x * cell_size - (MAP_WIDTH as f32 * cell_size) / 2.0 + cell_size / 2.0,
                1.0,
                z * cell_size - (MAP_HEIGHT as f32 * cell_size) / 2.0 + cell_size / 2.0,
            ),
            GlobalTransform::default(),
            Visibility::Visible,
            InheritedVisibility::default(),
        ));
    }
}