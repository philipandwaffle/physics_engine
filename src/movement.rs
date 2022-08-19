use bevy::{prelude::*, math::vec3};

use crate::GravitationalConstant;

#[derive(Component)]
pub struct RigidBody{
    pub mass: Option<f32>,
    pub acc: Vec3,
    pub vel: Vec3,
    pub move_enabled: bool,
    pub gravity_enabled: bool,
}
impl RigidBody{
    pub fn apply_acc(&mut self, delta_time: &f32){
        self.vel += self.acc * *delta_time;
    }

    pub fn apply_force(&mut self, f: Vec3){
        if self.mass.is_some(){
            self.vel += f / self.mass.unwrap();
        }
    }
}

pub fn move_entities(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut RigidBody)>
){
    let delta_time = time.delta_seconds();
    for (mut t, mut r) in query.iter_mut(){
        if r.move_enabled{
            r.apply_acc(&delta_time.clone());
            t.translation += delta_time * r.vel;
        }
    }
}

pub fn apply_gravity(
    g_const: Res<GravitationalConstant>,
    mut query: Query<(Entity, &mut Transform, &mut RigidBody)>
){
    let mut entities: Vec<(Entity, f32, Vec3, Vec3)> = vec![];
    for (e, t, r) in query.iter(){
        if r.mass.is_some() && r.gravity_enabled{
            entities.push((e, r.mass.unwrap(), t.translation, vec3(0., 0., 0.)));
        }
    }
    for i in 0..entities.len(){
        let mut total_force = vec3(0., 0., 0.);
        let m1 = entities.get(i).unwrap().1;
        for j in 0..entities.len(){            
            if i != j {
                let m2 = entities.get(j).unwrap().1;

                let p1 = entities.get(j).unwrap().2;
                let p2 = entities.get(i).unwrap().2;

                let r = p1.distance(p2);
                if r < 0.1 {
                    continue;
                }

                let force = g_const.g * ((m1 * m2) / r.powi(2));
                
                let force = (p1 - p2).normalize() * force;
                total_force += force;
            }
        }
        query.get_mut(entities.get(i).unwrap().0).unwrap().2.apply_force(total_force)
    }
}