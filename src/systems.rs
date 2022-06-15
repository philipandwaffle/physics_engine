use std::ops::{Mul, Add};

use bevy::{prelude::*, math::vec3};

use crate::{TimeStep, components::{*, system_identity_components::*}, GravitationalConstant};



pub fn move_entities(
    time_step: Res<TimeStep>,
    mut query: Query<(&mut Transform, &Movable, &mut Acc)>
){        
    for (mut transform, _movable, mut acc) in query.iter_mut(){
        //let delta_time = time_step.time.delta_seconds();
        acc.vel = acc.vel.add(acc.acc.mul(time_step.time_step));
        transform.translation.x += acc.vel.x;
        transform.translation.y += acc.vel.y;
        println!("vel added: {}",acc.acc);
    }
}



pub fn apply_drag(
    time_step: Res<TimeStep>,
    mut query: Query<(&mut Acc, &Drag, &Mass, &ExperienceDrag)>
){
    for (mut acc, drag, mass, _exp_drag) in query.iter_mut(){
        if acc.vel != vec3(0., 0., 0.){
            let drag_force = drag.drag_coefficient as f32 * ((drag.fluid_density as f32 * (drag.fluid_vel - acc.vel)) / 2.);
            let drag_vel = drag_force / mass.mass as f32;            

            if acc.vel.x.abs() < drag_vel.x.abs(){
                acc.vel.x = 0.;
            }else if acc.vel.y.abs() < -drag_vel.y.abs(){
                acc.vel.y = 0.;
            }else{
                acc.vel += drag_vel * time_step.time_step;   
            }                
        }
    }
}


pub fn gravity(
    time_step: Res<TimeStep>,
    g_const: Res<GravitationalConstant>,
    mut query: Query<(Entity, &Transform, &mut Acc, &Mass, &HasGravity)>
){
    let mut entities: Vec<(Entity, f64, Vec3, Vec3)> = vec![];       
    println!("calc for: {}", query.iter().len());
    for (entity, transform, _acc, mass, _gravity) in query.iter(){
        entities.push((entity, mass.mass, transform.translation, vec3(0., 0., 0.)));
    }
    println!("calc for: {}", entities.len());

    for i in 0..entities.len(){
        let mut total_force = vec3(0., 0., 0.);
        let m1 = entities.get(i).unwrap().1;
        for j in 0..entities.len(){
            println!("calc for: {} -> {}", i , j);
            if i != j {
                let m2 = entities.get(j).unwrap().1;
                let dir = entities.get(j).unwrap().2 - entities.get(i).unwrap().2;
                let r = dir.length() as f64;
                let force = g_const.g * ((m1 * m2) / r.powi(2)) as f64;
                
                println!("single Force: {}", &force);
                total_force += dir.normalize() * force as f32;
            }
        }        
        let acc = total_force / entities.get(i).unwrap().1 as f32;
        println!("Force: {}", &total_force);
        println!("gravity acc: {}", &acc);
        query.get_mut(entities.get(i).unwrap().0).unwrap().2.acc += acc;        
    }
}
