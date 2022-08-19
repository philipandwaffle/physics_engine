use bevy::{prelude::*, math::{vec3, vec2}};

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

#[derive(Component)]
pub struct CircleCollision{
    pub radius: f32
}

pub fn collision_sys(
    mut query: Query<(Entity, &mut RigidBody, &Transform, &CircleCollision)>
){
    // entity, translation, mass, radius, current vel, total vel to add
    let mut entities: Vec<(Entity, Vec3, f32, f32, Vec3, Vec3)> = vec![];
    for (e, rb, t, cc) in query.iter_mut(){
        if rb.mass.is_some(){
            entities.push((e, t.translation, rb.mass.unwrap(), cc.radius, rb.vel, vec3(0., 0., 0.)));
        }
    }

    for i in 0..entities.len(){
        for j in 0..entities.len(){
            if has_collided(entities[i].1, entities[j].1, entities[i].3, entities[j].3) && i != j{                
                let final_vels = calc_final_vel(entities[i].2, entities[j].2, entities[i].4, entities[j].4);
                entities[i].5 += final_vels.0;
                entities[j].5 += final_vels.1;
            }
        }   
    }

    for (e, _t, _m, _r, _cv, tv) in entities{
        query.get_mut(e).unwrap().1.apply_vel(tv);
    }
}

pub fn has_collided(c1: Vec3, c2: Vec3, r1: f32, r2: f32) -> bool{
    //println!("c1: {}, c2: {}, r1: {}, r2: {}", c1, c2, r1, r2);
    return c1.distance(c2) < r1 + r2;
}

fn calc_final_vel(m1: f32, m2: f32, v1i: Vec3, v2i: Vec3) -> (Vec3, Vec3){
    let v1f = (v1i * ((m1 - m2) / (m1 + m2))) + (v2i * ((2. * m2) / (m1 + m2)));
    let v2f = (v1i * ((2. * m2) / (m1 + m2))) + (v2i * ((m2 - m1) / (m1 + m2)));
    return (v1f, v2f);
}