pub mod movable_system{
    use std::ops::{Mul, Add};

    use bevy::{prelude::{Query, Transform, Res}};

    use crate::TimeStep;
    use crate::components::acceleration::Acc2;
    use crate::components::system_identity_components::Movable;

    pub fn move_entities(
        time_step: Res<TimeStep>,
        mut query: Query<(&mut Transform, &Movable, &mut Acc2)>
    ){        
        for (mut transform, _movable, mut acc) in query.iter_mut(){
            //let delta_time = time_step.time.delta_seconds();
            acc.vel = acc.vel.add(acc.acc.mul(time_step.time_step));
            transform.translation.x += acc.vel.x;
            transform.translation.y += acc.vel.y;

            println!("{}",acc.vel);
        }
    }
}

pub mod drag_system{
    use bevy::{prelude::{Query, Res}, math::vec2};

    use crate::{components::{acceleration::Acc2, drag::Drag2, mass::Mass}, TimeStep};
    use crate::components::system_identity_components::ExperienceDrag;

    pub fn apply_drag(
        time_step: Res<TimeStep>,
        mut query: Query<(&mut Acc2, &Drag2, &Mass, &ExperienceDrag)>
    ){
        for (mut acc, drag, mass, _exp_drag) in query.iter_mut(){
            if acc.vel != vec2(0., 0.){
                let drag_force = drag.drag_coefficient * ((drag.fluid_density * (drag.fluid_vel - acc.vel)) / 2.);
                let drag_vel = drag_force / mass.mass;

                println!("{}",drag_vel);

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
}

pub mod gravity_system{
    use std::vec;

    use bevy::{prelude::{Query, Res, Entity, Transform}, math::{vec2, Vec2, Vec3, vec3}};
    use crate::{components::{mass::Mass, acceleration::Acc2, system_identity_components::HasGravity}, TimeStep, GravitationalConstant};

    pub fn gravity(
        time_step: Res<TimeStep>,
        g_const: Res<GravitationalConstant>,
        mut query: Query<(Entity, &Transform, &mut Acc2, &Mass, &HasGravity)>
    ){
        let mut entities: Vec<(Entity, f32, Vec3, Vec2)> = vec![];       
        
        for (entity, transform, _acc, mass, _gravity) in query.iter(){
            entities.push((entity, mass.mass, transform.translation, vec2(0., 0.)));
        }

        for i in 0..entities.len() - 1{
            let mut total_force = vec3(0., 0., 0.);
            let m1 = entities.get(i).unwrap().1;
            for j in 0..entities.len() - 1{
                if i != j {
                    let m2 = entities.get(j).unwrap().1;
                    let dir = entities.get(j).unwrap().2 - entities.get(i).unwrap().2;
                    let r = dir.length();
                    let force = g_const.g * ((m1 * m2) / r.powi(2));
                    total_force += force * dir.normalize();
                }
            }
            let foo = query.get_mut(entities.get(i).unwrap().0).unwrap().2.acc;
        }

        let f = entities.get(0).unwrap().0;
        query.get(entities.get(0).unwrap().0);
    }
}