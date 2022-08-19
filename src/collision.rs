use bevy::{prelude::*, math::{vec3, vec2}, transform};

use crate::movement::RigidBody;

#[derive(Component)]
pub struct SquareCollision{
    pub size: f32

}
impl SquareCollision{
    // Calculates the 4 vertices in clockwise order starting with the top left vertex
    pub fn get_vertices(&self, centre: Vec2) -> (Vec2, Vec2, Vec2, Vec2){
        return (
            centre.clone() + vec2(-self.size, self.size),
            centre.clone() + vec2(self.size, self.size),
            centre.clone() + vec2(-self.size, -self.size),
            centre.clone() + vec2(self.size, -self.size)
        );
    }
}

pub fn collision_sys(
    query: Query<(&mut RigidBody, &Transform, &SquareCollision)>
){

}

// pub fn has_collided(
//     v1: (Vec2, Vec2, Vec2, Vec2),
//     v2: (Vec2, Vec2, Vec2, Vec2)
// ) -> bool {
//     if v1.1.x < v1
// }