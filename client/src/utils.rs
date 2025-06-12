use bevy::math::Vec3;

pub fn blender_to_world(blender : Vec3) -> Vec3{
    return Vec3::new(blender.x,blender.z,-blender.y);
}